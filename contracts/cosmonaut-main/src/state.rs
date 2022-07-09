use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::msg::ContractInitInfo;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FreightContractInfo {
    pub address: String,
    pub code_id: u64,
    pub denom: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub spaceship_cw721_contract: Addr,
    pub money_cw20_contract: Addr,
    pub freight_contracts: Vec<FreightContractInfo>,
}

pub const CONFIG: Item<Config> = Item::new("config");
