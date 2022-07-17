use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("InvalidContract")]
    InvalidContract {},

    #[error("InvalidToken")]
    InvalidToken {},

    #[error("NotEnoughToken")]
    NotEnoughToken {},

    #[error("DuplicatedContract")]
    DuplicatedContract {},

    #[error("TokenNotFound")]
    TokenNotFound {},

    #[error("NotEnoughCoin")]
    NotEnoughCoin {},

    #[error("FrightOverloaded")]
    FrightOverloaded {},
}
