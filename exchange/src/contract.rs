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
        total_luna_supply: Uint128(0),
        minimum_luna: msg.minimum_luna,
        owner: msg.owner,
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
            token_id,
        } => try_add_liquidity(
            deps,
            env,
            &luna_amount,
            &token_amount,
            &token_address,
            &token_id,
        ),
        HandleMsg::SwapTokenToLuna {
            amount,
            token_id,
            recipient,
        } => try_swap_to_luna(deps, env, &amount, &token_id, &recipient),
        HandleMsg::SwapLunaToToken {
            amount,
            token_id,
            recipient,
        } => try_swap_to_token(deps, env, &amount, &token_id, &recipient),
    }
}

/// Deposits LUNA and token to this contract address for registration
/// deps: Component to interact with cosmos Storage S, Api A, Querier Q
/// env: environment of the tx input
/// luna_amount: Amount of Luna to deposit to register token in Liquidity provider contract(a.k.a. uniswap)
/// token_amount: Amount of token to deposit to register token in Liquidity provider contract(a.k.a. uniswap)
/// token_id: Identifier for token following BIP standard
/// returns
/// StdResult<HandleResponse>: Result() of response message to send to cosmos SDK
fn try_add_liquidity<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    luna_amount: &Uint128,
    token_amount: &Uint128,
    token_address: &HumanAddr,
    token_id: &Uint128,
) -> StdResult<HandleResponse> {
    // Human address for sender
    let sender_h = deps.api.human_address(&env.message.sender)?;
    // Human address for token contract
    let contract_h = deps.api.human_address(&env.contract.address)?;
    // Check luna_amount > minimumLuna
    let config = config_get(&deps.storage)?;
    if *luna_amount < config.minimum_luna {
        return Err(StdError::generic_err(format!("Insufficient luna deposit: luna_amount={}, required={}", luna_amount, config.minimum_luna)));
    }

    // Check whether token is already registered
    if !pair_get(&deps.storage, *token_id).is_err() {
        return Err(StdError::generic_err("Token is already registered"));
    }

    // Check whether the sender is the owner of the token contract
    if config.owner != env.message.sender {
        return Err(StdError::unauthorized());
    }

    // Register token in Tokens
    pair_set(
        &mut deps.storage,
        *token_id,
        &deps.api.canonical_address(token_address)?,
    )?;
    // Register each reserve in reserves
    reserve_set(&mut deps.storage, *token_id, (*luna_amount, *token_amount))?;

    // Send msg to send each asset to contract address
    let luna_transfer = CosmosMsg::Bank(BankMsg::Send {
        from_address: sender_h,
        to_address: contract_h.clone(),
        amount: vec![Coin {
            denom: "uluna".to_string(),
            amount: *luna_amount,
        }],
    });
    let token_canonical = deps.api.canonical_address(token_address)?;
    let token_transfer =
        create_transfer_msg(&deps.api, &token_canonical, contract_h.clone(), *token_amount)?;

    let res = HandleResponse {
        messages: vec![luna_transfer, token_transfer],
        log: vec![log("action", "add_liquidity"),
                  log("from", deps.api.human_address(&env.message.sender)?),
                  log("to", contract_h),
                  log("luna_amount", *luna_amount),
                  log("token_amount", *token_amount)],
        data: None,
    };

    Ok(res)
}

/// Swap LUNA to a token from this contract address
/// deps: Component to interact with cosmos Storage S, Api A, Querier Q
/// env: Environment of tx input
/// amount: amount of token to swap
/// token_id: Identifier for token in BIP standard
/// recipient: address to receive LUNA
/// returns
/// StdResult<HandleResponse>: Result() of response message to send to cosmos SDK
fn try_swap_to_luna<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    amount: &Uint128,
    token_id: &Uint128,
    recipient: &HumanAddr,
) -> StdResult<HandleResponse> {
    let sender_h = deps.api.human_address(&env.message.sender)?;
    let contract_h = deps.api.human_address(&env.contract.address)?;
    // Check if token is registered from pair
    if pair_get(&deps.storage, *token_id).is_err() {
        return Err(StdError::generic_err("Token is not registered yet"));
    }

    // Get price from each reserve Index 0: Luna, Index 1: Token
    let reserves: (Uint128, Uint128) = reserve_get(&deps.storage, *token_id)?;
    let token_reserve: Uint128 = reserves.1;
    let luna_reserve: Uint128 = reserves.0;
    let luna_bought: Uint128 = get_input_price(*amount, token_reserve, luna_reserve)?;
    // Change reserve amount
    let new_luna_reserve: Uint128 = token_reserve + *amount;
    let new_token_reserve: Uint128 = (luna_reserve - luna_bought)?;
    reserve_set(&mut deps.storage, *token_id, (new_luna_reserve, new_token_reserve))?;

    // Get token and send luna to recipient address
    let token_address = pair_get(&deps.storage, *token_id)?;
    let token_transfer =
        create_transfer_msg(&deps.api, &token_address, contract_h.clone(),  *amount)?;

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
        messages: vec![token_transfer, luna_transfer],
        log: vec![log("action", "swap_to_luna"),
                  log("from", deps.api.human_address(&env.message.sender)?),
                  log("to", recipient),
                  log("input_amount", *amount),
                  log("luna_amount", luna_bought)],
        data: None,
    };

    Ok(res)
}

