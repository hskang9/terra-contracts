use std::cmp::min;

use cosmwasm_std::{
    log, to_binary, to_vec, Api, BankMsg, Binary, CanonicalAddr, Coin, CosmosMsg, Empty, Env,
    Extern, HandleResponse, HumanAddr, InitResponse, Querier, QueryRequest, StdError, StdResult,
    Storage, Uint128, WasmMsg,
};

use crate::msg::{
    ConfigResponse, ERC20HandleMsg, HandleMsg, InitMsg, PairResponse, QueryMsg, ReserveResponse,
};
use crate::state::{config, config_get, pair_get, pair_set, reserve_get, reserve_set, Config};

/// Contract instantiation tx
/// tx inputs are specified in InitMsg in msg.rs file
pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = Config {
        minimum_luna: msg.minimum_luna,
        owner: deps.api.canonical_address(&msg.owner)?,
    };

    config(&mut deps.storage).save(&state)?;

    Ok(InitResponse::default())
}

/// General handler for contract tx input
/// tx inputs are defined HandleMsg enum in msg.rs file
pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse<Empty>> {
    match msg {
        HandleMsg::AddLiquidity {
            luna_amount,
            token_amount,
            token_address,
            channel_id,
        } => try_add_liquidity(
            deps,
            env,
            &luna_amount,
            &token_amount,
            &token_address,
            &channel_id,
        ),
        HandleMsg::SwapTokenToLuna {
            amount,
            channel_id,
            recipient,
        } => try_swap_to_luna(deps, env, &amount, &channel_id, &recipient),
        HandleMsg::SwapLunaToToken {
            amount,
            channel_id,
            recipient,
        } => try_swap_to_token(deps, env, &amount, &channel_id, &recipient),
        HandleMsg::RemoveLiquidity {
            channel_id
        } => try_remove_liquidity(deps, env, &channel_id),
        HandleMsg::SwapToTokenOutput {
            tokens_bought,
            max_luna,
            deadline,
            channel_id,
            recipient
        } => try_swap_to_token_output(deps, env, &tokens_bought, &max_luna, &deadline, &channel_id, &recipient),
        HandleMsg::SwapToLunaOutput {
            luna_bought,
            max_tokens,
            deadline,
            channel_id,
            recipient
        } => try_swap_to_luna_output(deps, env, &luna_bought, &max_tokens, &deadline, &channel_id, &recipient)
    }
}

/// Deposits LUNA and token to this contract address for registration
/// deps: dependancies for cosmos SDK:  Storage S, Api A, Querier Q
/// env: environment of the tx input
/// luna_amount: Amount of Luna to deposit to register token in Liquidity provider contract(a.k.a. uniswap)
/// token_amount: Amount of token to deposit to register token in Liquidity provider contract(a.k.a. uniswap)
/// channel_id: Identifier for token following BIP standard
/// returns
/// StdResult<HandleResponse>: Result() of response message to send to cosmos SDK
fn try_add_liquidity<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    luna_amount: &Uint128,
    token_amount: &Uint128,
    token_address: &HumanAddr,
    channel_id: &Uint128,
) -> StdResult<HandleResponse> {
    // Human address for sender
    let sender_h = deps.api.human_address(&env.message.sender)?;
    // Human address for token contract
    let contract_h = deps.api.human_address(&env.contract.address)?;
    let token_canonical = deps.api.canonical_address(token_address)?;

    // Check luna_amount > minimumLuna
    let config = config_get(&deps.storage)?;
    if *luna_amount <= config.minimum_luna {
        return Err(StdError::generic_err(format!("Insufficient luna deposit: luna_amount={}, required={}", luna_amount, config.minimum_luna)));
    }

    // Check whether token is already registered
    let registered = pair_get(&deps.storage, *channel_id);
    if registered.is_some() {
        let registered_h = deps.api.human_address(&registered.clone().unwrap().0);
        let registrar_h = deps.api.human_address(&registered.unwrap().1);
        return Err(StdError::generic_err(format!("Token is already registered channel_id: {}, registered_token_address: {:?}, registrar_address: {:?}", *channel_id, registered_h, registrar_h)));
    }

    // Check whether the sender is the owner of the token contract
    if config.owner != env.message.sender
    {
        return Err(StdError::generic_err(format!("You are not authorized to execute this function. owner: {}, sender: {}", config.owner, env.message.sender)));
    }

    // Register token in Tokens
    pair_set(
        &mut deps.storage,
        *channel_id,
        Some((token_canonical.clone(), env.message.sender.clone()))
    )?;
    // Register each reserve in reserves
    reserve_set(&mut deps.storage, *channel_id, Some((*luna_amount, *token_amount)))?;

    let token_transfer_from =
        create_transfer_from_msg(&deps.api, &token_canonical, sender_h.clone(), contract_h.clone(), *token_amount, Some(vec![Coin {
            denom: "uluna".to_string(),
            amount: *luna_amount,
        }]))?;
    let res = HandleResponse {
        messages: vec![token_transfer_from],
        log: vec![log("action", "add_liquidity"),
                  log("registrar", deps.api.human_address(&env.message.sender)?),
                  log("to", contract_h),
                  log("luna_amount", *luna_amount),
                  log("token_amount", *token_amount)],
        data: None,
    };

    Ok(res)
}

