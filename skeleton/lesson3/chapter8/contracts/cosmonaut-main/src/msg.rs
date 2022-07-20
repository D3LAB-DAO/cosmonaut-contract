use crate::state::Config;
use cosmonaut_cw721::state::Extension;
use cosmwasm_std::{Addr, Uint128};
use cw721_base::MintMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub money_cw20_id: u64,
    pub fuel_cw20_id: u64,
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
    BuyFuelToken {
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
    FuelUp {
        token_id: String,
        amount: Uint128,
    },
    BurnFuel {
        token_id: String,
        amount: Uint128,
    },
    PlayGame {
        token_id: String,
        epoch: Uint128,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    MoneyBalance { address: String },
    OwnerOfSpaceShip { token_id: String },
    FreightTokenBalance { symbol: String, address: String },
    FuelBalance { address: String },
    SpaceShipInfo { token_id: String },
    Config {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub config: Config,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FreightTokenBalanceResponse {
    pub symbol: String,
    pub balance: Uint128,
}
