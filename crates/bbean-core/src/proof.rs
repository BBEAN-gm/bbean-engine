use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::error::{EngineError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrewProof {
    pub task_id: String,
    pub node_id: String,
    pub input_hash: String,
    pub output_hash: String,
    pub nonce: u64,
    pub difficulty: u8,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofChallenge {
    pub challenge_id: String,
    pub task_hash: String,
    pub difficulty: u8,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

pub struct BrewValidator {
    difficulty: u8,
}

impl BrewValidator {
    pub fn new(difficulty: u8) -> Self {
        Self { difficulty }
    }
