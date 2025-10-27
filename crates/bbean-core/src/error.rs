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