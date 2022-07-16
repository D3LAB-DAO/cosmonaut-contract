use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FreightContractInfo {
    pub address: String,
    pub denom: String,
    pub code_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub spaceship_cw721_contract: Addr,
    pub money_cw20_contract: Addr,
    pub fuel_cw20_contract: Addr,
    pub freight_contracts: Vec<FreightContractInfo>,
}

pub const CONFIG: Item<Config> = Item::new("config");
