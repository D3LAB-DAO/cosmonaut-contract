use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Empty, Uint128};
use cw721_base::state::Cw721Contract;
use cw_storage_plus::Item;

pub type Extension = Option<Metadata>;
pub type CosmonautContract<'a> = Cw721Contract<'a, Extension, Empty>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Freight {
    pub denom: String,
    pub amount: Uint128,
    pub unit_weight: u128,
}

// custom metadata for cw721 extension
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Metadata {
    pub unit_denom: String,
    pub price: u128,
    pub name: Option<String>,
    pub freight: Vec<Freight>,
    pub health: u128,
}

pub const STATE: Item<State> = Item::new("state");
