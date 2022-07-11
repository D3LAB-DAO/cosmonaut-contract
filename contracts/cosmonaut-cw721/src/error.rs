use cosmwasm_std::StdError;
use cw721_base::ContractError as Cw721ContractError;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Unimplemented")]
    Unimplemented {},

    #[error("token_id already claimed")]
    Claimed {},

    #[error("Expired")]
    Expired {},

    #[error("NotFound")]
    NotFound {},

    #[error("NotFound")]
    FuelNotEnough {},

    #[error("SameAddress")]
    SameAddress {},

    #[error("Approval not found for: {spender}")]
    ApprovalNotFound { spender: String },
}

impl TryFrom<Cw721ContractError> for ContractError {
    type Error = ContractError;

    fn try_from(err: Cw721ContractError) -> Result<Self, Self::Error> {
        match err {
            Cw721ContractError::Unauthorized {} => Ok(ContractError::Unauthorized {}),
            Cw721ContractError::Claimed {} => Ok(ContractError::Claimed {}),
            Cw721ContractError::Expired {} => Ok(ContractError::Expired {}),
            Cw721ContractError::ApprovalNotFound { spender } => {
                Ok(ContractError::ApprovalNotFound { spender })
            }
            _ => Err(ContractError::Unimplemented {}),
        }
    }
}
