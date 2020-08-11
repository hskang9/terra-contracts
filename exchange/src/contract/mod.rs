/// Handler functions module
pub mod exchange;
pub mod factory;

use crate::msg::{ConfigResponse, HandleMsg, InitMsg, PairResponse, QueryMsg, ReserveResponse};
use crate::state::exchange::{config, config_get, pair_get, reserve_get, Config};
use cosmwasm_std::{
    to_binary, Api, Binary, Empty, Env, Extern, HandleResponse, HumanAddr, InitResponse, Querier,
    StdResult, Storage, Uint128,
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
        } => exchange::try_add_liquidity(
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
        } => exchange::try_swap_to_luna(deps, env, &amount, &channel_id, &recipient),
        HandleMsg::SwapLunaToToken {
            amount,
            channel_id,
            recipient,
        } => exchange::try_swap_to_token(deps, env, &amount, &channel_id, &recipient),
        HandleMsg::WithdrawLiquidity { channel_id } => {
            exchange::try_withdraw_liquidity(deps, env, &channel_id)
        }
        HandleMsg::SwapToTokenOutput {
            tokens_bought,
            max_luna,
            deadline,
            channel_id,
            recipient,
        } => exchange::try_swap_to_token_output(
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
        } => exchange::try_swap_to_luna_output(
            deps,
            env,
            &luna_bought,
            &max_tokens,
            &deadline,
            &channel_id,
            &recipient,
        ),
        HandleMsg::WithdrawFee { channel_id } => exchange::try_withdraw_fee(deps, env, &channel_id),
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::exchange::{
        config, config_get, pair_get, pair_set, reserve_get, reserve_set, Config,
    };
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
