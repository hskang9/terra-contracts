use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, StdError, StdResult, Storage, Uint128};
use cosmwasm_storage::{singleton, Bucket, ReadonlyBucket, ReadonlySingleton, Singleton};

pub static CONFIG_KEY: &[u8] = b"config";
pub static TOKEN_OWNER_KEY: &[u8] = b"token_owner";
pub static HOLDER_TOKENS_KEY: &[u8] = b"holder_tokens";
pub static TOKEN_APPROVALS_KEY: &[u8] = b"token_approvals";
pub static TOKEN_URI_KEY: &[u8] = b"token_uri";
pub static OPERATOR_APPROVALS_KEY: &[u8] = b"operator_approvals";

/// Config struct
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    /// token contract address
    pub token: CanonicalAddr,
    /// owner
    pub owner: CanonicalAddr,
}

/// Config singleton initialization
pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, Config> {
    singleton(storage, CONFIG_KEY)
}

/// Get config
pub fn config_get<S: Storage>(storage: &S) -> StdResult<Config> {
    ReadonlySingleton::new(storage, CONFIG_KEY).load()
}

/// Set config
pub fn config_set<S: Storage>(storage: &mut S, config: &Config) -> StdResult<()> {
    Singleton::new(storage, CONFIG_KEY).save(config)
}

/// Get reserve between LUNA and token
/// token_id: token id in bip standard
pub fn reserve_get<S: Storage>(storage: &S, token_id: Uint128) -> Option<(Uint128, Uint128)> {
    let serialized = token_id.u128().to_le_bytes();
    match ReadonlyBucket::new(RESERVE_KEY, storage).may_load(&serialized) {
        Ok(Some(reserves)) => Some(reserves),
        _ => None,
    }
}

/// set reserve between LUNA and token
/// token_id: token id registered in bip standard
/// returns reserves: reserve in uniswapv1 contract (LUNA, Token)
pub fn reserve_set<S: Storage>(
    storage: &mut S,
    token_id: Uint128,
    reserves: (Uint128, Uint128)
) -> StdResult<()> {
    let serialized = token_id.u128().to_le_bytes();
    match Bucket::new(RESERVE_KEY, storage).save(&serialized, &reserves) {
        Ok(_) => Ok(()),
        Err(_) => Err(StdError::generic_err(format!("Failed to write to the state. key: {:?}, value: {:?}", serialized, reserves)))
    }
}