/// Swap LUNA to a token from this contract address with sell order
/// deps: dependancies for cosmos SDK:  Storage S, Api A, Querier Q
/// env: Environment of tx input
/// amount: amount of token to swap
/// channel_id: Identifier for token in BIP standard
/// recipient: address to receive LUNA
/// returns
/// StdResult<HandleResponse>: Result() of response message to send to cosmos SDK
fn try_swap_to_luna<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    amount: &Uint128,
    channel_id: &Uint128,
    recipient: &HumanAddr,
) -> StdResult<HandleResponse> {
    let sender_h = deps.api.human_address(&env.message.sender)?;
    let contract_h = deps.api.human_address(&env.contract.address)?;
    // Check if token is registered from pair
    if pair_get(&deps.storage, *channel_id).is_none() {
        return Err(StdError::generic_err("Token is not registered yet"));
    }

    // Get price from each reserve Index 0: Luna, Index 1: Token
    let reserves: (Uint128, Uint128) = reserve_get(&deps.storage, *channel_id).unwrap();
    let token_reserve: Uint128 = reserves.1;
    let luna_reserve: Uint128 = reserves.0;
    let luna_bought: Uint128 = get_input_price(*amount, token_reserve, luna_reserve)?;
    // Change reserve amount
    let new_token_reserve: Uint128 = token_reserve + *amount;
    let new_luna_reserve: Uint128 = (luna_reserve - luna_bought)?;
    reserve_set(&mut deps.storage, *channel_id, Some((new_luna_reserve, new_token_reserve)))?;

    // Get token and send luna to recipient address
    let channel = pair_get(&deps.storage, *channel_id).unwrap();
    let token_transfer_from =
        create_transfer_from_msg(&deps.api, &channel.0, sender_h.clone(), contract_h.clone(), *amount, None)?;

    let luna_transfer = BankMsg::Send {
        from_address: contract_h,
        to_address: recipient.clone(),
        amount: vec![Coin {
            denom: "uluna".to_string(),
            amount: luna_bought,
        }],
    }
    .into();

    let res = HandleResponse {
        messages: vec![token_transfer_from, luna_transfer],
        log: vec![log("action", "swap_to_luna"),
                  log("from", deps.api.human_address(&env.message.sender)?),
                  log("to", recipient),
                  log("input_amount", *amount),
                  log("luna_amount", luna_bought)],
        data: None,
    };

    Ok(res)
}

