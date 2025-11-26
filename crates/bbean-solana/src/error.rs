use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProgramError {
    #[error("invalid instruction data")]
    InvalidInstruction,

    #[error("account not initialized")]
    AccountNotInitialized,

    #[error("account already initialized")]
    AccountAlreadyInitialized,

    #[error("insufficient funds: required {required}, available {available}")]
    InsufficientFunds { required: u64, available: u64 },