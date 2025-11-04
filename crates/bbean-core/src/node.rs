use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

use crate::error::{EngineError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub id: String,
    pub address: String,
    pub capabilities: NodeCapabilities,
    pub status: NodeStatus,
    pub connected_at: DateTime<Utc>,
    pub last_heartbeat: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeCapabilities {
    pub webgpu: bool,
    pub max_model_size_mb: u32,
    pub supported_formats: Vec<String>,
    pub compute_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeStatus {
    Connected,
    Idle,
    Busy { task_id: String },
    Disconnected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMetrics {
    pub tasks_completed: u64,
    pub tasks_failed: u64,
    pub total_compute_ms: u64,
    pub avg_proof_time_ms: f64,
    pub uptime_secs: u64,
    pub reliability_score: f64,
}

impl NodeMetrics {
    pub fn new() -> Self {
        Self {
            tasks_completed: 0,
            tasks_failed: 0,
            total_compute_ms: 0,
            avg_proof_time_ms: 0.0,
            uptime_secs: 0,
            reliability_score: 1.0,
        }
    }

    pub fn record_completion(&mut self, compute_ms: u64) {
        self.tasks_completed += 1;
        self.total_compute_ms += compute_ms;
        let total = self.tasks_completed + self.tasks_failed;
        self.avg_proof_time_ms =
            (self.avg_proof_time_ms * (total - 1) as f64 + compute_ms as f64) / total as f64;
        self.update_reliability();
    }

    pub fn record_failure(&mut self) {
        self.tasks_failed += 1;
        self.update_reliability();
    }

    fn update_reliability(&mut self) {
        let total = self.tasks_completed + self.tasks_failed;
        if total > 0 {
            self.reliability_score = self.tasks_completed as f64 / total as f64;
        }
    }

    pub fn success_rate(&self) -> f64 {
        let total = self.tasks_completed + self.tasks_failed;
        if total == 0 {
            return 1.0;
        }
        self.tasks_completed as f64 / total as f64
    }
}

pub struct NodeRegistry {
    max_nodes: usize,
    nodes: RwLock<HashMap<String, NodeInfo>>,
    metrics: RwLock<HashMap<String, NodeMetrics>>,
}

impl NodeRegistry {
    pub fn new(max_nodes: usize) -> Self {
        Self {
            max_nodes,