use thiserror::Error;

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("engine is already running")]
    AlreadyRunning,

    #[error("engine is not running")]
    NotRunning,

    #[error("task too large: {size} bytes (max {max})")]
    TaskTooLarge { size: usize, max: usize },