use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
    #[error("Insufficient funds")]
    InsufficientFunds {},

    #[error("Token do not exist")]
    TokenNotFound {},

    #[error("Account already exists")]
    AccountExists{},

    #[error("Account does not exist")]
    AccountDoesNotExist {},

    #[error("Insufficient collateral")]
    InsufficientCollateral {},

    #[error("No repayments needed")]
    NoRepayment {},

    #[error("Values donot match")]
    MathError {},

    #[error("Wrong token")]
    WrongToken {},
}
