use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolMessage {
    pub id: String,
    pub msg_type: MessageType,
    pub payload: Vec<u8>,
    pub sender_id: String,
    pub timestamp: DateTime<Utc>,
    pub version: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageType {
    Handshake,
    HandshakeAck,
    TaskAssign,
    TaskResult,
    ProofSubmit,
    ProofAck,
    Heartbeat,
    HeartbeatAck,
    NodeAnnounce,
    NodeLeave,
    Error,
}

impl ProtocolMessage {
    pub fn new(msg_type: MessageType, payload: Vec<u8>, sender_id: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            msg_type,
            payload,
            sender_id: sender_id.into(),
            timestamp: Utc::now(),
            version: 1,
        }
    }

    pub fn heartbeat(sender_id: impl Into<String>) -> Self {
        Self::new(MessageType::Heartbeat, vec![], sender_id)
    }

    pub fn handshake(sender_id: impl Into<String>, capabilities: &[u8]) -> Self {