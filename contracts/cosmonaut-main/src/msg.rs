use cosmwasm_std::{Addr, Uint128};
use cw721_base::MintMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmonaut_cw721::state::Extension;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub money_cw20_id: u64,
    pub spaceship_cw721_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractInitInfo {
    pub addr: Option<Addr>,
    pub code_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    BuyMoneyToken {
        amount: Uint128,
    },
    BuyNft {
        original_owner: String,
        nft_id: String,
    },
    Mint(MintMsg<Extension>),
    SetMinter {
        minter: String,
    },
    BuyFreightToken {
        address: String,
        amount: Uint128,
    },
    AddFreightContract {
        address: String,
    },
    LoadFreight {
        address: String,
        token_id: String,
        amount: Uint128,
    },
    UnLoadFreight {
        address: String,
        token_id: String,
        amount: Uint128,
    },
    PlayGame {
        token_id: String,
        epoch: u64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    MoneyContract {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MoneyContractResponse {
    pub address: Addr,
}
