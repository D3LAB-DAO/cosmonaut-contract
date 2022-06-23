use cosmwasm_std::Binary;
use cw721_base::ExecuteMsg as Cw721ExecuteMsg;
use cw721_base::MintMsg;
use cw_utils::Expiration;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg<T> {
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

    Mint(MintMsg<T>),

    Burn {
        token_id: String,
    },

    SetMinter {
        minter: String,
    },

    LoadFreight {
        token_id: String,
        denom: String,
        amount: u128,
        unit_weight: u128,
    },

    UnloadFreight {
        token_id: String,
        denom: String,
        amount: u128,
    },

    DecreaseHealth {
        token_id: String,
        value: u128,
    },
}

impl<T> From<ExecuteMsg<T>> for Cw721ExecuteMsg<T> {
    fn from(msg: ExecuteMsg<T>) -> Self {
        match msg {
            ExecuteMsg::TransferNft {
                recipient,
                token_id,
            } => Cw721ExecuteMsg::TransferNft {
                recipient,
                token_id,
            },
            ExecuteMsg::Mint(mint_msg) => Cw721ExecuteMsg::Mint(mint_msg),
            ExecuteMsg::SendNft {
                contract,
                token_id,
                msg,
            } => Cw721ExecuteMsg::SendNft {
                contract,
                token_id,
                msg,
            },
            ExecuteMsg::Approve {
                spender,
                token_id,
                expires,
            } => Cw721ExecuteMsg::Approve {
                spender,
                token_id,
                expires,
            },
            ExecuteMsg::Revoke { spender, token_id } => {
                Cw721ExecuteMsg::Revoke { spender, token_id }
            }
            ExecuteMsg::Burn { token_id } => Cw721ExecuteMsg::Burn { token_id },
            ExecuteMsg::ApproveAll { operator, expires } => {
                Cw721ExecuteMsg::ApproveAll { operator, expires }
            }
            ExecuteMsg::RevokeAll { operator } => Cw721ExecuteMsg::RevokeAll { operator },
            _ => panic!("cannot convert msg to Cw721ExecuteMsg")
        }
    }
}
