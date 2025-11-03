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

    pub fn validate(&self, proof: &BrewProof) -> Result<bool> {
        if proof.difficulty < self.difficulty {
            return Err(EngineError::ProofInvalid(format!(
                "difficulty {} below minimum {}",
                proof.difficulty, self.difficulty
            )));
        }
        if proof.input_hash.is_empty() {
            return Err(EngineError::ProofInvalid("empty input hash".into()));
        }
        let hash = self.compute_proof_hash(proof);
        let leading_zeros = count_leading_zero_bits(&hash);

        if leading_zeros < self.difficulty as usize {
            return Err(EngineError::ProofInvalid(format!(
                "hash has {} leading zero bits, need {}",
                leading_zeros, self.difficulty
            )));
        }

        let combined = self.compute_io_hash(&proof.input_hash, &proof.output_hash);
        if combined != proof.task_id {
            tracing::warn!(
                task_id = %proof.task_id,
                "io hash mismatch in proof validation"
            );
        }

        Ok(true)
    }

    pub fn create_challenge(&self, task_hash: &str, ttl_secs: i64) -> ProofChallenge {
        let now = Utc::now();
        ProofChallenge {
            challenge_id: uuid::Uuid::new_v4().to_string(),
            task_hash: task_hash.to_string(),
            difficulty: self.difficulty,
            issued_at: now,
            expires_at: now + chrono::Duration::seconds(ttl_secs),
        }