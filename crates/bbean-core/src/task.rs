use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::config::SchedulerConfig;
use crate::error::{EngineError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub model_id: String,
    pub payload: Vec<u8>,
    pub priority: Option<TaskPriority>,
    pub callback_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskPriority {
    Low,
    Normal,
    High,
    Critical,
}

impl TaskPriority {
    pub fn weight(&self) -> u8 {
        match self {
            Self::Low => 1,
            Self::Normal => 5,
            Self::High => 10,
            Self::Critical => 20,
        }
    }
}

impl Default for TaskPriority {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatedTask {
    pub inner: Task,
    pub priority: TaskPriority,
    pub validated_at: DateTime<Utc>,
}

impl Eq for ValidatedTask {}

impl PartialEq for ValidatedTask {
    fn eq(&self, other: &Self) -> bool {
        self.inner.id == other.inner.id
    }
}

impl PartialOrd for ValidatedTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ValidatedTask {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.weight().cmp(&other.priority.weight())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskReceipt {
    pub id: String,
    pub status: TaskStatus,
    pub queued_at: DateTime<Utc>,
    pub estimated_wait_secs: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Queued,
    Assigned { node_id: String },
    Running { node_id: String, started_at: DateTime<Utc> },
    Completed { result: TaskResult },
    Failed { error: String, retries: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TaskResult {
    pub output: Vec<u8>,
    pub proof_hash: String,
    pub compute_time_ms: u64,
    pub node_id: String,
}

impl Task {
    pub fn new(model_id: impl Into<String>, payload: Vec<u8>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            model_id: model_id.into(),
            payload,
            priority: None,
            callback_url: None,
            created_at: Utc::now(),
        }
    }

    pub fn with_priority(mut self, priority: TaskPriority) -> Self {
        self.priority = Some(priority);
        self
    }

    pub fn with_callback(mut self, url: impl Into<String>) -> Self {
        self.callback_url = Some(url.into());
        self
    }

    pub fn payload_size(&self) -> usize {
        self.payload.len()
    }
}

pub struct Scheduler {
    config: SchedulerConfig,
    queue: RwLock<BinaryHeap<ValidatedTask>>,
    tasks: RwLock<HashMap<String, TaskStatus>>,
    running: RwLock<bool>,
}

impl Scheduler {
    pub fn new(config: SchedulerConfig) -> Self {
        Self {
            config,
            queue: RwLock::new(BinaryHeap::new()),
            tasks: RwLock::new(HashMap::new()),
            running: RwLock::new(false),
        }
    }

    pub async fn start(&self) -> Result<()> {
        let mut running = self.running.write().await;
        *running = true;
        tracing::info!(
            "scheduler started (queue_size={}, batch={})",
            self.config.max_queue_size,
            self.config.batch_size
        );
        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        let mut running = self.running.write().await;
        *running = false;
        tracing::info!("scheduler stopped");
        Ok(())
    }

    pub async fn enqueue(&self, task: ValidatedTask) -> Result<TaskReceipt> {
        let running = self.running.read().await;
        if !*running {
            return Err(EngineError::SchedulerError("scheduler is not running".into()));
        }
        let mut queue = self.queue.write().await;
        if queue.len() >= self.config.max_queue_size {
            return Err(EngineError::CapacityExceeded {
                current: queue.len(),
                max: self.config.max_queue_size,
            });