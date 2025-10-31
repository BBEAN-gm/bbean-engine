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

fn default_port() -> u16 { 9420 }
fn default_host() -> String { "0.0.0.0".into() }
fn default_max_nodes() -> usize { 10_000 }
fn default_proof_difficulty() -> u8 { 16 }
fn default_data_dir() -> PathBuf { PathBuf::from("./data") }
fn default_queue_size() -> usize { 50_000 }
fn default_task_timeout() -> u64 { 300 }
fn default_max_retries() -> u32 { 3 }
fn default_batch_size() -> usize { 64 }
fn default_max_peers() -> usize { 256 }
fn default_heartbeat() -> u64 { 30 }
fn default_discovery_port() -> u16 { 9421 }
fn default_rpc() -> String { "https://api.mainnet-beta.solana.com".into() }
fn default_commitment() -> String { "confirmed".into() }

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            port: default_port(),
            host: default_host(),
            max_nodes: default_max_nodes(),
            proof_difficulty: default_proof_difficulty(),
            data_dir: default_data_dir(),
            scheduler: SchedulerConfig::default(),
            network: NetworkConfig::default(),
            solana: SolanaConfig::default(),
        }
    }
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            max_queue_size: default_queue_size(),
            task_timeout_secs: default_task_timeout(),
            max_retries: default_max_retries(),
            batch_size: default_batch_size(),
        }
    }
}