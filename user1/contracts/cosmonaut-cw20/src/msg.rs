use crate::contract::TokenExtension;
use cosmwasm_std::{Binary, StdError, Uint128};
use cw20::{Cw20Coin, Logo, MinterResponse};
use cw20_base::msg::{InstantiateMarketingInfo, QueryMsg as Cw20QueryMsg};
use cw_utils::Expiration;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub initial_balances: Vec<Cw20Coin>,
    pub mint: Option<MinterResponse>,
    pub marketing: Option<InstantiateMarketingInfo>,
    pub token_extension: Option<TokenExtension>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Transfer {
        recipient: String,
        amount: Uint128,
    },
    Burn {
        amount: Uint128,
    },
    Send {
        contract: String,
        amount: Uint128,
        msg: Binary,
    },
    IncreaseAllowance {
        spender: String,
        amount: Uint128,
        expires: Option<Expiration>,
    },
    DecreaseAllowance {
        spender: String,
        amount: Uint128,
        expires: Option<Expiration>,
    },
    TransferFrom {
        owner: String,
        recipient: String,
        amount: Uint128,
    },
    SendFrom {
        owner: String,
        contract: String,
        amount: Uint128,
        msg: Binary,
    },
    BurnFrom {
        owner: String,
        amount: Uint128,
    },
    Mint {
        recipient: String,
        amount: Uint128,
    },
    UpdateMarketing {
        project: Option<String>,
        description: Option<String>,
        marketing: Option<String>,
    },
    UploadLogo(Logo),
    UpdateMinter {
        new_minter: String,
    },
    SetTokenExtension {
        unit_weight: Uint128,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Balance {
        address: String,
    },
    TokenInfo {},
    Minter {},
    Allowance {
        owner: String,
        spender: String,
    },
    AllAllowances {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    AllAccounts {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    MarketingInfo {},
    DownloadLogo {},
    TokenExtension {},
}

impl TryFrom<QueryMsg> for Cw20QueryMsg {
    type Error = StdError;

    fn try_from(msg: QueryMsg) -> Result<Self, Self::Error> {
        match msg {
            QueryMsg::Balance { address } => Ok(Cw20QueryMsg::Balance { address }),
            QueryMsg::TokenInfo {} => Ok(Cw20QueryMsg::TokenInfo {}),
            QueryMsg::Minter {} => Ok(Cw20QueryMsg::Minter {}),
            QueryMsg::Allowance { owner, spender } => {
                Ok(Cw20QueryMsg::Allowance { owner, spender })
            }
            QueryMsg::AllAllowances {
                owner,
                start_after,
                limit,
            } => Ok(Cw20QueryMsg::AllAllowances {
                owner,
                start_after,
                limit,
            }),
            QueryMsg::AllAccounts { start_after, limit } => {
                Ok(Cw20QueryMsg::AllAccounts { start_after, limit })
            }
            QueryMsg::MarketingInfo {} => Ok(Cw20QueryMsg::MarketingInfo {}),
            QueryMsg::DownloadLogo {} => Ok(Cw20QueryMsg::DownloadLogo {}),
            _ => Err(StdError::not_found("message not found")),
        }
    }
}
