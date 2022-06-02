use cosmwasm_std::Binary;
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
