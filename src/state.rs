use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

use crate::msg::ContractInitInfo;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub money_cw20_contract: ContractInitInfo,
    pub spaceship_cw721_contract: ContractInitInfo,
}

pub const CONFIG: Item<Config> = Item::new("config");
