use std::cmp::min;

use cosmwasm_std::{
    log, to_binary, to_vec, Api, BankMsg, Binary, CanonicalAddr, Coin, CosmosMsg, Empty, Env,
    Extern, HandleResponse, HumanAddr, InitResponse, Querier, QueryRequest, StdError, StdResult,
    Storage, Uint128, WasmMsg,
};

use crate::msg::{
    ApprovedForAllResponse, ApprovedResponse, BalanceOfResponse, HandleMsg, InitMsg,
    OwnerOfResponse, QueryMsg, TokenURIResponse,
};
use crate::state::{
    config, config_get, Config,
};

/// Contract instantiation tx
/// tx inputs are specified in InitMsg in msg.rs file
pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = Config {
        name: msg.name,
        symbol: msg.symbol,
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
        HandleMsg::Buy{
            amount
        } => try_buy(deps,env,&amount),
        HandleMsg::ChangeOwnership{
            new
        } => try_change_ownership(deps, env, &new),
        HandleMsg::RevokeOwnership {
        } => try_revoke_ownership(deps, env)
    }
}

fn try_buy<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    amount: &Uint128
) -> StdResult<HandleResponse> {
    unimplemented!();
}

fn try_change_ownership<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    new: &HumanAddr
) -> StdResult<HandleResponse> {
    unimplemented!();
}

fn try_revoke_ownership<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env
) -> StdResult<HandleResponse> {
    unimplemented!();
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config{} => {Ok(to_binary(&9)?)},
        QueryMsg::OrderOf{address} => {Ok(to_binary(&9)?)}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{config, config_get,Config};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, MOCK_CONTRACT_ADDR};
    use cosmwasm_std::{coin, coins, CanonicalAddr, CosmosMsg, StdError, Uint128};

    const CANONICAL_LENGTH: usize = 20;

    // TODO: Add test cases for each HandleMsg
}
