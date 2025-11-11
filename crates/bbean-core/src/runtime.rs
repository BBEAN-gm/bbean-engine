use std::sync::Arc;
use tokio::sync::Semaphore;
use chrono::Utc;

use crate::error::{EngineError, Result};
use crate::node::NodeRegistry;
use crate::proof::BrewValidator;
use crate::task::{Scheduler, TaskResult, TaskStatus, ValidatedTask};

pub struct TaskExecutor {
    registry: Arc<NodeRegistry>,
    validator: Arc<BrewValidator>,
    concurrency: Semaphore,
    max_retries: u32,
}

impl TaskExecutor {
    pub fn new(
        registry: Arc<NodeRegistry>,
        validator: Arc<BrewValidator>,
        max_concurrent: usize,
        max_retries: u32,
    ) -> Self {
        Self {
            registry,
            validator,
            concurrency: Semaphore::new(max_concurrent),
            max_retries,
        }
    }

    pub async fn execute(&self, scheduler: &Scheduler, task: ValidatedTask) -> Result<TaskResult> {
        let _permit = self
            .concurrency
            .acquire()
            .await
            .map_err(|e| EngineError::SchedulerError(e.to_string()))?;

        let task_id = task.inner.id.clone();
        let mut attempts = 0;

        loop {
            attempts += 1;
            let node = self
                .registry
                .select_best_node()
                .await
                .ok_or_else(|| EngineError::SchedulerError("no available nodes".into()))?;

            let node_id = node.id.clone();