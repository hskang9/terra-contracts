use std::cmp::min;

use cosmwasm_std::{log, CanonicalAddr, to_binary, to_vec, Api, BankMsg, Binary, Coin, CosmosMsg, Env, Extern, HandleResponse, HumanAddr, InitResponse, Querier, QueryRequest, StdResult, Storage, Uint128, StdError, Empty, WasmMsg};


use crate::msg::{ConfigResponse, ERC20HandleMsg, HandleMsg, InitMsg, QueryMsg, ReserveResponse, PairResponse};
use crate::state::{config, config_get, Config, pair_get, pair_set, reserve_get, reserve_set};


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
        HandleMsg::AddLiquidity {luna_amount, token_amount, token_address, token_id} => try_add_liquidity(deps, env, &luna_amount, &token_amount, &token_address, &token_id),
        HandleMsg::SwapTokenToLuna {amount, token_id, recipient} => try_swap_to_luna(deps, env, &amount, &token_id, &recipient),
        HandleMsg::SwapLunaToToken {amount, token_id, recipient} => try_swap_to_token(deps, env, &amount, &token_id, &recipient),
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
fn try_add_liquidity<S: Storage, A:Api, Q: Querier>(
    deps: &mut Extern<S,A,Q>,
    env: Env,
    luna_amount: &Uint128,
    token_amount: &Uint128,
    token_address: &HumanAddr,
    token_id: &Uint128
) -> StdResult<HandleResponse> {
    // Human address for sender
    let senderH = deps.api.human_address(&env.message.sender)?;
    // Human address for token contract
    let contractH = deps.api.human_address(&env.contract.address)?;
    // Check luna_amount > minimumLuna

    // Check whether token is already registered
    // Check whether the sender is the owner of the token contract

    // Register token in Tokens
    pair_set(&mut deps.storage, *token_id, &deps.api.canonical_address(token_address)?);
    // Register each reserve in reserves
    reserve_set(&mut deps.storage, *token_id, (*luna_amount, *token_amount));

    // Send msg to send each asset to contract address
    let luna_transfer = CosmosMsg::Bank(BankMsg::Send {
        from_address: HumanAddr(env.message.sender.to_string()),
        to_address: HumanAddr(env.contract.address.to_string()),
        amount: vec![Coin{
            denom: "LUNA".to_string(),
            amount: *luna_amount,
        }]
    });
    let token_canonical = deps.api.canonical_address(token_address)?;
    let token_transfer = create_transfer_from_msg(&deps.api, &token_canonical, senderH, contractH, *token_amount).unwrap();

    let res = HandleResponse {
        messages: vec![luna_transfer, token_transfer],
        log: vec![],
        data: None
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
fn try_swap_to_luna<S: Storage, A:Api, Q: Querier>(
    deps: &mut Extern<S,A,Q>,
    env: Env,
    amount: &Uint128,
    token_id: &Uint128,
    recipient: &HumanAddr
) -> StdResult<HandleResponse<Empty>> {
    let senderH = deps.api.human_address(&env.message.sender)?;
    let contractH = deps.api.human_address(&env.contract.address)?;
    // Check if token is registered from pair

    // Get price from each reserve Index 0: Luna, Index 1: Token

    // Change reserve amount

    // Get token and send luna to recipient address
    // Send msg to send each asset to contract address

    let luna_transfer = CosmosMsg::Bank(BankMsg::Send {
        from_address: HumanAddr(env.contract.address.to_string()),
        to_address: HumanAddr(env.message.sender.to_string()),
        amount: vec![Coin{
            denom: "LUNA".to_string(),
            amount: *amount,
        }]
    });
    let token_address = pair_get(&deps.storage, *token_id)?;
    let token_transfer = create_transfer_from_msg(&deps.api, &token_address, senderH, contractH, *amount).unwrap();

    let res = HandleResponse {
        messages: vec![luna_transfer, token_transfer],
        log: vec![],
        data: None
    };

    Ok(res)
}


/// Swap LUNA to a token from this contract address
fn try_swap_to_token<S: Storage, A:Api, Q: Querier>(
    deps: &mut Extern<S,A,Q>,
    env: Env,
    amount: &Uint128,
    token_id: &Uint128,
    recipient: &HumanAddr
) -> StdResult<HandleResponse<Empty>> {
    let senderH = deps.api.human_address(&env.message.sender)?;
    let contractH = deps.api.human_address(&env.contract.address)?;
    // Check if token is registered from pair

    // Get price from each reserve Index 0: Luna, Index 1: Token

    // Change reserve amount

    // Get token and send luna to recipient address
    let luna_transfer = CosmosMsg::Bank(BankMsg::Send {
        from_address: HumanAddr(env.message.sender.to_string()),
        to_address: HumanAddr(env.contract.address.to_string()),
        amount: vec![Coin{
            denom: "LUNA".to_string(),
            amount: *amount,
        }]
    });
    let token_address = pair_get(&deps.storage, *token_id)?;
    let token_transfer = create_transfer_from_msg(&deps.api, &token_address, contractH, senderH, *amount).unwrap();



    let res = HandleResponse {
        messages: vec![luna_transfer, token_transfer],
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
        },
        QueryMsg::Pair { token_id } => {
            let token_address = pair_get(&deps.storage, token_id)?;
            let out = to_binary(&PairResponse {
                token_address: deps.api.human_address(&token_address)?
            })?;
            Ok(out)
        }
        QueryMsg::Reserve { token_id } => {
            let reserves = reserve_get(&deps.storage, token_id)?;
            let out = to_binary(&ReserveResponse{
                reserves
            })?;
            Ok(out)
        }
    }
}

/// Execute TransferFrom message in ERC20 cosmwasm contract
fn create_transfer_from_msg<A: Api>(api: &A, contract: &CanonicalAddr, owner: HumanAddr, recipient: HumanAddr, amount: Uint128) -> StdResult<CosmosMsg> {
    let msg = ERC20HandleMsg::TransferFrom {
        owner,
        recipient,
        amount: amount
    };
    let exec = WasmMsg::Execute {
        contract_addr: api.human_address(contract)?,
        msg: to_binary(&msg)?,
        send: vec![],
    };
    Ok(exec.into())
}

/// Execute TransferFrom message in ERC20 cosmwasm contract
fn create_transfer_msg<A: Api>(api: &A, contract: &CanonicalAddr, recipient: HumanAddr, amount: Uint128) -> StdResult<CosmosMsg> {
    let msg = ERC20HandleMsg::Transfer {
        recipient,
        amount: amount
    };
    let exec = WasmMsg::Execute {
        contract_addr: api.human_address(contract)?,
        msg: to_binary(&msg)?,
        send: vec![],
    };
    Ok(exec.into())
}

/// Execute TransferFrom message in ERC20 cosmwasm contract
fn create_dex_approve_from_msg<A: Api>(api: &A, contract: &CanonicalAddr, recipient: HumanAddr, amount: Uint128) -> StdResult<CosmosMsg> {
    let msg = ERC20HandleMsg::Transfer {
        recipient,
        amount: amount
    };
    let exec = WasmMsg::Execute {
        contract_addr: api.human_address(contract)?,
        msg: to_binary(&msg)?,
        send: vec![],
    };
    Ok(exec.into())
}


/// Get input price like UniswapV1
fn get_input_price() {
    unimplemented!()
}


/// Get output price like UniswapV1
fn get_output_price() {
    unimplemented!()
}