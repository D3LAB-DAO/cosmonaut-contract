use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::Item;
use crate::msg::ContractInitInfo;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LuggageContractInfo {
    pub address: String,
    pub denom: String,
    pub code_id: u64
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub spaceship_cw721_contract: ContractInitInfo,
    pub money_cw20_contract: ContractInitInfo,
    pub luggage_contracts: Vec<LuggageContractInfo>,
}

pub const CONFIG: Item<Config> = Item::new("config");
