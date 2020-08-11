use crate::msg::{
    ConfigResponse, ERC20HandleMsg, HandleMsg, InitMsg, PairResponse, QueryMsg, ReserveResponse,
};
use crate::state::{
    config, config_get, fee_get, fee_set, pair_get, pair_set, reserve_get, reserve_set, share_get,
    share_set, total_share_get, total_share_set, Config,
};
use cosmwasm_std::{
    log, to_binary, Api, BankMsg, Binary, CanonicalAddr, Coin, CosmosMsg, Empty, Env, Extern,
    HandleResponse, HumanAddr, InitResponse, Querier, StdError, StdResult, Storage, Uint128,
    WasmMsg,
};

/// Contract instantiation tx
/// tx inputs are specified in InitMsg in msg.rs file
pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
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
        HandleMsg::WithdrawLiquidity { channel_id } => {
            try_withdraw_liquidity(deps, env, &channel_id)
        }
        HandleMsg::SwapToTokenOutput {
            tokens_bought,
            max_luna,
            deadline,
            channel_id,
            recipient,
        } => try_swap_to_token_output(
            deps,
            env,
            &tokens_bought,
            &max_luna,
            &deadline,
            &channel_id,
            &recipient,
        ),
        HandleMsg::SwapToLunaOutput {
            luna_bought,
            max_tokens,
            deadline,
            channel_id,
            recipient,
        } => try_swap_to_luna_output(
            deps,
            env,
            &luna_bought,
            &max_tokens,
            &deadline,
            &channel_id,
            &recipient,
        ),
        HandleMsg::WithdrawFee { channel_id } => try_withdraw_fee(deps, env, &channel_id),
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
pub fn try_add_liquidity<S: Storage, A: Api, Q: Querier>(
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
        return Err(StdError::generic_err(format!(
            "Insufficient luna deposit: luna_amount={}, required={}",
            luna_amount, config.minimum_luna
        )));
    }

    // Check whether channel is already registered
    let registered = pair_get(&deps.storage, *channel_id);
    if registered.is_some() {
        // increment each reserve in the state
        let reserves = reserve_get(&deps.storage, *channel_id).unwrap();
        reserve_set(
            &mut deps.storage,
            *channel_id,
            Some((reserves.0 + *luna_amount, reserves.1 + *token_amount)),
        )?;
        // Set share for liquidity provider
        share_set(
            &mut deps.storage,
            *channel_id,
            env.message.sender.clone(),
            Some((*luna_amount, *token_amount)),
        )?;
        // Set total share of liquidity providers
        let total_shares = total_share_get(&deps.storage, *channel_id).unwrap();
        total_share_set(
            &mut deps.storage,
            *channel_id,
            Some((
                total_shares.0 + *luna_amount,
                total_shares.1 + *token_amount,
            )),
        )?;
    } else {
        // TODO: replace registrar with specific lunar address value
        pair_set(
            &mut deps.storage,
            *channel_id,
            Some((token_canonical.clone(), env.message.sender.clone())),
        )?;
        // Register each reserve in the state
        reserve_set(
            &mut deps.storage,
            *channel_id,
            Some((*luna_amount, *token_amount)),
        )?;
        fee_set(
            &mut deps.storage,
            *channel_id,
            Some((Uint128(0), Uint128(0))),
        )?;
        // Set share for liquidity provider
        share_set(
            &mut deps.storage,
            *channel_id,
            env.message.sender.clone(),
            Some((*luna_amount, *token_amount)),
        )?;
        // Set total share of liquidity providers
        total_share_set(
            &mut deps.storage,
            *channel_id,
            Some((*luna_amount, *token_amount)),
        )?;
    }
    // Check if one sent luna enough
    if env.message.sent_funds[0].denom == "uluna".to_string()
        && env.message.sent_funds[0].amount != *luna_amount
    {
        return Err(StdError::generic_err(format!(
            "Insufficient luna deposit: luna_amount={}, required={}",
            luna_amount, env.message.sent_funds[0].amount
        )));
    }
    let token_transfer_from = create_transfer_from_msg(
        &deps.api,
        &token_canonical,
        sender_h,
        contract_h.clone(),
        *token_amount,
        None,
    )?;
    let res = HandleResponse {
        messages: vec![token_transfer_from],
        log: vec![
            log("action", "add_liquidity"),
            log("registrar", deps.api.human_address(&env.message.sender)?),
            log("to", contract_h),
            log("luna_amount", *luna_amount),
            log("token_amount", *token_amount),
        ],
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
    // Check if token is registered from pair
    let channel: Option<(CanonicalAddr, CanonicalAddr)> = pair_get(&deps.storage, *channel_id);
    if channel.is_none() {
        return Err(StdError::generic_err("Token is not registered yet"));
    }

    // Extract fee Index 0: Luna, Index 1: Token
    let fee = amount.multiply_ratio(1u128, 1000u128);
    let fees = fee_get(&deps.storage, *channel_id).unwrap();
    let token_fee = fees.1;
    let new_token_fee = token_fee + fee;
    fee_set(
        &mut deps.storage,
        *channel_id,
        Some((fees.0, new_token_fee)),
    )?;
    let amt = (*amount - fee)?;

    // Get price from each reserve Index 0: Luna, Index 1: Token
    let reserves: (Uint128, Uint128) = reserve_get(&deps.storage, *channel_id).unwrap();
    let token_reserve: Uint128 = reserves.1;
    let luna_reserve: Uint128 = reserves.0;
    let luna_bought: Uint128 = get_input_price(amt, token_reserve, luna_reserve)?;
    // Change reserve amount
    let new_token_reserve: Uint128 = token_reserve + amt;
    let new_luna_reserve: Uint128 = (luna_reserve - luna_bought)?;
    reserve_set(
        &mut deps.storage,
        *channel_id,
        Some((new_luna_reserve, new_token_reserve)),
    )?;

    // Get token and send luna to recipient address
    let sender_h = deps.api.human_address(&env.message.sender)?;
    let contract_h = deps.api.human_address(&env.contract.address)?;
    let token_transfer_from = create_transfer_from_msg(
        &deps.api,
        &channel.unwrap().0,
        sender_h.clone(),
        contract_h.clone(),
        *amount,
        None,
    )?;
    let luna_transfer: CosmosMsg = CosmosMsg::Bank(BankMsg::Send {
        from_address: contract_h,
        to_address: sender_h,
        amount: vec![Coin {
            denom: "uluna".to_string(),
            amount: luna_bought.clone(),
        }],
    });
    let res = HandleResponse {
        messages: vec![token_transfer_from, luna_transfer],
        log: vec![
            log("action", "swap_to_luna"),
            log("from", deps.api.human_address(&env.message.sender)?),
            log("to", recipient),
            log("input_amount", *amount),
            log("luna_amount", luna_bought),
        ],
        data: None,
    };

    Ok(res)
}

