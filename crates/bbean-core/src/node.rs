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