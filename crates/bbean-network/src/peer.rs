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
}

impl PeerManager {
    pub fn new(max_peers: usize) -> Self {
        Self {
            max_peers,
            peers: RwLock::new(HashMap::new()),
            states: RwLock::new(HashMap::new()),
        }
    }

    pub async fn add_peer(&self, peer: PeerInfo) -> std::result::Result<(), String> {
        let mut peers = self.peers.write().await;
        if peers.len() >= self.max_peers {
            return Err(format!("peer limit reached: {}", self.max_peers));
        }
        let id = peer.id.clone();
        peers.insert(id.clone(), peer);
        self.states.write().await.insert(id, PeerState::Connected);
        Ok(())
    }

    pub async fn remove_peer(&self, peer_id: &str) -> Option<PeerInfo> {
        self.states.write().await.remove(peer_id);
        self.peers.write().await.remove(peer_id)
    }

    pub async fn get_peer(&self, peer_id: &str) -> Option<PeerInfo> {
        self.peers.read().await.get(peer_id).cloned()
    }

    pub async fn connected_peers(&self) -> Vec<PeerInfo> {
        let peers = self.peers.read().await;
        let states = self.states.read().await;
        peers
            .values()
            .filter(|p| {
                states
                    .get(&p.id)
                    .map(|s| *s == PeerState::Connected || *s == PeerState::Authenticated)