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
