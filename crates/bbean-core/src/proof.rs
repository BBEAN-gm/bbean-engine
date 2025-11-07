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
    }

    fn compute_proof_hash(&self, proof: &BrewProof) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(proof.task_id.as_bytes());
        hasher.update(proof.node_id.as_bytes());
        hasher.update(proof.input_hash.as_bytes());
        hasher.update(proof.output_hash.as_bytes());
        hasher.update(proof.nonce.to_le_bytes());
        hasher.finalize().to_vec()
    }

    fn compute_io_hash(&self, input: &str, output: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        hasher.update(output.as_bytes());
        hex::encode(hasher.finalize())
    }

    pub fn difficulty(&self) -> u8 {
        self.difficulty
    }
}

pub fn hash_payload(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

fn count_leading_zero_bits(hash: &[u8]) -> usize {
    let mut count = 0;
    for byte in hash {
        if *byte == 0 {
            count += 8;
        } else {
            count += byte.leading_zeros() as usize;
            break;
        }
    }
    count
}

pub fn verify_hash_chain(hashes: &[String]) -> bool {
    if hashes.len() < 2 {
        return true;
    }
    for window in hashes.windows(2) {
        let mut hasher = Sha256::new();
        hasher.update(window[0].as_bytes());
        let expected = hex::encode(hasher.finalize());
        if expected != window[1] {
            return false;
        }
    }
    true
}
