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

pub struct Engine {
    config: EngineConfig,
    scheduler: Scheduler,
    registry: NodeRegistry,
    validator: BrewValidator,
    running: bool,
}

impl Engine {
    pub fn new(config: EngineConfig) -> Result<Self> {
        tracing::info!("initializing bbean engine v{}", VERSION);
        let scheduler = Scheduler::new(config.scheduler.clone());
        let registry = NodeRegistry::new(config.max_nodes);
        let validator = BrewValidator::new(config.proof_difficulty);

        Ok(Self {
            config,
            scheduler,
            registry,
            validator,
            running: false,
        })
    }

    pub async fn start(&mut self) -> Result<()> {
        if self.running {
            return Err(EngineError::AlreadyRunning);
        }
        tracing::info!("starting engine on {}:{}", self.config.host, self.config.port);
        self.registry.start_discovery().await?;
        self.scheduler.start().await?;
        self.running = true;
        tracing::info!("engine is running");
        Ok(())
    }

    pub async fn stop(&mut self) -> Result<()> {
        if !self.running {
            return Ok(());
        }
        tracing::info!("stopping engine");
        self.scheduler.stop().await?;
        self.registry.disconnect_all().await?;
        self.running = false;
        Ok(())
    }

    pub async fn submit_task(&self, task: task::Task) -> Result<task::TaskReceipt> {
        if !self.running {
            return Err(EngineError::NotRunning);
        }
        let validated = self.validate_task(&task)?;
        let receipt = self.scheduler.enqueue(validated).await?;
        tracing::debug!(task_id = %receipt.id, "task submitted");
        Ok(receipt)
    }

    pub async fn get_task_status(&self, task_id: &str) -> Result<task::TaskStatus> {
        self.scheduler.get_status(task_id).await
    }

    pub async fn get_node_count(&self) -> usize {
        self.registry.active_count().await
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    fn validate_task(&self, task: &task::Task) -> Result<task::ValidatedTask> {
        if task.payload.is_empty() {
            return Err(EngineError::InvalidTask("payload cannot be empty".into()));
        }
        if task.payload.len() > MAX_TASK_SIZE_BYTES {
            return Err(EngineError::TaskTooLarge {
                size: task.payload.len(),
                max: MAX_TASK_SIZE_BYTES,
            });
        }
        if task.model_id.is_empty() {
            return Err(EngineError::InvalidTask("model_id is required".into()));
        }
        let priority = task.priority.unwrap_or(task::TaskPriority::Normal);
        Ok(task::ValidatedTask {
            inner: task.clone(),
            priority,
            validated_at: chrono::Utc::now(),
        })
    }
}
