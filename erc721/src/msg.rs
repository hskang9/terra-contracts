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
        to: HumanAddr,
        token_id: Uint128,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Pair { token_id: Uint128 },
    Reserve { token_id: Uint128 },
    Config {},
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct TokenOwnerResponse {
    pub token_address: HumanAddr,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct HolderTokensResponse {
    pub token_id: Vec<Uint128>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct TokenApprovalsResponse {
    pub token_address: HumanAddr,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub name: String,
    pub symbol: String,
}
