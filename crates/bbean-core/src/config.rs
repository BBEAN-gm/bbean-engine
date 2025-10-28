use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineConfig {
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_max_nodes")]
    pub max_nodes: usize,
    #[serde(default = "default_proof_difficulty")]
    pub proof_difficulty: u8,
    #[serde(default = "default_data_dir")]
    pub data_dir: PathBuf,
    #[serde(default)]
    pub scheduler: SchedulerConfig,
    #[serde(default)]
    pub network: NetworkConfig,
    #[serde(default)]
    pub solana: SolanaConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulerConfig {
    #[serde(default = "default_queue_size")]
    pub max_queue_size: usize,
    #[serde(default = "default_task_timeout")]
    pub task_timeout_secs: u64,
    #[serde(default = "default_max_retries")]
    pub max_retries: u32,
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    #[serde(default = "default_max_peers")]
    pub max_peers: usize,
    #[serde(default = "default_heartbeat")]
    pub heartbeat_interval_secs: u64,
    #[serde(default = "default_discovery_port")]
    pub discovery_port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolanaConfig {
    #[serde(default = "default_rpc")]
    pub rpc_url: String,
    pub program_id: Option<String>,
    pub reward_mint: Option<String>,
    #[serde(default = "default_commitment")]
    pub commitment: String,
}
