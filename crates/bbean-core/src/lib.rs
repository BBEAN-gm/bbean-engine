pub mod config;
pub mod error;
pub mod node;
pub mod proof;
pub mod runtime;
pub mod task;

pub use config::EngineConfig;
pub use error::{EngineError, Result};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DEFAULT_PORT: u16 = 9420;
pub const MAX_TASK_SIZE_BYTES: usize = 50 * 1024 * 1024;
pub const PROOF_DIFFICULTY: u8 = 16;
pub const HEARTBEAT_INTERVAL_SECS: u64 = 30;
pub const MAX_RETRIES: u32 = 3;

use node::NodeRegistry;
use proof::BrewValidator;
use task::Scheduler;