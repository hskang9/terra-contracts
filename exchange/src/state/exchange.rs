use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, StdError, StdResult, Storage, Uint128};
use cosmwasm_storage::{singleton, Bucket, ReadonlyBucket, ReadonlySingleton, Singleton};

pub static CONFIG_KEY: &[u8] = b"config";
pub static PAIR_KEY: &[u8] = b"pair";
pub static SHARE_KEY: &[u8] = b"share";
pub static TOTAL_SHARE_KEY: &[u8] = b"total_share";
pub static RESERVE_KEY: &[u8] = b"reserve";
pub static FEE_KEY: &[u8] = b"fee";

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
pub fn config_get<S: Storage>(storage: &S) -> StdResult<Config> {
    ReadonlySingleton::new(storage, CONFIG_KEY).load()
}

/// Set config
pub fn config_set<S: Storage>(storage: &mut S, config: &Config) -> StdResult<()> {
    Singleton::new(storage, CONFIG_KEY).save(config)
}

/// Get pair between LUNA and token
/// channel_id: identifier for the channel between an asset and LUNA
/// returns tuple (to_address: asset address which swaps to, from_address: address swapping from, CanonicalAddr(0) if LUNA)
pub fn pair_get<S: Storage>(
    storage: &S,
    channel_id: Uint128,
) -> Option<(CanonicalAddr, CanonicalAddr)> {
    let serialized = channel_id.u128().to_le_bytes();
    match ReadonlyBucket::new(PAIR_KEY, storage).may_load(&serialized) {
        Ok(Some(wrapped_address)) => wrapped_address,
        _ => None,
    }
}

/// Set pair between LUNA and token
/// channel_id: identifier for the channel between an asset and LUNA
/// tuple: (to_address: asset address which swaps to, from_address: address swapping from, CanonicalAddr(0) if LUNA)
pub fn pair_set<S: Storage>(
    storage: &mut S,
    channel_id: Uint128,
    channel: Option<(CanonicalAddr, CanonicalAddr)>,
) -> StdResult<()> {
    let serialized = channel_id.u128().to_le_bytes();
    match Bucket::new(PAIR_KEY, storage).save(&serialized, &channel) {
        Ok(_) => Ok(()),
        Err(_) => Err(StdError::generic_err(format!(
            "Failed to write to the state. key: {:?}, value: {:?}",
            serialized, channel
        ))),
    }
}

/// Get total_share between LUNA and token
/// channel_id: identifier for the channel between an asset and LUNA
/// returns tuple (luna total share, token total share)
pub fn total_share_get<S: Storage>(storage: &S, channel_id: Uint128) -> Option<(Uint128, Uint128)> {
    let serialized = channel_id.u128().to_le_bytes();
    match ReadonlyBucket::new(TOTAL_SHARE_KEY, storage).may_load(&serialized) {
        Ok(Some(wrapped_total_shares)) => wrapped_total_shares,
        _ => None,
    }
}

/// Set total share between LUNA and token
/// channel_id: identifier for the channel between an asset and LUNA
/// tuple: (luna total share, token total share)
pub fn total_share_set<S: Storage>(
    storage: &mut S,
    channel_id: Uint128,
    channel: Option<(Uint128, Uint128)>,
) -> StdResult<()> {
    let serialized = channel_id.u128().to_le_bytes();
    match Bucket::new(TOTAL_SHARE_KEY, storage).save(&serialized, &channel) {
        Ok(_) => Ok(()),
        Err(_) => Err(StdError::generic_err(format!(
            "Failed to write to the state. key: {:?}, value: {:?}",
            serialized, channel
        ))),
    }
}

/// Get share of liquidity between LUNA and token
/// channel_id: identifier for the channel between an asset and LUNA
/// lp: liquidity provider address)
/// returns tuple (from_liquidity: provided amount of liquidity with asset swapping from, to_liquidity: provided amount of liquidity with asset swapping to)
pub fn share_get<S: Storage>(
    storage: &S,
    channel_id: Uint128,
    lp: CanonicalAddr,
) -> Option<(Uint128, Uint128)> {
    let serialized = channel_id.u128().to_le_bytes();
    let lp_slice = lp.as_slice();
    let key = [&serialized, lp_slice].concat();
    match ReadonlyBucket::new(SHARE_KEY, storage).may_load(&key) {
        Ok(Some(wrapped_share)) => wrapped_share,
        _ => None,
    }
}

/// Set pair between LUNA and token
/// channel_id: identifier for the channel between an asset and LUNA
/// tuple: (address: token contract address, registrar: registrar of the channel)
pub fn share_set<S: Storage>(
    storage: &mut S,
    channel_id: Uint128,
    lp: CanonicalAddr,
    channel: Option<(Uint128, Uint128)>,
) -> StdResult<()> {
    let serialized = channel_id.u128().to_le_bytes();
    let lp_slice = lp.as_slice();
    let key = [&serialized, lp_slice].concat();
    match Bucket::new(SHARE_KEY, storage).save(&key, &channel) {
        Ok(_) => Ok(()),
        Err(_) => Err(StdError::generic_err(format!(
            "Failed to write to the state. key: {:?}, value: {:?}",
            serialized, channel
        ))),
    }
}

/// Get reserve of LUNA and token
/// channel_id: channel identifier in dex contract
pub fn reserve_get<S: Storage>(storage: &S, channel_id: Uint128) -> Option<(Uint128, Uint128)> {
    let serialized = channel_id.u128().to_le_bytes();
    match ReadonlyBucket::new(RESERVE_KEY, storage).may_load(&serialized) {
        Ok(Some(wrapped_reserves)) => Some(wrapped_reserves),
        _ => None,
    }
}

/// set reserve of LUNA and token
/// channel_id: channel identifier in dex contract
/// returns reserves: reserve in uniswapv1 contract (LUNA, Token)
pub fn reserve_set<S: Storage>(
    storage: &mut S,
    channel_id: Uint128,
    reserves: Option<(Uint128, Uint128)>,
) -> StdResult<()> {
    let serialized = channel_id.u128().to_le_bytes();
    match Bucket::new(RESERVE_KEY, storage).save(&serialized, &reserves) {
        Ok(_) => Ok(()),
        Err(_) => Err(StdError::generic_err(format!(
            "Failed to write to the state. key: {:?}, value: {:?}",
            serialized, reserves
        ))),
    }
}

/// Get dex contract owner fee of LUNA and token
/// channel_id: channel identifier in dex contract
pub fn fee_get<S: Storage>(storage: &S, channel_id: Uint128) -> Option<(Uint128, Uint128)> {
    let serialized = channel_id.u128().to_le_bytes();
    match ReadonlyBucket::new(FEE_KEY, storage).may_load(&serialized) {
        Ok(Some(wrapped_fees)) => Some(wrapped_fees),
        _ => None,
    }
}

/// Set dex contract owner fee of LUNA and token
/// channel_id: channel identifier in dex contract
/// returns reserves: fee in the dex contract (LUNA, Token)
pub fn fee_set<S: Storage>(
    storage: &mut S,
    channel_id: Uint128,
    fees: Option<(Uint128, Uint128)>,
) -> StdResult<()> {
    let serialized = channel_id.u128().to_le_bytes();
    match Bucket::new(FEE_KEY, storage).save(&serialized, &fees) {
        Ok(_) => Ok(()),
        Err(_) => Err(StdError::generic_err(format!(
            "Failed to write to the state. key: {:?}, value: {:?}",
            serialized, fees
        ))),
    }
}
