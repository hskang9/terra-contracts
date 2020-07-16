use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, StdError, StdResult, Storage, Uint128};
use cosmwasm_storage::{singleton, Bucket, ReadonlyBucket, ReadonlySingleton, Singleton};

pub static CONFIG_KEY: &[u8] = b"config";
pub static TOKEN_OWNER_KEY: &[u8] = b"token_owner";
pub static HOLDER_TOKENS_KEY: &[u8] = b"holder_tokens";
pub static TOKEN_APPROVALS_KEY: &[u8] = b"token_approvals";
pub static TOKEN_URI_KEY: &[u8] = b"token_uri";

/// Config struct
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    /// total luna supply that the contract has
    pub name: String,
    /// minimum luna to deposit to provide liquidity
    pub symbol: String,
    /// owner address
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

/// Get token owner with asset index
/// token_id: index in ERC721
pub fn token_owner_get<S: Storage>(storage: &S, token_id: Uint128) -> Option<CanonicalAddr> {
    let serialized = token_id.u128().to_le_bytes();
    match ReadonlyBucket::new(TOKEN_OWNER_KEY, storage).may_load(&serialized) {
        Ok(Some(address)) => address,
        _ => None,
    }
}

/// Set token owner of the NFT with asset index
/// token_id: token id in bip standard
/// address: token contract address
pub fn token_owner_set<S: Storage>(
    storage: &mut S,
    token_id: Uint128,
    address: Option<CanonicalAddr>,
) -> StdResult<()> {
    let serialized = token_id.u128().to_le_bytes();
    match Bucket::new(TOKEN_OWNER_KEY, storage).save(&serialized, &address) {
        Ok(_) => Ok(()),
        Err(_) => Err(StdError::generic_err(format!(
            "Failed to write to the state. key: {:?}, value: {:?}",
            serialized, address
        ))),
    }
}

/// Get holder's tokens from holder's address
/// holer: holder's address
pub fn holder_tokens_get<S: Storage>(storage: &S, holder: &CanonicalAddr) -> Option<Vec<Uint128>> {
    match ReadonlyBucket::new(HOLDER_TOKENS_KEY, storage).may_load(holder.as_slice()) {
        Ok(Some(tokens)) => tokens,
        _ => None,
    }
}

/// Set holder's token with holder's address
/// holder: holder's address
/// tokens: array of token indices that holder has
/// returns tokens: indices in the token contract
pub fn holder_tokens_set<S: Storage>(
    storage: &mut S,
    holder: &CanonicalAddr,
    tokens: Option<Vec<Uint128>>,
) -> StdResult<()> {
    match Bucket::new(HOLDER_TOKENS_KEY, storage).save(holder.as_slice(), &tokens) {
        Ok(_) => Ok(()),
        Err(_) => Err(StdError::generic_err(format!(
            "Failed to write to the state. key: {:?}, value: {:?}",
            holder, tokens
        ))),
    }
}

/// Get token approval with asset index
/// token_id: index in ERC721
pub fn token_approvals_get<S: Storage>(storage: &S, token_id: Uint128) -> Option<CanonicalAddr> {
    let serialized = token_id.u128().to_le_bytes();
    match ReadonlyBucket::new(TOKEN_APPROVALS_KEY, storage).may_load(&serialized) {
        Ok(Some(address)) => address,
        _ => None,
    }
}

/// Set token approval with asset index
/// token_id: token id in bip standard
/// address: token contract address
pub fn token_approvals_set<S: Storage>(
    storage: &mut S,
    token_id: Uint128,
    address: Option<CanonicalAddr>,
) -> StdResult<()> {
    let serialized = token_id.u128().to_le_bytes();
    match Bucket::new(TOKEN_APPROVALS_KEY, storage).save(&serialized, &address) {
        Ok(_) => Ok(()),
        Err(_) => Err(StdError::generic_err(format!(
            "Failed to write to the state. key: {:?}, value: {:?}",
            serialized, address
        ))),
    }
}

/// Get optional token URI with asset index
/// token_id: index in ERC721
pub fn token_uri_get<S: Storage>(storage: &S, token_id: Uint128) -> Option<String> {
    let serialized = token_id.u128().to_le_bytes();
    match ReadonlyBucket::new(TOKEN_URI_KEY, storage).may_load(&serialized) {
        Ok(Some(uri)) => uri,
        _ => None,
    }
}

/// Set token approval with asset index
/// token_id: token id in bip standard
/// address: token contract address
pub fn token_uri_set<S: Storage>(
    storage: &mut S,
    token_id: Uint128,
    uri: &String,
) -> StdResult<()> {
    let serialized = token_id.u128().to_le_bytes();
    match Bucket::new(TOKEN_URI_KEY, storage).save(&serialized, uri) {
        Ok(_) => Ok(()),
        Err(_) => Err(StdError::generic_err(format!(
            "Failed to write to the state. key: {:?}, value: {:?}",
            serialized, uri
        ))),
    }
}
