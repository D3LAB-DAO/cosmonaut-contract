use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Item;

// TODO: q1) Declare extension as type of struct Metadata below

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Freight {
    pub denom: String,
    pub amount: Uint128,
    pub unit_weight: Uint128,
}

// custom metadata for cw721 extension
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Metadata {
    pub unit_denom: String,
    pub price: Uint128,
    pub name: Option<String>,
    // TODO: q2) freights as vector of Freight?
    pub health: Uint128,
    pub fuel: Uint128,
}

pub const STATE: Item<State> = Item::new("state");
