use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Sender and recipient are same")]
    SameAddress {},

    #[error("Not enough balance: {amount} > {current_balance}")]
    NotEnoughBalance { current_balance: u128, amount: u128 },

    #[error("Zero amount is invalid")]
    InvalidZeroAmount {},

    #[error("Minting cannot exceed the cap")]
    CannotExceedCap {},

    #[error("Allowance is expired")]
    Expired {},
}
