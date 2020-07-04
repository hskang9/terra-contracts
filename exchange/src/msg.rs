use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Coin, Decimal, HumanAddr, Uint128, CanonicalAddr, Binary, to_binary, WasmMsg, StdResult, CosmosMsg};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub minimum_luna: Uint128,
    pub owner: CanonicalAddr,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Balance { address: HumanAddr },
    Config {},
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct BalanceResponse {
    pub balance: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub name: String,
    pub symbol: String,
    pub owner: HumanAddr,
}




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