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
            nodes: RwLock::new(HashMap::new()),
            metrics: RwLock::new(HashMap::new()),
        }
    }

    pub async fn register(&self, node: NodeInfo) -> Result<()> {
        let mut nodes = self.nodes.write().await;
        if nodes.len() >= self.max_nodes {
            return Err(EngineError::CapacityExceeded {
                current: nodes.len(),
                max: self.max_nodes,
            });
        }
        let id = node.id.clone();
        nodes.insert(id.clone(), node);
        self.metrics.write().await.insert(id, NodeMetrics::new());
        Ok(())
    }

    pub async fn unregister(&self, node_id: &str) -> Result<()> {
        let mut nodes = self.nodes.write().await;
        nodes
            .remove(node_id)
            .ok_or_else(|| EngineError::NodeNotFound(node_id.into()))?;
        self.metrics.write().await.remove(node_id);
        Ok(())
    }

    pub async fn get_node(&self, node_id: &str) -> Result<NodeInfo> {
        let nodes = self.nodes.read().await;
        nodes
            .get(node_id)
            .cloned()
            .ok_or_else(|| EngineError::NodeNotFound(node_id.into()))
    }

    pub async fn get_idle_nodes(&self) -> Vec<NodeInfo> {
        let nodes = self.nodes.read().await;
        nodes
            .values()
            .filter(|n| n.status == NodeStatus::Idle)
            .cloned()
            .collect()
    }

    pub async fn select_best_node(&self) -> Option<NodeInfo> {
        let nodes = self.nodes.read().await;
        let metrics = self.metrics.read().await;
        nodes
            .values()
            .filter(|n| n.status == NodeStatus::Idle)
            .max_by(|a, b| {
                let ma = metrics.get(&a.id).map(|m| m.reliability_score).unwrap_or(0.5);
                let mb = metrics.get(&b.id).map(|m| m.reliability_score).unwrap_or(0.5);
                let sa = a.capabilities.compute_score * ma;
                let sb = b.capabilities.compute_score * mb;
                sa.partial_cmp(&sb).unwrap_or(std::cmp::Ordering::Equal)
            })
            .cloned()
    }

    pub async fn update_status(&self, node_id: &str, status: NodeStatus) -> Result<()> {
        let mut nodes = self.nodes.write().await;
        let node = nodes
            .get_mut(node_id)
            .ok_or_else(|| EngineError::NodeNotFound(node_id.into()))?;
        node.status = status;
        node.last_heartbeat = Utc::now();
        Ok(())
    }

    pub async fn active_count(&self) -> usize {
        let nodes = self.nodes.read().await;
        nodes
            .values()
            .filter(|n| n.status != NodeStatus::Disconnected)
            .count()
    }

    pub async fn start_discovery(&self) -> Result<()> {
        tracing::info!("node discovery started");
        Ok(())
    }

    pub async fn disconnect_all(&self) -> Result<()> {
        let mut nodes = self.nodes.write().await;
        for node in nodes.values_mut() {
            node.status = NodeStatus::Disconnected;
        }
        tracing::info!("all nodes disconnected");
        Ok(())
    }

    pub async fn record_task_result(
        &self,
        node_id: &str,
        compute_ms: u64,
        success: bool,
    ) -> Result<()> {
        let mut metrics = self.metrics.write().await;
        let m = metrics
            .get_mut(node_id)
            .ok_or_else(|| EngineError::NodeNotFound(node_id.into()))?;
        if success {
            m.record_completion(compute_ms);
        } else {
            m.record_failure();
        }
        Ok(())
    }

    pub async fn get_metrics(&self, node_id: &str) -> Result<NodeMetrics> {
        let metrics = self.metrics.read().await;
        metrics
            .get(node_id)
            .cloned()
            .ok_or_else(|| EngineError::NodeNotFound(node_id.into()))
    }
}