// Buy luna fully using limited amount of tokens in a channel
fn try_swap_to_luna_output<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    luna_bought: &Uint128,
    max_tokens: &Uint128,
    deadline: &Uint128,
    channel_id: &Uint128,
    recipient: &HumanAddr,
) -> StdResult<HandleResponse> {
    let sender_h = deps.api.human_address(&env.message.sender)?;
    let contract_h = deps.api.human_address(&env.contract.address)?;
    
    if *deadline >= Uint128(env.block.height as u128) && (*luna_bought > Uint128(0)) {}
    else {        
        return Err(StdError::generic_err(format!("Invalid request: one of three inputs is lower or equal to zero")));
    }

    let reserves: (Uint128, Uint128) = reserve_get(&deps.storage, *channel_id).unwrap();
    let token_reserve: Uint128 = reserves.1;
    let luna_reserve: Uint128 = reserves.0;
    let tokens_sold = get_output_price(*luna_bought, token_reserve, luna_reserve);
    if *max_tokens >= tokens_sold {}
    else {        
        return Err(StdError::generic_err(format!("Invalid request: max_tokens is lower than the tokens_sold")));
    }

    
    let channel = pair_get(&deps.storage, *channel_id).unwrap();
    let token_transfer_from =
        create_transfer_from_msg(&deps.api, &channel.0, sender_h.clone(), contract_h.clone(), tokens_sold, None)?;


    let luna_transfer = BankMsg::Send {
        from_address: contract_h,
        to_address: recipient.clone(),
        amount: vec![Coin {
            denom: "uluna".to_string(),
            amount: *luna_bought,
        }],
    }
    .into();
        
    let res = HandleResponse {
        messages: vec![luna_transfer, token_transfer_from],
        log: vec![log("action", "swap_to_luna_output"),
                  log("from", deps.api.human_address(&env.message.sender)?),
                  log("to", recipient),
                  log("max_tokens_amount", *max_tokens),
                  log("luna_output_amount", *luna_bought)],
        data: None,
    };

    Ok(res)
}

/// Swap Luna to a token from this contract address with sell order
/// deps: dependancies for cosmos SDK:  Storage S, Api A, Querier Q
/// env: Environment of tx input
/// amount: amount of Luna to swap
/// channel_id: Identifier for token in BIP standard
/// recipient: address to receive Token
/// returns
/// StdResult<HandleResponse>: Result() of response message to send to cosmos SDK
fn try_swap_to_token<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    amount: &Uint128,
    channel_id: &Uint128,
    recipient: &HumanAddr,
) -> StdResult<HandleResponse> {
    let sender_h = deps.api.human_address(&env.message.sender)?;
    let contract_h = deps.api.human_address(&env.contract.address)?;
    // Check if token is registered from pair
    let channel: Option<(CanonicalAddr, CanonicalAddr)> = pair_get(&deps.storage, *channel_id);
    if channel.is_none() {
        return Err(StdError::generic_err("Token is not registered yet"));
    }

    // Get price from each reserve Index 0: Luna, Index 1: Token
    let reserves: (Uint128, Uint128) = reserve_get(&deps.storage, *channel_id).unwrap();
    let token_reserve: Uint128 = reserves.1;
    let luna_reserve: Uint128 = reserves.0;
    let input_reserve: Uint128 = (luna_reserve - *amount).unwrap();
    let tokens_bought: Uint128 = get_input_price(*amount, input_reserve, token_reserve)?;


    // Change reserve amount
    let new_luna_reserve: Uint128 = luna_reserve + *amount;
    let new_token_reserve: Uint128 = (token_reserve - tokens_bought)?;
    reserve_set(&mut deps.storage, *channel_id, Some((new_luna_reserve, new_token_reserve)))?;

    // Get token and send luna to recipient address
    let token_transfer =
        create_transfer_msg(&deps.api, &channel.unwrap().0, sender_h, tokens_bought, Some(vec![Coin {
            denom: "uluna".to_string(),
            amount: *amount,
        }]))?;

    let res = HandleResponse {
        messages: vec![token_transfer],
        log: vec![log("action", "swap_to_token"),
                  log("from", deps.api.human_address(&env.message.sender)?),
                  log("to", recipient),
                  log("input_amount", *amount),
                  log("luna_amount", tokens_bought)],
        data: None,
    };

    Ok(res)


}

