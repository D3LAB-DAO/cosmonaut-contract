use cosmwasm_std::StdError;
use thiserror::Error;
use cw721_base::ContractError as Cw721ContractError;

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

    #[error("SameAddress")]
    SameAddress {},

    #[error("Approval not found for: {spender}")]
    ApprovalNotFound { spender: String },
}

impl From<Cw721ContractError> for ContractError {
    fn from(err: Cw721ContractError) -> Self {
        match err {
            Cw721ContractError::Unauthorized {} => ContractError::Unauthorized {},
            Cw721ContractError::Claimed {} => ContractError::Claimed {},
            Cw721ContractError::Expired {} => ContractError::Expired {},
            Cw721ContractError::ApprovalNotFound { spender } => ContractError::ApprovalNotFound { spender },
            _ => ContractError::Unimplemented {}
        }
    }
}
