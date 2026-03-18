pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

pub use error::ProgramError;
pub use instruction::BbeanInstruction;

pub const PROGRAM_ID: &str = "BbEaN1111111111111111111111111111111111111";
pub const REWARD_DECIMALS: u8 = 9;
pub const MIN_STAKE_AMOUNT: u64 = 1_000_000_000;
pub const REWARD_PER_TASK: u64 = 100_000;
pub const BURN_RATE_BPS: u16 = 300;
