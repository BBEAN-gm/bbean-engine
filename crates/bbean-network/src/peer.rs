use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    pub id: String,
    pub address: SocketAddr,
    pub user_agent: String,
    pub capabilities: Vec<String>,
    pub connected_at: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub latency_ms: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PeerState {
    Connecting,
    Connected,
    Authenticated,
    Disconnecting,
    Disconnected,
}

pub struct PeerManager {
    max_peers: usize,
    peers: RwLock<HashMap<String, PeerInfo>>,
    states: RwLock<HashMap<String, PeerState>>,