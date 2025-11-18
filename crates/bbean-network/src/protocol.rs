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
        Self::new(MessageType::Handshake, capabilities.to_vec(), sender_id)
    }

    pub fn error(sender_id: impl Into<String>, error_msg: &str) -> Self {
        Self::new(MessageType::Error, error_msg.as_bytes().to_vec(), sender_id)
    }

    pub fn encode(&self) -> std::result::Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(self)
    }

    pub fn decode(data: &[u8]) -> std::result::Result<Self, serde_json::Error> {
        serde_json::from_slice(data)
    }

    pub fn payload_as_str(&self) -> Option<&str> {
        std::str::from_utf8(&self.payload).ok()
    }

    pub fn is_control(&self) -> bool {
        matches!(
            self.msg_type,
            MessageType::Handshake
                | MessageType::HandshakeAck
                | MessageType::Heartbeat
                | MessageType::HeartbeatAck
        )
    }
}

impl std::fmt::Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Handshake => write!(f, "HANDSHAKE"),
            Self::HandshakeAck => write!(f, "HANDSHAKE_ACK"),
            Self::TaskAssign => write!(f, "TASK_ASSIGN"),
            Self::TaskResult => write!(f, "TASK_RESULT"),
            Self::ProofSubmit => write!(f, "PROOF_SUBMIT"),
            Self::ProofAck => write!(f, "PROOF_ACK"),
            Self::Heartbeat => write!(f, "HEARTBEAT"),
            Self::HeartbeatAck => write!(f, "HEARTBEAT_ACK"),
            Self::NodeAnnounce => write!(f, "NODE_ANNOUNCE"),
            Self::NodeLeave => write!(f, "NODE_LEAVE"),
            Self::Error => write!(f, "ERROR"),
        }
    }
}
