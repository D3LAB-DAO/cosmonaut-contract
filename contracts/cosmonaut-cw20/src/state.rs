use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::msg::{AllowanceResponse, MinterResponse};
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: Option<Uint128>,
    pub mint: Option<MinterResponse>,
}

impl TokenInfo {
    pub fn get_cap(&self) -> Option<Uint128> {
        self.mint.as_ref().and_then(|data| data.cap)
    }
}

pub const TOKEN_INFO: Item<TokenInfo> = Item::new("token_info");
pub const BALANCES: Map<&Addr, Uint128> = Map::new("balance");
pub const ALLOWANCES: Map<(&Addr, &Addr), AllowanceResponse> = Map::new("allowance");