use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Coin, Decimal, HumanAddr, Uint128, CanonicalAddr, Binary, to_binary, WasmMsg, StdResult, CosmosMsg};

use terra_bindings::TerraQueryWrapper;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub offer: String,
    pub ask: String,
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
    /// Config returns the stored configuration state. Returns State
    Config {},
    /// Exchange rate returns how many ASK we can get for 1 OFFER
    ExchangeRate {},
    /// Simulate will try to sell the given number of tokens (denom must be either ask or offer, we trade for the other)
    Simulate { offer: Coin },
    /// Reflect is used for developer integration tests on the go layer.
    /// This will cause the contract to make this query (which goes to the SDK), then return the result
    /// to the user. This can be used to test the query handlers full-stack in Go code.
    ///
    /// There are many possible return values here, this will just return the raw bytes, the caller
    /// is required to know the proper response type (defined in terra_bindings)
    Reflect { query: TerraQueryWrapper },
}

/// Returns rate of ASK/OFFER
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ExchangeRateResponse {
    pub rate: Decimal,
    pub ask: String,
    pub offer: String,
}

/// Returns how many coins we could BUY if we SELL the given amount
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SimulateResponse {
    pub sell: Coin,
    pub buy: Coin,
}

/// Human readable state
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub offer: String,
    pub ask: String,
    pub owner: HumanAddr,
}


/// Cw20TransferMsg should be de/serialized under `Receive()` variant in a HandleMsg
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Cw20TransferMsg {
    recipient: HumanAddr,
    amount: Uint128,
}

impl Cw20TransferMsg {
    /// serializes the message
    pub fn into_binary(self) -> StdResult<Binary> {
        let msg = ReceiverHandleMsg::Transfer(self);
        to_binary(&msg)
    }

    /// creates a cosmos_msg sending this struct to the named contract
    pub fn into_cosmos_msg(self, contract_addr: HumanAddr) -> StdResult<CosmosMsg> {
        let msg = self.into_binary()?;
        let execute = WasmMsg::Execute {
            contract_addr,
            msg,
            send: vec![],
        };
        Ok(execute.into())
    }
}

/// Cw20TransferMsg should be de/serialized under `Transfer()` variant in a HandleMsg
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Cw20TransferFromMsg {
    pub(crate) owner: HumanAddr,
    pub(crate) recipient: HumanAddr,
    pub(crate) amount: Uint128,
}

impl Cw20TransferFromMsg {
    /// serializes the message
    pub fn into_binary(self) -> StdResult<Binary> {
        let msg = ReceiverHandleMsg::TransferFrom(self);
        to_binary(&msg)
    }

    /// creates a cosmos_msg sending this struct to the named contract
    pub fn into_cosmos_msg(self, contract_addr: HumanAddr) -> StdResult<CosmosMsg> {
        let msg = self.into_binary()?;
        let execute = WasmMsg::Execute {
            contract_addr,
            msg,
            send: vec![],
        };
        Ok(execute.into())
    }
}


// This is just a helper to properly serialize the above message
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
enum ReceiverHandleMsg {
    Transfer(Cw20TransferMsg),
    TransferFrom(Cw20TransferFromMsg)
}


