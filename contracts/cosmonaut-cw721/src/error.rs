use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Unimplemented")]
    Unimplemented {},

    #[error("Expired")]
    Expired {},

    #[error("NotFound")]
    NotFound {},

    #[error("SameAddress")]
    SameAddress {},
}
