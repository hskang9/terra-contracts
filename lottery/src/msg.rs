use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, HumanAddr, Uint128};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub name: String,
    pub symbol: String,
    pub owner: HumanAddr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    Approve {
        to: HumanAddr,
        token_id: Uint128,
    },
    TransferFrom {
        from: HumanAddr,
        to: HumanAddr,
        token_id: Uint128,
    },
    Mint {
        to: HumanAddr,
        token_id: Uint128,
    },
    Burn {
        token_id: Uint128,
    },
    SetApprovalForAll {
        operator: HumanAddr,
        token_id: Uint128,
    },
    SetTokenURI {
        token_id: Uint128,
        uri: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Balance of NFT for the address
    BalanceOf { address: HumanAddr },
    /// Owner of NFT asset
    OwnerOf { token_id: Uint128 },
    /// Approved address to transfer asset
    Approved { token_id: Uint128 },
    /// Approved for all
    ApprovedForAll {
        owner: HumanAddr,
        operator: HumanAddr,
    },
    /// token URI
    TokenURI { token_id: Uint128 },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct BalanceOfResponse {
    pub balance: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct OwnerOfResponse {
    pub owner: HumanAddr,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct ApprovedResponse {
    pub approved: HumanAddr,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct ApprovedForAllResponse {
    pub approved_for_all: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct TokenURIResponse {
    pub uri: String,
}