/// Buy maximum amount of token with limited luna(max_luna)
fn try_swap_to_token_output<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    tokens_bought: &Uint128,
    max_luna: &Uint128,
    deadline: &Uint128,
    channel_id: &Uint128,
    recipient: &HumanAddr,
) -> StdResult<HandleResponse> {
    let sender_h = deps.api.human_address(&env.message.sender)?;
    let contract_h = deps.api.human_address(&env.contract.address)?;
    
    if *deadline >= Uint128(env.block.height as u128) && (*tokens_bought > Uint128(0) && *max_luna > Uint128(0)) {}
    else {        
        return Err(StdError::generic_err(format!("Invalid request: one of three inputs is lower or equal to zero")));
    }

    let reserves: (Uint128, Uint128) = reserve_get(&deps.storage, *channel_id).unwrap();
    let token_reserve: Uint128 = reserves.1;
    let luna_reserve: Uint128 = reserves.0;
    let luna_sold = get_output_price(*tokens_bought, (luna_reserve - *max_luna)?, token_reserve);
    let luna_refund = (*max_luna - luna_sold)?;
    
    let luna_transfer = BankMsg::Send {
        from_address: contract_h,
        to_address: recipient.clone(),
        amount: vec![Coin {
            denom: "uluna".to_string(),
            amount: luna_refund,
        }],
    }
    .into();
    let channel = pair_get(&deps.storage, *channel_id).unwrap();
    
    let token_transfer =
        create_transfer_msg(&deps.api, &channel.0, sender_h, *tokens_bought, Some(vec![Coin {
            denom: "uluna".to_string(),
            amount: luna_sold,
        }]))?;
        
    let res = HandleResponse {
        messages: vec![luna_transfer, token_transfer],
        log: vec![log("action", "swap_to_luna_output"),
                  log("from", deps.api.human_address(&env.message.sender)?),
                  log("to", recipient),
                  log("max_luna_amount", *max_luna),
                  log("token_output_amount", *tokens_bought)],
        data: None,
    };

    Ok(res)
}


/// registrar removes channel and retrieves deposited Luna and tokens
pub fn try_remove_liquidity<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    channel_id: &Uint128
) -> StdResult<HandleResponse> {

    let sender_h = deps.api.human_address(&env.message.sender)?;
    let contract_h = deps.api.human_address(&env.contract.address)?;
    // Check if token is registered from pair
    let channel: Option<(CanonicalAddr, CanonicalAddr)> = pair_get(&deps.storage, *channel_id);
    if channel.is_none() {
        return Err(StdError::generic_err("Token is not registered yet"));
    }

    // Check if the sender is the channel registrar
    if channel.clone().unwrap().1 != env.message.sender
    {
        return Err(StdError::generic_err(format!("You are not authorized to execute this function. registrar: {}, sender: {}", channel.unwrap().1, env.message.sender)));
    }

    // Get price from each reserve Index 0: Luna, Index 1: Token
    let reserves: (Uint128, Uint128) = reserve_get(&deps.storage, *channel_id).unwrap();
    let token_reserve: Uint128 = reserves.1;
    let luna_reserve: Uint128 = reserves.0;

    let token_transfer =
    create_transfer_msg(&deps.api, &channel.unwrap().0, sender_h.clone(), token_reserve, None)?;


    let luna_transfer = BankMsg::Send {
        from_address: contract_h,
        to_address: sender_h.clone(),
        amount: vec![Coin {
            denom: "uluna".to_string(),
            amount: luna_reserve,
        }],
    }
    .into();

    pair_set(&mut deps.storage, *channel_id, None)?;
    reserve_set(&mut deps.storage, *channel_id, None)?;

    let res = HandleResponse {
        messages: vec![luna_transfer, token_transfer],
        log: vec![log("action", "remove_liquidity"),
                  log("removed_channel_id", *channel_id)],
        data: None,
    };

    Ok(res)
}

/// Querier interface implementation
/// Queries are specified in QueryMsg in msg.rs
pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => {
            let config = config_get(&deps.storage)?;
            let out = to_binary(&ConfigResponse {
                minimum_luna: config.minimum_luna,
                owner: deps.api.human_address(&config.owner)?,
            })?;
            Ok(out)
        }
        QueryMsg::Pair { channel_id } => {
            let token_address: HumanAddr = match pair_get(&deps.storage, channel_id) {
                Some(n) => {
                    deps.api.human_address(&n.0)?
                }
                None => {
                    HumanAddr::from("0")
                }
            };

            let out = to_binary(&PairResponse {
                token_address,
            })?;
            Ok(out)
        }
        QueryMsg::Reserve { channel_id } => {
            let reserves: (Uint128,  Uint128) = reserve_get(&deps.storage, channel_id).unwrap();
            let out = to_binary(&ReserveResponse { reserves })?;
            Ok(out)
        }
    }
}

