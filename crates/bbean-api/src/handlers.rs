use std::sync::Arc;

use axum::extract::{Path, State};
use axum::Json;
use serde::{Deserialize, Serialize};

use bbean_core::proof::BrewProof;
use bbean_core::task::{Task, TaskPriority};

use crate::response::{ApiError, ApiResponse};
use crate::state::AppState;

// ---------- Health ----------

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

pub async fn health() -> Json<ApiResponse<HealthResponse>> {
    ApiResponse::ok(HealthResponse {
        status: "ok".into(),
        version: bbean_core::VERSION.into(),
    })
}

// ---------- Engine Status ----------

#[derive(Serialize)]
pub struct StatusResponse {
    pub running: bool,
    pub version: String,
    pub uptime_secs: i64,
    pub node_count: usize,
    pub port: u16,
    pub proof_difficulty: u8,
}

pub async fn engine_status(
    State(state): State<Arc<AppState>>,
) -> Json<ApiResponse<StatusResponse>> {
    let engine = state.engine.read().await;
    let node_count = engine.get_node_count().await;
    ApiResponse::ok(StatusResponse {
        running: engine.is_running(),
        version: bbean_core::VERSION.into(),
        uptime_secs: state.uptime_secs(),
        node_count,
        port: state.config.port,
        proof_difficulty: state.config.proof_difficulty,
    })
}

// ---------- Tasks ----------

#[derive(Deserialize)]
pub struct SubmitTaskRequest {
    pub model_id: String,
    pub payload: Vec<u8>,
    pub priority: Option<String>,
    pub callback_url: Option<String>,
}

#[derive(Serialize)]
pub struct TaskReceiptResponse {
    pub task_id: String,
    pub status: String,
    pub queued_at: String,
    pub estimated_wait_secs: Option<u64>,
}

pub async fn submit_task(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SubmitTaskRequest>,
) -> Result<Json<ApiResponse<TaskReceiptResponse>>, ApiError> {
    let priority = match req.priority.as_deref() {
        Some("low") => TaskPriority::Low,
        Some("high") => TaskPriority::High,
        Some("critical") => TaskPriority::Critical,
        _ => TaskPriority::Normal,
    };

    let mut task = Task::new(&req.model_id, req.payload);
    task.priority = Some(priority);
    task.callback_url = req.callback_url;

    let engine = state.engine.read().await;
    let receipt = engine.submit_task(task).await.map_err(ApiError::from)?;

    Ok(ApiResponse::ok(TaskReceiptResponse {
        task_id: receipt.id,
        status: "queued".into(),
        queued_at: receipt.queued_at.to_rfc3339(),
        estimated_wait_secs: receipt.estimated_wait_secs,
    }))
}

#[derive(Serialize)]
pub struct TaskResponse {
    pub task_id: String,
    pub status: String,
}

pub async fn get_task(
    State(state): State<Arc<AppState>>,
    Path(task_id): Path<String>,
) -> Result<Json<ApiResponse<TaskResponse>>, ApiError> {
    let engine = state.engine.read().await;
    let status = engine.get_task_status(&task_id).await.map_err(ApiError::from)?;
    Ok(ApiResponse::ok(TaskResponse {
        task_id,
        status: format!("{:?}", status),
    }))
}

pub async fn get_task_status(
    State(state): State<Arc<AppState>>,
    Path(task_id): Path<String>,
) -> Result<Json<ApiResponse<TaskResponse>>, ApiError> {
    get_task(State(state), Path(task_id)).await
}

// ---------- Nodes ----------

#[derive(Serialize)]
pub struct NodesResponse {
    pub count: usize,
}

pub async fn list_nodes(
    State(state): State<Arc<AppState>>,
) -> Json<ApiResponse<NodesResponse>> {
    let engine = state.engine.read().await;
    let count = engine.get_node_count().await;
    ApiResponse::ok(NodesResponse { count })
}

#[derive(Serialize)]
pub struct NodeResponse {
    pub node_id: String,
    pub message: String,
}

pub async fn get_node(
    Path(node_id): Path<String>,
) -> Json<ApiResponse<NodeResponse>> {
    ApiResponse::ok(NodeResponse {
        node_id: node_id.clone(),
        message: format!("node {} info", node_id),
    })
}

pub async fn get_node_metrics(
    Path(node_id): Path<String>,
) -> Json<ApiResponse<NodeResponse>> {
    ApiResponse::ok(NodeResponse {
        node_id: node_id.clone(),
        message: format!("node {} metrics", node_id),
    })
}

// ---------- Proofs ----------

#[derive(Deserialize)]
pub struct ValidateProofRequest {
    pub task_id: String,
    pub node_id: String,
    pub input_hash: String,
    pub output_hash: String,
    pub nonce: u64,
    pub difficulty: u8,
}

#[derive(Serialize)]
pub struct ProofResponse {
    pub valid: bool,
    pub task_id: String,
}

pub async fn validate_proof(
    Json(req): Json<ValidateProofRequest>,
) -> Result<Json<ApiResponse<ProofResponse>>, ApiError> {
    let proof = BrewProof {
        task_id: req.task_id.clone(),
        node_id: req.node_id,
        input_hash: req.input_hash,
        output_hash: req.output_hash,
        nonce: req.nonce,
        difficulty: req.difficulty,
        timestamp: chrono::Utc::now(),
    };

    let validator = bbean_core::proof::BrewValidator::new(req.difficulty);
    match validator.validate(&proof) {
        Ok(valid) => Ok(ApiResponse::ok(ProofResponse {
            valid,
            task_id: req.task_id,
        })),
        Err(e) => Ok(ApiResponse::ok(ProofResponse {
            valid: false,
            task_id: req.task_id,
        })),
    }
}

// ---------- Config ----------

#[derive(Serialize)]
pub struct ConfigResponse {
    pub port: u16,
    pub max_nodes: usize,
    pub proof_difficulty: u8,
    pub max_queue_size: usize,
    pub batch_size: usize,
}

pub async fn get_config(
    State(state): State<Arc<AppState>>,
) -> Json<ApiResponse<ConfigResponse>> {
    ApiResponse::ok(ConfigResponse {
        port: state.config.port,
        max_nodes: state.config.max_nodes,
        proof_difficulty: state.config.proof_difficulty,
        max_queue_size: state.config.scheduler.max_queue_size,
        batch_size: state.config.scheduler.batch_size,
    })
}
