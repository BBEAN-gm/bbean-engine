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
            scheduler
                .update_status(
                    &task_id,
                    TaskStatus::Running {
                        node_id: node_id.clone(),
                        started_at: Utc::now(),
                    },
                )
                .await?;

            match self.dispatch_to_node(&task, &node_id).await {
                Ok(result) => {
                    self.registry
                        .record_task_result(&node_id, result.compute_time_ms, true)
                        .await?;
                    scheduler
                        .update_status(
                            &task_id,
                            TaskStatus::Completed {
                                result: result.clone(),
                            },
                        )
                        .await?;
                    return Ok(result);
                }
                Err(e) => {
                    tracing::warn!(
                        task_id = %task_id,
                        node_id = %node_id,
                        attempt = attempts,
                        error = %e,
                        "task execution failed"
                    );
                    self.registry
                        .record_task_result(&node_id, 0, false)
                        .await?;

                    if attempts >= self.max_retries {
                        scheduler
                            .update_status(
                                &task_id,
                                TaskStatus::Failed {
                                    error: e.to_string(),
                                    retries: attempts,
                                },
                            )
                            .await?;
                        return Err(e);
                    }
                }
            }
        }
    }

    async fn dispatch_to_node(
        &self,
        task: &ValidatedTask,
        node_id: &str,
    ) -> Result<TaskResult> {
        let start = std::time::Instant::now();

        self.registry
            .update_status(
                node_id,
                crate::node::NodeStatus::Busy {
                    task_id: task.inner.id.clone(),
                },
            )
            .await?;

        let output = self.run_inference(task).await?;
        let compute_time_ms = start.elapsed().as_millis() as u64;
        let proof_hash = crate::proof::hash_payload(&output);

        self.registry
            .update_status(node_id, crate::node::NodeStatus::Idle)
            .await?;

        Ok(TaskResult {
            output,
            proof_hash,
            compute_time_ms,
            node_id: node_id.to_string(),
        })
    }

    async fn run_inference(&self, task: &ValidatedTask) -> Result<Vec<u8>> {
        let input_hash = crate::proof::hash_payload(&task.inner.payload);
        let mut output = Vec::with_capacity(task.inner.payload.len());
        output.extend_from_slice(input_hash.as_bytes());
        output.extend_from_slice(&task.inner.payload);
        Ok(output)
    }

    pub fn validator(&self) -> &BrewValidator {
        &self.validator
    }
}
