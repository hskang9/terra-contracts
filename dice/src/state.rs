use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, StdError, StdResult, Storage, Uint128};
use cosmwasm_storage::{singleton, Bucket, ReadonlyBucket, ReadonlySingleton, Singleton};

pub static CONFIG_KEY: &[u8] = b"config";
pub static ORDER_KEY: &[u8] = b"order";

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

/// Get order
pub fn order_get<S: Storage>(storage: &S, token_id: Uint128) -> Option<(Uint128, Uint128)> {
    let serialized = token_id.u128().to_le_bytes();
    match ReadonlyBucket::new(ORDER_KEY, storage).may_load(&serialized) {
        Ok(Some(wrapped_order)) => Some(wrapped_order),
        _ => None,
    }
}

/// Set order
pub fn order_set<S: Storage>(
    storage: &mut S,
    buyer: CanonicalAddr,
    amount: Uint128
) -> StdResult<()> {
    match Bucket::new(ORDER_KEY, storage).save(&buyer.as_slice(), &amount) {
        Ok(_) => Ok(()),
        Err(_) => Err(StdError::generic_err(format!("Failed to write to the state. key: {:?}, value: {:?}", buyer, amount)))
    }
}