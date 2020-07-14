use std::cmp::min;

use cosmwasm_std::{
    log, to_binary, to_vec, Api, BankMsg, Binary, CanonicalAddr, Coin, CosmosMsg, Empty, Env,
    Extern, HandleResponse, HumanAddr, InitResponse, Querier, QueryRequest, StdError, StdResult,
    Storage, Uint128, WasmMsg,
};

use crate::msg::{
    ConfigResponse, HandleMsg, HolderTokensResponse, InitMsg, QueryMsg, TokenApprovalsResponse,
    TokenOwnerResponse,
};
use crate::state::{
    config, config_get, holder_tokens_get, holder_tokens_set, token_approvals_get,
    token_approvals_set, token_owner_get, token_owner_set, token_uri_get, token_uri_set, Config,
};

//pub static ALL_ADDRESS: HumanAddr = HumanAddr::from("ALL");

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
        HandleMsg::SetApprovalForAll { to, token_id } => {
            try_set_approval_for_all(deps, env, &token_id)
        }
        HandleMsg::Burn { token_id } => try_burn(deps, env, &token_id),
        HandleMsg::Mint { to, token_id } => try_mint(deps, env, &to, &token_id),
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
    token_approvals_set(&mut deps.storage, *token_id, Some(to_c));

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
    let to_c = deps.api.canonical_address(&HumanAddr::from("ALL"))?;
    token_approvals_set(&mut deps.storage, *token_id, Some(to_c));

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
    let token_owner = token_owner_get(&deps.storage, *token_id);
    let approver = token_approvals_get(&deps.storage, *token_id);
    if token_owner.is_none() {
        return Err(StdError::generic_err(
            "invalid token: token with the given identifier is not minted",
        ));
    }

    if token_owner.clone().unwrap() != env.message.sender && approver.unwrap() != env.message.sender
    {
        return Err(StdError::generic_err(
            "invalid request: sender is not the owner nor approver of the token",
        ));
    }

    let holder_tokens = holder_tokens_get(&deps.storage, &token_owner.unwrap());

    if holder_tokens.is_none() {
        return Err(StdError::generic_err(
            "invalid ownership: sender does not own the token",
        ));
    }

    let mut new: Vec<Uint128> = holder_tokens.clone().unwrap();
    new.retain(|&i| i != *token_id);
    holder_tokens_set(&mut deps.storage, &from_c, Some(new));

    let res = HandleResponse {
        messages: vec![],
        log: vec![
            log("action", "Transferfrom"),
            log("from", deps.api.human_address(&env.message.sender)?),
            log("to", "all"),
            log("token_id", token_id),
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
    let res = HandleResponse {
        messages: vec![],
        log: vec![
            log("action", "Transfer"),
            log("from", deps.api.human_address(&env.message.sender)?),
            log("to", HumanAddr::from("0")),
            log("token_id", token_id),
        ],
        data: None,
    };

    Ok(res)
}

fn try_mint<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    to: &HumanAddr,
    token_id: &Uint128,
) -> StdResult<HandleResponse> {
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
/*
fn before_token_transfer<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    from: &HumanAddr,
    to: &HumanAddr,
    token_id: &Uint128,
) -> StdResult<()> {
    match ((*from == ZERO_ADDRESS) as i32, (*to == ZERO_ADDRESS) as i32) {
        (0, 0) => _transfer,
        (0, 1) => _mint,
        (1, 0) => _burn,
        (1, 1) => return Err(StdError::generic_err("Internal Error: both `from` and `to` cannot be the zero address."))
    }
}

*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{config, config_get, pair_get, pair_set, reserve_get, reserve_set, Config};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, MOCK_CONTRACT_ADDR};
    use cosmwasm_std::{coin, coins, CanonicalAddr, CosmosMsg, StdError, Uint128};

    const CANONICAL_LENGTH: usize = 20;

    // State tests
    /// Test input price calculation
    /// After execution, result will be:
    /// TODO: add example calculation
    fn get_input_price_works() {
        unimplemented!();
    }

    /// Test output price calculation
    /// After execution, result will be:
    /// TODO: add example calculation
    fn get_output_price_works() {
        unimplemented!();
    }

    /// Test adding liquidity
    /// After execution, result will be:
    /// sender depositing Luna and token to the exchange contract
    /// exchange contract getting token and luna bigger than minimum amount
    fn try_add_liquidity_works() {
        unimplemented!();
    }

    /// Test token to Luna swap
    /// After execution, result will be:
    /// sender having luna based on output price
    /// exchange contract getting token and giving luna to the sender
    fn try_swap_to_luna_works() {
        unimplemented!();
    }
}