/// Execute TransferFrom message in ERC20 cosmwasm contract
fn create_transfer_from_msg<A: Api>(
    api: &A,
    contract: &CanonicalAddr,
    owner: HumanAddr,
    recipient: HumanAddr,
    amount: Uint128,
    send: Option<Vec<Coin>>
) -> StdResult<CosmosMsg> {
    let msg = ERC20HandleMsg::TransferFrom {
        owner,
        recipient,
        amount: amount,
    };
    let exec = match send {
        Some(n) => WasmMsg::Execute {
            contract_addr: api.human_address(contract)?,
            msg: to_binary(&msg)?,
            send: n
        },
        None => WasmMsg::Execute {
            contract_addr: api.human_address(contract)?,
            msg: to_binary(&msg)?,
            send: vec![],
        }
    };
    Ok(exec.into())
}

/// Execute TransferFrom message in ERC20 cosmwasm contract
fn create_transfer_msg<A: Api>(
    api: &A,
    contract: &CanonicalAddr,
    recipient: HumanAddr,
    amount: Uint128,
    send: Option<Vec<Coin>>
) -> StdResult<CosmosMsg> {
    let msg = ERC20HandleMsg::Transfer {
        recipient,
        amount,
    };
    let exec = WasmMsg::Execute {
        contract_addr: api.human_address(contract)?,
        msg: to_binary(&msg)?,
        send: vec![],
    };
    Ok(exec.into())
}

/// Get input price like UniswapV1 for sell order
/// sell order means selling assets with following equation x*y = k
/// fee is 0.3% of the input and it is kept within contract
/// input_amount: amount of asset to exchange from
/// input_reserve: reserve of asset to exchange from in Pair state
/// output_reserve: reserve of asset to exchange to in Pair state
///
/// returns (u128, u128) as (numerator, denominator)
fn get_input_price(
    input_amount: Uint128,
    input_reserve: Uint128,
    output_reserve: Uint128,
) -> StdResult<Uint128> {
    if input_reserve > Uint128(0) && output_reserve > Uint128(0) {
        let input_amount_with_fee = input_amount.u128() * 997;
        let numerator: u128 = input_amount_with_fee * output_reserve.u128();
        let denominator: u128 = (input_reserve.u128() * 1000) + input_amount.u128();
        return Ok(Uint128(numerator / denominator));
    }
    else {
        return Err(StdError::generic_err(format!("invalid reserves luna:{}, token:{}", output_reserve, input_reserve)));
    }
}

/// Get output price like UniswapV1 for buy order
/// buy order is a limit order to buy assets
/// fee is 0.3% of the input and it is kept within contract
/// TODO: implement buy order
/// output_amount: amount of asset to exchange to
/// input_reserve: reserve of asset to exchange from in Pair state
/// output_reserve: reserve of asset to exchange to in Pair state
///
/// returns (u128, u128) as (numerator, denominator)
fn get_output_price(
    output_amount: Uint128,
    input_reserve: Uint128,
    output_reserve: Uint128,
) -> Uint128 {
    assert!(input_reserve > Uint128(0) && output_reserve > Uint128(0));
    let numerator: u128 = input_reserve.u128() * output_amount.u128() * 1000;
    let denominator: u128 = output_reserve.u128() - output_amount.u128() * 997;
    return Uint128(numerator + denominator / denominator.clone()); // (numerator / denominator) + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{config, config_get, pair_get, pair_set, reserve_get, reserve_set, Config};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, MOCK_CONTRACT_ADDR};
    use cosmwasm_std::{coin, coins, CanonicalAddr, CosmosMsg, StdError, Uint128};

    const CANONICAL_LENGTH: usize = 20;

    // TODO: test each HandleMsg functions
}