/// Swap LUNA to a token from this contract address
/// NOTICE: This is currently not possible due to absence of sender specification for contract in cosmwasm
/// check issue #420
fn try_swap_to_token<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    amount: &Uint128,
    token_id: &Uint128,
    recipient: &HumanAddr,
) -> StdResult<HandleResponse<Empty>> {
    let sender_h = deps.api.human_address(&env.message.sender)?;
    let contract_h = deps.api.human_address(&env.contract.address)?;
    // Check if token is registered from pair

    // Get price from each reserve Index 0: Luna, Index 1: Token

    // Change reserve amount

    // Get token and send luna to recipient address
    /*
    let luna_transfer = BankMsg::Send {
        from_address: HumanAddr(env.message.sender.to_string()),
        to_address: HumanAddr(env.contract.address.to_string()),
        amount: vec![Coin {
            denom: "LUNA".to_string(),
            amount: *amount,
        }],
    }
    .into();
    let token_address = pair_get(&deps.storage, *token_id)?;
    let token_transfer =
        create_transfer_from_msg(&deps.api, &token_address, contract_h, sender_h, *amount)?;
    */
    let res = HandleResponse {
        messages: vec![/*luna_transfer, token_transfer*/],
        log: vec![],
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
        QueryMsg::Pair { token_id } => {
            let token_address = pair_get(&deps.storage, token_id)?;
            let out = to_binary(&PairResponse {
                token_address: deps.api.human_address(&token_address)?,
            })?;
            Ok(out)
        }
        QueryMsg::Reserve { token_id } => {
            let reserves = reserve_get(&deps.storage, token_id)?;
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
) -> StdResult<CosmosMsg> {
    let msg = ERC20HandleMsg::TransferFrom {
        owner,
        recipient,
        amount: amount,
    };
    let exec = WasmMsg::Execute {
        contract_addr: api.human_address(contract)?,
        msg: to_binary(&msg)?,
        send: vec![],
    };
    Ok(exec.into())
}

/// Execute TransferFrom message in ERC20 cosmwasm contract
fn create_transfer_msg<A: Api>(
    api: &A,
    contract: &CanonicalAddr,
    recipient: HumanAddr,
    amount: Uint128,
) -> StdResult<CosmosMsg> {
    let msg = ERC20HandleMsg::Transfer {
        recipient,
        amount: amount,
    };
    let exec = WasmMsg::Execute {
        contract_addr: api.human_address(contract)?,
        msg: to_binary(&msg)?,
        send: vec![],
    };
    Ok(exec.into())
}

/// Get input price like UniswapV1
/// price follows the equation (x + delta_x) * (y) = k_0
/// fees are excluded for now
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
        let numerator: u128 = input_amount.u128() * output_reserve.u128();
        let denominator: u128 = (input_reserve.u128()) + input_amount.u128();
        return Ok(Uint128(numerator / denominator));
    }
    else {
        return Err(StdError::generic_err(format!("invalid reserves luna:{}, token:{}", output_reserve, input_reserve)));
    }
}

/// Get output price like UniswapV1
/// price follows the equation (x)(y-delta_y) = k_0
/// fees are excluded for now
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
    let numerator: u128 = input_reserve.u128() * output_amount.u128();
    let denominator: u128 = output_reserve.u128() - output_amount.u128();
    return Uint128(numerator + denominator / denominator.clone()); // (numerator / denominator) + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{config, config_get, pair_get, pair_set, reserve_get, reserve_set, Config};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, MOCK_CONTRACT_ADDR};
    use cosmwasm_std::{coin, coins, CanonicalAddr, CosmosMsg, StdError, Uint128};

    const CANONICAL_LENGTH: usize = 20;

    // State tests
    #[test]
    fn config_get_works() {
        let mut deps = mock_dependencies(CANONICAL_LENGTH, &[]);
        let config = config_get(&deps.storage);
        println!("{:?}", config);
    }

    fn config_set_works() {
        unimplemented!();
    }

    fn pair_get_works() {
        unimplemented!();
    }

    fn pair_set_works() {
        unimplemented!();
    }

    fn reserve_get_works() {
        unimplemented!();
    }

    fn reserve_set_works() {
        unimplemented!();
    }

    // Querier tests

    // Business logic tests

    /// Test input price calculation
    /// After execution, result will be:
    /// TODO: add example calculation
    fn get_input_price_works(
        input_amount: Uint128,
        input_reserve: Uint128,
        output_reserve: Uint128,
    ) {
        unimplemented!();
    }

    /// Test output price calculation
    /// After execution, result will be:
    /// TODO: add example calculation
    fn get_output_price_works(
        output_amount: Uint128,
        input_reserve: Uint128,
        output_reserve: Uint128,
    ) {
        unimplemented!();
    }

    /// Test adding liquidity
    /// After execution, result will be:
    /// sender depositing Luna and token to the exchange contract
    /// exchange contract getting token and luna bigger than minimum amount
    fn try_add_liquidity_works(
        luna_amount: &Uint128,
        token_amount: &Uint128,
        token_address: &HumanAddr,
        token_id: &Uint128,
    ) {
        unimplemented!();
    }

    /// Test token to Luna swap
    /// After execution, result will be:
    /// sender having luna based on output price
    /// exchange contract getting token and giving luna to the sender
    fn try_swap_to_luna_works(amount: &Uint128, token_id: &Uint128, recipient: &HumanAddr) {
        unimplemented!();
    }
}
