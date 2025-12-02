use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

use crate::error::ProgramError;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub enum BbeanInstruction {
    InitializePool {
        reward_rate: u64,
        max_nodes: u32,
    },
    RegisterNode {
        node_id: [u8; 32],
        stake_amount: u64,
    },
    SubmitProof {
        task_id: [u8; 32],
        proof_hash: [u8; 32],
        compute_units: u64,
    },
    ClaimReward {
        node_id: [u8; 32],
    },
    UpdateRewardRate {
        new_rate: u64,
    },
    UnregisterNode {
        node_id: [u8; 32],
    },
    BurnTokens {
        amount: u64,
    },
    UpdatePool {
        max_nodes: Option<u32>,
        reward_rate: Option<u64>,
    },
}

impl BbeanInstruction {
    pub fn unpack(data: &[u8]) -> Result<Self, ProgramError> {
        Self::try_from_slice(data).map_err(|e| ProgramError::Serialization(e.to_string()))
    }

    pub fn pack(&self) -> Result<Vec<u8>, ProgramError> {
        self.try_to_vec()
            .map_err(|e| ProgramError::Serialization(e.to_string()))
    }

    pub fn instruction_name(&self) -> &'static str {
        match self {
            Self::InitializePool { .. } => "InitializePool",
            Self::RegisterNode { .. } => "RegisterNode",
            Self::SubmitProof { .. } => "SubmitProof",
            Self::ClaimReward { .. } => "ClaimReward",
            Self::UpdateRewardRate { .. } => "UpdateRewardRate",
            Self::UnregisterNode { .. } => "UnregisterNode",
            Self::BurnTokens { .. } => "BurnTokens",
            Self::UpdatePool { .. } => "UpdatePool",
        }
    }
}
