//! Error types for the BBEAN compute engine.
//!
//! All engine operations return `Result<T, EngineError>`.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("engine is already running")]
    AlreadyRunning,

    #[error("engine is not running")]
    NotRunning,

    #[error("task too large: {size} bytes (max {max})")]
    TaskTooLarge { size: usize, max: usize },

    #[error("invalid task: {0}")]
    InvalidTask(String),

    #[error("task not found: {0}")]
    TaskNotFound(String),

    #[error("node not found: {0}")]
    NodeNotFound(String),

    #[error("proof validation failed: {0}")]
    ProofInvalid(String),

    #[error("scheduler error: {0}")]
    SchedulerError(String),

    #[error("network error: {0}")]
    NetworkError(String),

    #[error("capacity exceeded: {current}/{max}")]
    CapacityExceeded { current: usize, max: usize },

    #[error("timeout after {0:?}")]
    Timeout(std::time::Duration),

    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("solana rpc error: {0}")]
    SolanaRpc(String),

    #[error("node disconnected: {0}")]
    NodeDisconnected(String),

    #[error("duplicate task id: {0}")]
    DuplicateTaskId(String),
}

pub type Result<T> = std::result::Result<T, EngineError>;
