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
    config, config_get, holder_tokens_get, holder_tokens_set, operator_approvals_get,
    operator_approvals_set, token_approvals_get, token_approvals_set, token_owner_get,
    token_owner_set, token_uri_get, token_uri_set, Config,
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
        HandleMsg::Approve { to, token_id } => try_approve(deps, env, &to, &token_id),
        HandleMsg::SetApprovalForAll { operator, token_id } => {
            try_set_approval_for_all(deps, env, &operator, &token_id)
        }
        HandleMsg::Burn { token_id } => try_burn(deps, env, &token_id),
        HandleMsg::Mint { to, token_id } => try_mint(deps, env, &to, &token_id),
        HandleMsg::SetTokenURI { token_id, uri } => try_set_token_uri(deps, env, &token_id, &uri),
        HandleMsg::TransferFrom { from, to, token_id } => {
            try_transfer_from(deps, env, &from, &to, &token_id)
        }
    }
}

fn try_approve<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    to: &HumanAddr,
    token_id: &Uint128,
) -> StdResult<HandleResponse> {
    let token_owner = token_owner_get(&deps.storage, *token_id);
    if token_owner.is_none() {
        return Err(StdError::generic_err(
            "invalid token: token with the given identifier is not minted",
        ));
    }

    if token_owner.unwrap() != env.message.sender {
        return Err(StdError::generic_err(
            "invalid request: sender is not the owner of the token",
        ));
    }

    let to_c = deps.api.canonical_address(to)?;
    token_approvals_set(&mut deps.storage, *token_id, Some(to_c))?;

    let res = HandleResponse {
        messages: vec![],
        log: vec![
            log("action", "Approval"),
            log("from", deps.api.human_address(&env.message.sender)?),
            log("to", to),
            log("token_id", token_id),
        ],
        data: None,
    };

    Ok(res)
}

fn try_set_approval_for_all<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    operator: &HumanAddr,
    token_id: &Uint128,
) -> StdResult<HandleResponse> {
    let operator_c = deps.api.canonical_address(operator)?;
    let token_owner = token_owner_get(&deps.storage, *token_id);
    if token_owner.is_none() {
        return Err(StdError::generic_err(
            "invalid token: token with the given identifier is not minted",
        ));
    }

    if token_owner.clone().unwrap() != env.message.sender {
        return Err(StdError::generic_err(
            "invalid request: sender is not the owner of the token",
        ));
    }
    token_approvals_set(&mut deps.storage, *token_id, Some(operator_c.clone()))?;
    operator_approvals_set(&mut deps.storage, &token_owner.unwrap(), Some(operator_c))?;

    let res = HandleResponse {
        messages: vec![],
        log: vec![
            log("action", "ApprovalForAll"),
            log("from", deps.api.human_address(&env.message.sender)?),
            log("to", "all"),
            log("token_id", token_id),
        ],
        data: None,
    };

    Ok(res)
}

fn try_transfer_from<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    from: &HumanAddr,
    to: &HumanAddr,
    token_id: &Uint128,
) -> StdResult<HandleResponse> {
    let from_c = deps.api.canonical_address(&from)?;
    let to_c = deps.api.canonical_address(&to)?;
    let token_owner = token_owner_get(&deps.storage, *token_id);
    let approver = token_approvals_get(&deps.storage, *token_id);
    let operator = operator_approvals_get(&deps.storage, &env.message.sender);

    // Check ownership of the token
    if token_owner.is_none() {
        return Err(StdError::generic_err(
            "invalid token: token with the given identifier is not minted",
        ));
    }

    // Check whether sender is owner,approver, or operator 
    if token_owner.clone().unwrap() != env.message.sender || approver.unwrap() != env.message.sender  || operator.unwrap() != env.message.sender 
    {
        return Err(StdError::generic_err(
            "invalid request: sender is not the owner nor approver of the token",
        ));
    }

    // get list of tokens for both
    let from_tokens = holder_tokens_get(&deps.storage, &from_c);
    let to_tokens = holder_tokens_get(&deps.storage, &to_c);

    // transfer asset
    // 1) remove asset from sender
    let mut removed: Vec<Uint128> = from_tokens.clone().unwrap();
    removed.retain(|&i| i != *token_id);
    holder_tokens_set(&mut deps.storage, &from_c, Some(removed))?;

    // 2) add asset to recipient
    let mut added: Vec<Uint128> = to_tokens.clone().unwrap();
    added.push(*token_id);
    holder_tokens_set(&mut deps.storage, &to_c, Some(added))?;

    // 3) set token owner
    token_owner_set(&mut deps.storage, *token_id, Some(to_c))?;

    let res = HandleResponse {
        messages: vec![],
        log: vec![
            log("action", "Transferfrom"),
            log("from", deps.api.human_address(&env.message.sender)?),
            log("to", to),
            log("token_id", *token_id),
        ],
        data: None,
    };

    Ok(res)
}

