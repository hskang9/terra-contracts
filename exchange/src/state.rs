use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, Storage, Uint128, StdResult, StdError};
use cosmwasm_storage::{ReadonlyBucket, Bucket, singleton, ReadonlySingleton, Singleton};

pub static CONFIG_KEY: &[u8] = b"config";
pub static PAIR_KEY: &[u8] = b"pair";
pub static RESERVE_KEY: &[u8] = b"reserve";

/// Config struct
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    /// minimum luna to deposit to provide liquidity
    pub minimum_luna: Uint128,
    /// canonical address of the dex owner
    pub owner: CanonicalAddr,
}

/// Config singleton initialization
pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, Config> {
    singleton(storage, CONFIG_KEY)
}

/// Get config
pub fn config_get<S: Storage>(storage: &S) ->  StdResult<Config> {
    ReadonlySingleton::new(storage, CONFIG_KEY).load()
}

/// Set config
pub fn config_set<S: Storage>(storage: &mut S, config: &Config) -> StdResult<()> {
    Singleton::new(storage, CONFIG_KEY).save(config)
}

/// Get pair between LUNA and token
/// channel_id: identifier for the channel between an asset and LUNA 
/// returns tuple (address: token contract address, registrar: registrar of the channel)
pub fn pair_get<S: Storage>(storage: &S, channel_id: Uint128) -> Option<(CanonicalAddr, CanonicalAddr)> {
    let serialized = channel_id.u128().to_le_bytes();
    match ReadonlyBucket::new(PAIR_KEY, storage).may_load(&serialized) {
        Ok(Some(wrapped_address)) => wrapped_address,
        _ => None,
    }
}

/// Set pair between LUNA and token
/// channel_id: identifier for the channel between an asset and LUNA 
/// tuple: (address: token contract address, registrar: registrar of the channel)
pub fn pair_set<S: Storage>(
    storage: &mut S,
    channel_id: Uint128,
    channel: Option<(CanonicalAddr, CanonicalAddr)>
) -> StdResult<()> {
    let serialized = channel_id.u128().to_le_bytes();
    match Bucket::new(PAIR_KEY, storage).save(&serialized, &channel) {
        Ok(_) => Ok(()),
        Err(_) => Err(StdError::generic_err(format!("Failed to write to the state. key: {:?}, value: {:?}", serialized, channel)))
    }
}

/// Get reserve between LUNA and token
/// token_id: token id in bip standard
pub fn reserve_get<S: Storage>(storage: &S, token_id: Uint128) -> Option<(Uint128, Uint128)> {
    let serialized = token_id.u128().to_le_bytes();
    match ReadonlyBucket::new(RESERVE_KEY, storage).may_load(&serialized) {
        Ok(Some(wrapped_reserves)) => Some(wrapped_reserves),
        _ => None,
    }
}

/// set reserve between LUNA and token
/// token_id: token id registered in bip standard
/// returns reserves: reserve in uniswapv1 contract (LUNA, Token)
pub fn reserve_set<S: Storage>(
    storage: &mut S,
    token_id: Uint128,
    reserves: Option<(Uint128, Uint128)>
) -> StdResult<()> {
    let serialized = token_id.u128().to_le_bytes();
    match Bucket::new(RESERVE_KEY, storage).save(&serialized, &reserves) {
        Ok(_) => Ok(()),
        Err(_) => Err(StdError::generic_err(format!("Failed to write to the state. key: {:?}, value: {:?}", serialized, reserves)))
    }
}


