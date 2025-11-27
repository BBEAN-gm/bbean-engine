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

    #[error("invalid authority")]
    InvalidAuthority,

    #[error("arithmetic overflow")]
    ArithmeticOverflow,

    #[error("invalid proof: {0}")]
    InvalidProof(String),

    #[error("stake amount below minimum: {0}")]
    StakeBelowMinimum(u64),

    #[error("reward pool exhausted")]
    RewardPoolExhausted,

    #[error("cooldown period active: {remaining_secs} seconds remaining")]
    CooldownActive { remaining_secs: u64 },

    #[error("serialization error: {0}")]
    Serialization(String),
}
