use crate::state::Extension;
use crate::ContractError;
use cosmwasm_std::{Binary, Uint128};
use cw721_base::ExecuteMsg as Cw721ExecuteMsg;
use cw721_base::MintMsg;
use cw_utils::Expiration;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    TransferNft {
        recipient: String,
        token_id: String,
    },
    SendNft {
        contract: String,
        token_id: String,
        msg: Binary,
    },
    Approve {
        spender: String,
        token_id: String,
        expires: Option<Expiration>,
    },
    Revoke {
        spender: String,
        token_id: String,
    },
    ApproveAll {
        operator: String,
        expires: Option<Expiration>,
    },
    RevokeAll {
        operator: String,
    },

    // TODO: q3) message "Mint" which extends MintMsg of cw721_base with Extension type declared at state.rs


    Burn {
        token_id: String,
    },

    SetMinter {
        minter: String,
    },

    LoadFreight {
        token_id: String,
        denom: String,
        amount: Uint128,
        unit_weight: Uint128,
    },

    FuelUp {
        token_id: String,
        amount: Uint128,
    },

    BurnFuel {
        token_id: String,
        amount: Uint128,
    },

    UnloadFreight {
        token_id: String,
        denom: String,
        amount: Uint128,
    },
    DecreaseHealth {
        token_id: String,
        value: Uint128,
    },
}

impl TryFrom<ExecuteMsg> for Cw721ExecuteMsg<Extension> {
    type Error = ContractError;

    fn try_from(msg: ExecuteMsg) -> Result<Self, Self::Error> {
        match msg {
            //TODO: q4) Convert ExecuteMsg::TransferNft to Cw721ExecuteMsg::TransferNft

            ExecuteMsg::Mint(mint_msg) => Ok(Cw721ExecuteMsg::Mint(mint_msg)),
            ExecuteMsg::SendNft {
                contract,
                token_id,
                msg,
            } => Ok(Cw721ExecuteMsg::SendNft {
                contract,
                token_id,
                msg,
            }),
            ExecuteMsg::Approve {
                spender,
                token_id,
                expires,
            } => Ok(Cw721ExecuteMsg::Approve {
                spender,
                token_id,
                expires,
            }),
            ExecuteMsg::Revoke { spender, token_id } => {
                Ok(Cw721ExecuteMsg::Revoke { spender, token_id })
            }
            ExecuteMsg::Burn { token_id } => Ok(Cw721ExecuteMsg::Burn { token_id }),
            ExecuteMsg::ApproveAll { operator, expires } => {
                Ok(Cw721ExecuteMsg::ApproveAll { operator, expires })
            }
            ExecuteMsg::RevokeAll { operator } => Ok(Cw721ExecuteMsg::RevokeAll { operator }),
            _ => Err(ContractError::Unimplemented {}),
        }
    }
}
