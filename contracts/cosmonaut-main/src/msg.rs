use cosmwasm_std::Addr;
use cw721_base::MintMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
pub enum ExecuteMsg<T> {
    BuyMoneyToken {
        amount: u128,
    },
    BuyNft {
        original_owner: String,
        nft_id: String,
    },
    Mint(MintMsg<T>),
    SetMinter {
        minter: String,
    },
    BuyFreightToken {
        denom: String,
        amount: u128,
    },
    AddFreightContract {
        address: String,
        denom: String,
    },
    LoadFreight {
        token_id: String,
        denom: String,
        amount: u128,
        unit_weight: u128,
    },
    UnLoadFreight {
        token_id: String,
        denom: String,
        amount: u128,
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
