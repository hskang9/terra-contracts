use std::cmp::min;

use cosmwasm_std::{log, to_binary, to_vec, Api, BankMsg, Binary, Coin, CosmosMsg, Env, Extern, HandleResponse, HumanAddr, InitResponse, Querier, QueryRequest, StdResult, Storage, Uint128, StdError, Empty, WasmMsg};
use terra_bindings::{
    create_swap_msg, create_swap_send_msg, TerraMsgWrapper, TerraQuerier, TerraQueryWrapper,
};

use crate::msg::{ConfigResponse, ExchangeRateResponse, HandleMsg, InitMsg, QueryMsg, SimulateResponse, Cw20TransferFromMsg};
use crate::state::{config, config_read, State};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {
        total_luna_supply: Uint128(0),
        minimum_luna: Uint128(1000),
        owner: env.message.sender,
    };

    config(&mut deps.storage).save(&state)?;

    Ok(InitResponse::default())
}

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


/// Deposits LUNA and token to this contract address
fn try_add_liquidity<S: Storage, A:Api, Q: Querier>(
    deps: &mut Extern<S,A,Q>,
    env: Env,
    luna_amount: &Uint128,
    token_amount: &Uint128,
    token_address: &HumanAddr,
    token_id: &Uint128
) -> StdResult<HandleResponse<Empty>> {
    // Check luna_amount > minimumLuna
    // Check whether token is already registered
    // Check whether the sender is the owner of the token contract

    // Register token in Tokens

    // Register each reserve in reserves


    // Send msg to send each asset to contract address
    let luna_transfer = CosmosMsg::Bank(BankMsg::Send {
        from_address: HumanAddr(env.message.sender.to_string()),
        to_address: HumanAddr(env.contract.address.to_string()),
        amount: vec![Coin{
            denom: "LUNA".to_string(),
            amount: *luna_amount,
        }]
    });
    let token_transfer = Cw20TransferFromMsg {
        owner:HumanAddr(env.message.sender.to_string()),
        recipient: HumanAddr(env.contract.address.to_string()),
        amount: *token_amount,
    }.into_cosmos_msg(token_address.clone())?;

    let res = HandleResponse {
        messages: vec![luna_transfer, token_transfer],
        log: vec![],
        data: None
    };

    Ok(res)
}

/// Swap LUNA to a token from this contract address
fn try_swap_to_luna<S: Storage, A:Api, Q: Querier>(
    deps: &mut Extern<S,A,Q>,
    env: Env,
    amount: &Uint128,
    token_id: &Uint128,
    recipient: &HumanAddr
) -> StdResult<HandleResponse<Empty>> {
    // Check if token is registered from pair

    // Get price from each reserve

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

    let res = HandleResponse {
        messages: vec![luna_transfer],
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
    // Check if token is registered from pair

    // Get price from each reserve

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
    let token_address = HumanAddr("".to_string());
    let token_transfer = Cw20TransferFromMsg {
        owner:HumanAddr(env.contract.address.to_string()),
        recipient: HumanAddr(env.message.sender.to_string()),
        amount: *amount,
    }.into_cosmos_msg(token_address)?;


    let res = HandleResponse {
        messages: vec![luna_transfer, token_transfer],
        log: vec![],
        data: None,
    };

    Ok(res)
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        ConfigResponse
    }
}


