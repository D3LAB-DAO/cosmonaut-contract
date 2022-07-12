use std::convert::TryFrom;
use cosmwasm_std::StdError;
use cw20_base::msg::QueryMsg as Cw20QueryMsg;
use cw20_base::ContractError;
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Balance { address: String },
    TokenInfo {},
    Minter {},
    Allowance { owner: String, spender: String },
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
    TokenExtension {},w
}

impl TryFrom<QueryMsg> for Cw20QueryMsg{
    type Error = StdError;

    fn try_from(msg: QueryMsg) -> Result<Self, Self::Error> {
        match msg {
            QueryMsg::Balance { address } => Ok(Cw20QueryMsg::Balance { address }),
            QueryMsg::TokenInfo {} => Ok(Cw20QueryMsg::TokenInfo {}),
            QueryMsg::Minter {} => Ok(Cw20QueryMsg::Minter {}),
            QueryMsg::Allowance { owner, spender } => Ok(Cw20QueryMsg::Allowance { owner, spender }),
            QueryMsg::AllAllowances { owner, start_after, limit } => Ok(Cw20QueryMsg::AllAllowances { owner, start_after, limit }),
            QueryMsg::AllAccounts { start_after, limit } => Ok(Cw20QueryMsg::AllAccounts { start_after, limit }),
            QueryMsg::MarketingInfo {} => Ok(Cw20QueryMsg::MarketingInfo {}),
            QueryMsg::DownloadLogo {} => Ok(Cw20QueryMsg::DownloadLogo {}),
            _ => Err(StdError::not_found("message not found"))
        }
    }
}