// Buy maximum amount of luna using limited amount of tokens in a channel
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

    if *deadline >= Uint128(env.block.height as u128) && (*luna_bought > Uint128(0)) {
    } else {
        return Err(StdError::generic_err(format!(
            "Invalid request: one of three inputs is lower or equal to zero"
        )));
    }

    let reserves: (Uint128, Uint128) = reserve_get(&deps.storage, *channel_id).unwrap();
    let token_reserve: Uint128 = reserves.1;
    let luna_reserve: Uint128 = reserves.0;
    // Check if the proposed price is right
    let tokens_sold = get_output_price(*luna_bought, token_reserve, luna_reserve);
    if *max_tokens >= tokens_sold {
    } else {
        return Err(StdError::generic_err(format!(
            "Invalid request: max_tokens should be bigger than the tokens_sold"
        )));
    }

    // Extract fee
    let fee = luna_bought.multiply_ratio(1u128, 1000u128);
    let fees = fee_get(&deps.storage, *channel_id).unwrap();
    let luna_fee = fees.0;
    let new_luna_fee = luna_fee + fee;
    fee_set(&mut deps.storage, *channel_id, Some((new_luna_fee, fees.1)))?;
    let amt = (*luna_bought - fee)?;

    let channel = pair_get(&deps.storage, *channel_id).unwrap();
    let token_transfer_from = create_transfer_from_msg(
        &deps.api,
        &channel.0,
        sender_h.clone(),
        contract_h.clone(),
        amt,
        None,
    )?;

    let luna_transfer = BankMsg::Send {
        from_address: contract_h,
        to_address: recipient.clone(),
        amount: vec![Coin {
            denom: "uluna".to_string(),
            amount: amt,
        }],
    }
    .into();
    let res = HandleResponse {
        messages: vec![luna_transfer, token_transfer_from],
        log: vec![
            log("action", "swap_to_luna_output"),
            log("from", deps.api.human_address(&env.message.sender)?),
            log("to", recipient),
            log("max_tokens_amount", *max_tokens),
            log("luna_output_amount", amt),
        ],
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
    // Check if token is registered from pair
    let channel: Option<(CanonicalAddr, CanonicalAddr)> = pair_get(&deps.storage, *channel_id);
    if channel.is_none() {
        return Err(StdError::generic_err("Channel is not registered yet"));
    }

    // Extract fee
    let fee = amount.multiply_ratio(1u128, 1000u128);
    let fees = fee_get(&deps.storage, *channel_id).unwrap();
    let luna_fee = fees.0;
    let new_luna_fee = luna_fee + fee;
    fee_set(&mut deps.storage, *channel_id, Some((new_luna_fee, fees.1)))?;
    let amt = (*amount - fee)?;

    // Get price from each reserve Index 0: Luna, Index 1: Token
    let reserves: (Uint128, Uint128) = reserve_get(&deps.storage, *channel_id).unwrap();
    let token_reserve: Uint128 = reserves.1;
    let luna_reserve: Uint128 = reserves.0;
    let input_reserve: Uint128 = (luna_reserve - amt)?;
    let tokens_bought: Uint128 = get_input_price(amt, input_reserve, token_reserve)?;

    // Change reserve amount
    let new_luna_reserve: Uint128 = luna_reserve + amt;
    let new_token_reserve: Uint128 = (token_reserve - tokens_bought)?;
    reserve_set(
        &mut deps.storage,
        *channel_id,
        Some((new_luna_reserve, new_token_reserve)),
    )?;

    // Receive luna and send token to recipient address
    let token_transfer = create_transfer_msg(
        &deps.api,
        &channel.unwrap().0,
        recipient.clone(),
        tokens_bought,
        Some(vec![Coin {
            denom: "uluna".to_string(),
            amount: *amount,
        }]),
    )?;

    let res = HandleResponse {
        messages: vec![token_transfer],
        log: vec![
            log("action", "swap_to_token"),
            log("from", deps.api.human_address(&env.message.sender)?),
            log("to", recipient),
            log("input_amount", *amount),
            log("token_amount", tokens_bought),
        ],
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

    if *deadline >= Uint128(env.block.height as u128)
        && (*tokens_bought > Uint128(0) && *max_luna > Uint128(0))
    {
    } else {
        return Err(StdError::generic_err(format!(
            "Invalid request: one of three inputs is lower or equal to zero"
        )));
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
    // Extract fee
    let fee = tokens_bought.multiply_ratio(1u128, 1000u128);
    let fees = fee_get(&deps.storage, *channel_id).unwrap();
    let token_fee = fees.1;
    let new_token_fee = token_fee + fee;
    fee_set(
        &mut deps.storage,
        *channel_id,
        Some((fees.0, new_token_fee)),
    )?;
    let amt = (*tokens_bought - fee)?;

    let token_transfer = create_transfer_msg(
        &deps.api,
        &channel.0,
        sender_h,
        amt,
        Some(vec![Coin {
            denom: "uluna".to_string(),
            amount: luna_sold,
        }]),
    )?;
    let res = HandleResponse {
        messages: vec![luna_transfer, token_transfer],
        log: vec![
            log("action", "swap_to_luna_output"),
            log("from", deps.api.human_address(&env.message.sender)?),
            log("to", recipient),
            log("max_luna_amount", *max_luna),
            log("token_output_amount", amt),
        ],
        data: None,
    };

    Ok(res)
}

/// registrar removes channel and retrieves deposited Luna and tokens
pub fn try_withdraw_liquidity<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    channel_id: &Uint128,
) -> StdResult<HandleResponse> {
    let sender_h = deps.api.human_address(&env.message.sender)?;
    let contract_h = deps.api.human_address(&env.contract.address)?;
    // Check if channel is registered from pair
    let channel: Option<(CanonicalAddr, CanonicalAddr)> = pair_get(&deps.storage, *channel_id);
    if channel.is_none() {
        return Err(StdError::generic_err("Token is not registered yet"));
    }

    // Get each reserve Index 0: Luna, Index 1: Token
    let reserves: (Uint128, Uint128) = reserve_get(&deps.storage, *channel_id).unwrap();
    let token_reserve: Uint128 = reserves.1;
    let luna_reserve: Uint128 = reserves.0;

    // Get share based the liquidity provider's contribution
    let shares: (Uint128, Uint128) =
        share_get(&deps.storage, *channel_id, env.message.sender.clone()).unwrap();
    let total_shares: (Uint128, Uint128) = total_share_get(&deps.storage, *channel_id).unwrap();
    let luna_share = luna_reserve.u128() * (shares.0.u128() / total_shares.0.u128());
    let token_share = token_reserve.u128() * (shares.1.u128() / total_shares.1.u128());

    let token_transfer = create_transfer_msg(
        &deps.api,
        &channel.unwrap().0,
        sender_h.clone(),
        Uint128(token_share),
        None,
    )?;

    let luna_transfer = BankMsg::Send {
        from_address: contract_h,
        to_address: sender_h.clone(),
        amount: vec![Coin {
            denom: "uluna".to_string(),
            amount: Uint128(luna_share),
        }],
    }
    .into();

    pair_set(&mut deps.storage, *channel_id, None)?;
    reserve_set(&mut deps.storage, *channel_id, None)?;

    let res = HandleResponse {
        messages: vec![luna_transfer, token_transfer],
        log: vec![
            log("action", "remove_liquidity"),
            log("removed_channel_id", *channel_id),
        ],
        data: None,
    };

    Ok(res)
}

/// Withdraw fee
fn try_withdraw_fee<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    channel_id: &Uint128,
) -> StdResult<HandleResponse> {
    let sender_h = deps.api.human_address(&env.message.sender)?;
    let owner = config_get(&deps.storage)?.owner;
    let contract_h = deps.api.human_address(&env.contract.address)?;

    // Check if channel is registered from pair
    let channel: Option<(CanonicalAddr, CanonicalAddr)> = pair_get(&deps.storage, *channel_id);
    if channel.is_none() {
        return Err(StdError::generic_err("Token is not registered yet"));
    }

    // Check if the sender is the contract owner
    if owner != env.message.sender {
        return Err(StdError::generic_err(format!(
            "You are not authorized to execute this function. owner: {}, sender: {}",
            owner, env.message.sender
        )));
    }

    // Get each reserve Index 0: Luna, Index 1: Token
    let reserves: (Uint128, Uint128) = fee_get(&deps.storage, *channel_id).unwrap();
    let token_fee: Uint128 = reserves.1;
    let luna_fee: Uint128 = reserves.0;

    let token_transfer = create_transfer_msg(
        &deps.api,
        &channel.unwrap().0,
        sender_h.clone(),
        token_fee,
        None,
    )?;

    let luna_transfer = BankMsg::Send {
        from_address: contract_h,
        to_address: sender_h.clone(),
        amount: vec![Coin {
            denom: "uluna".to_string(),
            amount: luna_fee,
        }],
    }
    .into();

    fee_set(
        &mut deps.storage,
        *channel_id,
        Some((Uint128(0), Uint128(0))),
    )?;
    let res = HandleResponse {
        messages: vec![luna_transfer, token_transfer],
        log: vec![
            log("action", "remove_liquidity"),
            log("withdrawn luna", luna_fee),
            log("withdrawn token", token_fee),
        ],
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
                Some(n) => deps.api.human_address(&n.0)?,
                None => HumanAddr::from("0"),
            };

            let out = to_binary(&PairResponse { token_address })?;
            Ok(out)
        }
        QueryMsg::Reserve { channel_id } => {
            let reserves: (Uint128, Uint128) = reserve_get(&deps.storage, channel_id).unwrap();
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
    send: Option<Vec<Coin>>,
) -> StdResult<CosmosMsg> {
    let msg = ERC20HandleMsg::TransferFrom {
        owner,
        recipient,
        amount: amount,
    };
    let exec = match send {
        Some(vector) => WasmMsg::Execute {
            contract_addr: api.human_address(contract)?,
            msg: to_binary(&msg)?,
            send: vector,
        },
        None => WasmMsg::Execute {
            contract_addr: api.human_address(contract)?,
            msg: to_binary(&msg)?,
            send: vec![],
        },
    };
    Ok(exec.into())
}

