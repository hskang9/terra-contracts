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

    // Check whether token is already registered
    // Check whether the sender is the owner of the token contract

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
        from_address: HumanAddr(env.message.sender.to_string()),
        to_address: HumanAddr(env.contract.address.to_string()),
        amount: vec![Coin {
            denom: "LUNA".to_string(),
            amount: *luna_amount,
        }],
    });
    let token_canonical = deps.api.canonical_address(token_address)?;
    let token_transfer =
        create_transfer_msg(&deps.api, &token_canonical, contract_h, *token_amount)?;

    let res = HandleResponse {
        messages: vec![luna_transfer, token_transfer],
        log: vec![],
        data: None,
    };

    Ok(res)
}

/// Swap LUNA to a token from this contract address
/// deps: Component to interact with cosmos Storage S, Api A, Querier Q
/// env: Environment of tx input
/// amount: amount of token to swp
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

    // Get price from each reserve Index 0: Luna, Index 1: Token

    // Change reserve amount

    // Get token and send luna to recipient address
    // Send msg to send each asset to contract address

    let luna_transfer = BankMsg::Send {
        from_address: HumanAddr(env.contract.address.to_string()),
        to_address: HumanAddr(env.message.sender.to_string()),
        amount: vec![Coin {
            denom: "LUNA".to_string(),
            amount: *amount,
        }],
    }
    .into();
    let token_address = pair_get(&deps.storage, *token_id)?;
    let token_transfer =
        create_transfer_from_msg(&deps.api, &token_address, sender_h, contract_h, *amount)?;

    let res = HandleResponse {
        messages: vec![luna_transfer, token_transfer],
        log: vec![],
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
/// input_amount: amount of asset to exchange from
/// input_reserve: reserve of asset to exchange from in Pair state
/// output_reserve: reserve of asset to exchange to in Pair state
///
/// returns (u128, u128) as (numerator, denominator)
fn get_input_price(
    input_amount: Uint128,
    input_reserve: Uint128,
    output_reserve: Uint128,
) -> (u128, u128) {
    assert!(input_reserve > Uint128(0) && output_reserve > Uint128(0));
    let input_amount_with_fee: u128 = input_amount.u128() * 997;
    let numerator: u128 = input_amount_with_fee * output_reserve.u128();
    let denominator: u128 = (input_reserve.u128() * 1000) + input_amount_with_fee;
    return (numerator, denominator);
}

/// Get output price like UniswapV1
/// output_amount: amount of asset to exchange to
/// input_reserve: reserve of asset to exchange from in Pair state
/// output_reserve: reserve of asset to exchange to in Pair state
///
/// returns (u128, u128) as (numerator, denominator)
fn get_output_price(
    output_amount: Uint128,
    input_reserve: Uint128,
    output_reserve: Uint128,
) -> (u128, u128) {
    assert!(input_reserve > Uint128(0) && output_reserve > Uint128(0));
    let numerator: u128 = input_reserve.u128() * output_amount.u128() * 1000;
    let denominator: u128 = (output_reserve.u128() - output_amount.u128()) * 997;
    return (numerator + denominator, denominator); // (numerator / denominator) + 1
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
