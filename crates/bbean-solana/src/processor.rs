use crate::error::ProgramError;
use crate::instruction::BbeanInstruction;
use crate::state::{NodeAccount, RewardPool, TaskRecord};

pub fn process_instruction(
    program_id: &str,
    instruction: BbeanInstruction,
    pool: &mut RewardPool,
) -> Result<ProcessResult, ProgramError> {
    match instruction {
        BbeanInstruction::InitializePool {
            reward_rate,
            max_nodes,
        } => initialize_pool(program_id, pool, reward_rate, max_nodes),
        BbeanInstruction::RegisterNode {
            node_id,
            stake_amount,
        } => register_node(pool, node_id, stake_amount),
        BbeanInstruction::SubmitProof {
            task_id,
            proof_hash,
            compute_units,
        } => submit_proof(pool, task_id, proof_hash, compute_units),
        BbeanInstruction::ClaimReward { node_id } => claim_reward(pool, node_id),
        BbeanInstruction::UpdateRewardRate { new_rate } => {
            pool.reward_rate = new_rate;
            Ok(ProcessResult::Updated)
        }
        BbeanInstruction::UnregisterNode { node_id } => unregister_node(pool, node_id),
        BbeanInstruction::BurnTokens { amount } => burn_tokens(pool, amount),
        BbeanInstruction::UpdatePool {
            max_nodes,
            reward_rate,
        } => {
            if let Some(max) = max_nodes {
                pool.max_nodes = max;
            }
            if let Some(rate) = reward_rate {
                pool.reward_rate = rate;
            }
            Ok(ProcessResult::Updated)
        }
    }
}

#[derive(Debug)]
pub enum ProcessResult {
    Initialized,
    Registered { node_id: [u8; 32] },
    ProofAccepted { reward: u64 },
    RewardClaimed { amount: u64 },
    Updated,
    Unregistered,
    Burned { amount: u64 },
}

fn initialize_pool(
    _program_id: &str,
    pool: &mut RewardPool,
    reward_rate: u64,
    max_nodes: u32,
) -> Result<ProcessResult, ProgramError> {
    if pool.initialized {
        return Err(ProgramError::AccountAlreadyInitialized);
    }