/// Execute TransferFrom message in ERC20 cosmwasm contract
fn create_transfer_msg<A: Api>(
    api: &A,
    contract: &CanonicalAddr,
    recipient: HumanAddr,
    amount: Uint128,
    send: Option<Vec<Coin>>,
) -> StdResult<CosmosMsg> {
    let msg = ERC20HandleMsg::Transfer { recipient, amount };
    let exec = match send {
        Some(vector) => WasmMsg::Execute {
            contract_addr: api.human_address(contract)?,
            msg: to_binary(&msg)?,
            send: vector,
        },
        None => WasmMsg::Execute {
            contract_addr: api.human_address(contract)?,
            msg: to_binary(&msg)?,
            send: vec![],
        },
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
/// returns price in Uint128
fn get_input_price(
    input_amount: Uint128,
    input_reserve: Uint128,
    output_reserve: Uint128,
) -> StdResult<Uint128> {
    if input_reserve > Uint128(0) && output_reserve > Uint128(0) {
        let input_amount_with_fee = input_amount.u128() * 997;
        let numerator: u128 = input_amount_with_fee * output_reserve.u128();
        let denominator: u128 = (input_reserve.u128() * 1000) + input_amount_with_fee;
        return Ok(Uint128(numerator / denominator));
    } else {
        return Err(StdError::generic_err(format!(
            "invalid reserves luna:{}, token:{}",
            output_reserve, input_reserve
        )));
    }
}

/// Get output price like UniswapV1 for buy order
/// buy order is a limit order to buy assets
/// fee is 0.3% of the input and it is kept within contract
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
    #[test]
    fn try_add_liquidity_works() {
        let mut deps = mock_dependencies(CANONICAL_LENGTH, &[]);
        let amount1 = Uint128::from(11223344u128);
        let owner = HumanAddr::from("addr0001");
        let amount2 = Uint128::from(7890987u128);
        let liquidity_provider = HumanAddr::from("addr0002");
    }
}
