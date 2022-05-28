use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::Item;
use crate::msg::ContractInitInfo;


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub spaceship_cw721_contract: ContractInitInfo,
    pub money_cw20_contract: ContractInitInfo,
    pub luggage_contracts: Option<Vec<ContractInitInfo>>,
}

pub const CONFIG: Item<Config> = Item::new("config");