fn try_burn<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    token_id: &Uint128,
) -> StdResult<HandleResponse> {
    let sender_h = deps.api.human_address(&env.message.sender)?;
    let res = try_transfer_from(
        deps,
        env,
        &sender_h,
        &HumanAddr::from("0".to_string()),
        token_id,
    );
    res
}

fn try_mint<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    to: &HumanAddr,
    token_id: &Uint128,
) -> StdResult<HandleResponse> {
    let to_c = deps.api.canonical_address(&to)?;
    let owner = config_get(&deps.storage).unwrap().owner;

    // Check sender is the token contract owner
    if owner != env.message.sender {
        return Err(StdError::generic_err(
            "invalid request: sender is not the owner of the token contract",
        ));
    }

    // mint asset to recipient
    let to_tokens = holder_tokens_get(&deps.storage, &to_c);
    let mut added: Vec<Uint128> = to_tokens.clone().unwrap();
    added.push(*token_id);
    holder_tokens_set(&mut deps.storage, &to_c, Some(added))?;

    // set token owner
    token_owner_set(&mut deps.storage, *token_id, Some(to_c))?;

    let res = HandleResponse {
        messages: vec![],
        log: vec![
            log("action", "Transfer"),
            log("from", HumanAddr::from("0")),
            log("to", to),
            log("token_id", token_id),
        ],
        data: None,
    };

    Ok(res)
}

pub fn try_set_token_uri<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    token_id: &Uint128,
    uri: &String,
) -> StdResult<HandleResponse> {
    let token_owner = token_owner_get(&deps.storage, *token_id);
    if token_owner.is_none() {
        return Err(StdError::generic_err(
            "invalid token: token with the given identifier is not minted",
        ));
    }

    if token_owner.unwrap() != env.message.sender {
        return Err(StdError::generic_err(
            "invalid request: sender is not the owner of the token",
        ));
    }

    token_uri_set(&mut deps.storage, *token_id, uri)?;

    let res = HandleResponse {
        messages: vec![],
        log: vec![
            log("action", "SetTokenURI"),
            log("token_id", token_id),
            log("token_id", uri),
        ],
        data: None,
    };

    Ok(res)
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::BalanceOf { address } => {
            let address_key = deps.api.canonical_address(&address)?;
            let balance = holder_tokens_get(&deps.storage, &address_key).unwrap();
            let out = to_binary(&BalanceOfResponse {
                balance: Uint128(balance.len() as u128),
            })?;
            Ok(out)
        }
        QueryMsg::OwnerOf { token_id } => {
            let owner = token_owner_get(&deps.storage, token_id).unwrap();
            let owner_h = deps.api.human_address(&owner)?;
            let out = to_binary(&OwnerOfResponse { owner: owner_h })?;
            Ok(out)
        }
        QueryMsg::Approved { token_id } => {
            let approved = token_approvals_get(&deps.storage, token_id).unwrap();
            let approved_h = deps.api.human_address(&approved)?;
            let out = to_binary(&ApprovedResponse {
                approved: approved_h,
            })?;
            Ok(out)
        }
        QueryMsg::TokenURI { token_id } => {
            let uri = token_uri_get(&deps.storage, token_id).unwrap();
            let out = to_binary(&TokenURIResponse { uri })?;
            Ok(out)
        }
        QueryMsg::ApprovedForAll { owner, operator } => {
            let owner_c = deps.api.canonical_address(&owner)?;
            let operating = operator_approvals_get(&deps.storage, &owner_c);
            let approved_for_all = match operating {
                Some(address) => true,
                None => false,
            };
            let out = to_binary(&ApprovedForAllResponse { approved_for_all })?;
            Ok(out)
        }
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
