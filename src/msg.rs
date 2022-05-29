use cosmwasm_std::Addr;
use cw721_base::MintMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub money_cw20_contract: ContractInitInfo,
    pub spaceship_cw721_contract: ContractInitInfo,
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
    BuyLuggageToken {
        denom: String,
        amount: u128,
    },
    AddLuggageContract {
        address: String,
        denom: String,
        code_id: u64,
    },
    LoadLuggage {
        token_id: String,
        denom: String,
        amount: u128,
    },
    UnLoadLuggage {
        token_id: String,
        denom: String,
        amount: u128,
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
