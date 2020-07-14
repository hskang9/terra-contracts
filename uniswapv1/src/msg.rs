use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{HumanAddr, Uint128, CanonicalAddr};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub minimum_luna: Uint128,
    pub owner: HumanAddr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    /// AddLiquidity will deposit LUNA and Token to add liquidity between them
    AddLiquidity {
        luna_amount: Uint128,
        token_amount: Uint128,
        token_address: HumanAddr,
        token_id: Uint128
    },
    SwapTokenToLuna {
        amount: Uint128,
        token_id: Uint128,
        recipient: HumanAddr
    },
    SwapLunaToToken {
        amount: Uint128,
        token_id: Uint128,
        recipient: HumanAddr
    }
}

/// msg for calling external contract transaction
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ERC20HandleMsg {
    Approve {
        spender: HumanAddr,
        amount: Uint128,
    },
    Transfer {
        recipient: HumanAddr,
        amount: Uint128,
    },
    TransferFrom {
        owner: HumanAddr,
        recipient: HumanAddr,
        amount: Uint128,
    },
    Burn {
        amount: Uint128,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Pair { token_id: Uint128 },
    Reserve { token_id: Uint128 },
    Config{},
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct PairResponse {
    pub token_address: HumanAddr,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct ReserveResponse {
    pub reserves: (Uint128, Uint128)
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub minimum_luna: Uint128,
    pub owner: HumanAddr,
}



