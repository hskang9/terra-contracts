use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, HumanAddr, Uint128};
use crate::state::Config;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub token: HumanAddr,
    pub owner: HumanAddr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
   Buy {
       amount: Uint128
   },
   ChangeOwnership {
       new: HumanAddr
   },
   RevokeOwnership {}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Order info for the given address
    OrderOf { address: HumanAddr },
    /// Configuration
    Config {},
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct OrderResponse {
    pub balance: Uint128,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub config: Config,
}
