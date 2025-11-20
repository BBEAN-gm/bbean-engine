use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::sync::mpsc;

use crate::protocol::ProtocolMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportConfig {
    pub bind_address: SocketAddr,
    pub max_connections: usize,
    pub max_message_size: usize,
    pub ping_interval_secs: u64,
}

impl Default for TransportConfig {
    fn default() -> Self {
        Self {
            bind_address: "0.0.0.0:9420".parse().unwrap(),
            max_connections: 1024,
            max_message_size: 64 * 1024 * 1024,
            ping_interval_secs: 30,
        }
    }
}

pub trait Transport: Send + Sync {
    fn send(&self, peer_id: &str, message: ProtocolMessage) -> std::result::Result<(), TransportError>;
    fn broadcast(&self, message: ProtocolMessage) -> std::result::Result<usize, TransportError>;
    fn disconnect(&self, peer_id: &str) -> std::result::Result<(), TransportError>;
}

#[derive(Debug, thiserror::Error)]
pub enum TransportError {
    #[error("connection refused: {0}")]
    ConnectionRefused(String),
    #[error("message too large: {size} bytes (max {max})")]
    MessageTooLarge { size: usize, max: usize },
    #[error("peer not found: {0}")]
    PeerNotFound(String),
    #[error("send failed: {0}")]
    SendFailed(String),
    #[error("channel closed")]
    ChannelClosed,
}

pub struct WsTransport {
    config: TransportConfig,
    outbound: mpsc::Sender<(String, ProtocolMessage)>,
    _receiver: mpsc::Receiver<(String, ProtocolMessage)>,
}

impl WsTransport {
    pub fn new(config: TransportConfig) -> Self {
        let (tx, rx) = mpsc::channel(4096);
        Self {
            config,
            outbound: tx,
            _receiver: rx,
        }
    }

    pub fn config(&self) -> &TransportConfig {
        &self.config
    }

    fn check_message_size(&self, msg: &ProtocolMessage) -> std::result::Result<(), TransportError> {
        let size = serde_json::to_vec(msg).unwrap_or_default().len();
        if size > self.config.max_message_size {
            return Err(TransportError::MessageTooLarge {
                size,
                max: self.config.max_message_size,
            });
        }
        Ok(())
    }
}

impl Transport for WsTransport {
    fn send(&self, peer_id: &str, message: ProtocolMessage) -> std::result::Result<(), TransportError> {
        self.check_message_size(&message)?;
        self.outbound
            .try_send((peer_id.to_string(), message))
            .map_err(|_| TransportError::ChannelClosed)
    }

    fn broadcast(&self, message: ProtocolMessage) -> std::result::Result<usize, TransportError> {
        self.check_message_size(&message)?;
        self.outbound
            .try_send(("*".to_string(), message))
            .map_err(|_| TransportError::ChannelClosed)?;
        Ok(1)
    }

    fn disconnect(&self, peer_id: &str) -> std::result::Result<(), TransportError> {
        tracing::debug!(peer_id = peer_id, "disconnecting peer");
        Ok(())
    }
}
