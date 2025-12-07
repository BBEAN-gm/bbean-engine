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
    pool.reward_rate = reward_rate;
    pool.max_nodes = max_nodes;
    pool.initialized = true;
    pool.total_distributed = 0;
    pool.total_burned = 0;
    Ok(ProcessResult::Initialized)
}

fn register_node(
    pool: &mut RewardPool,
    node_id: [u8; 32],
    stake_amount: u64,
) -> Result<ProcessResult, ProgramError> {
    if !pool.initialized {
        return Err(ProgramError::AccountNotInitialized);
    }
    if stake_amount < crate::MIN_STAKE_AMOUNT {
        return Err(ProgramError::StakeBelowMinimum(stake_amount));
    }
    if pool.nodes.len() as u32 >= pool.max_nodes {
        return Err(ProgramError::InvalidInstruction);
    }
    let node = NodeAccount {
        node_id,
        stake: stake_amount,
        pending_rewards: 0,
        tasks_completed: 0,
        registered_at: current_timestamp(),
        last_claim: 0,
    };
    pool.nodes.push(node);
    pool.total_staked = pool
        .total_staked
        .checked_add(stake_amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    Ok(ProcessResult::Registered { node_id })
}

fn submit_proof(
    pool: &mut RewardPool,
    task_id: [u8; 32],
    proof_hash: [u8; 32],
    compute_units: u64,
) -> Result<ProcessResult, ProgramError> {
    if !pool.initialized {
        return Err(ProgramError::AccountNotInitialized);
    }
    let reward = compute_units
        .checked_mul(pool.reward_rate)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    let burn_amount = reward
        .checked_mul(crate::BURN_RATE_BPS as u64)
        .ok_or(ProgramError::ArithmeticOverflow)?
        / 10_000;
    let net_reward = reward
        .checked_sub(burn_amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    pool.total_distributed = pool
        .total_distributed
        .checked_add(net_reward)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    pool.total_burned = pool
        .total_burned
        .checked_add(burn_amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    let record = TaskRecord {
        task_id,
        proof_hash,
        compute_units,
        reward: net_reward,
        timestamp: current_timestamp(),
    };
    pool.task_history.push(record);
    Ok(ProcessResult::ProofAccepted { reward: net_reward })
}

fn claim_reward(
    pool: &mut RewardPool,
    node_id: [u8; 32],
) -> Result<ProcessResult, ProgramError> {
    let node = pool
        .nodes
        .iter_mut()
        .find(|n| n.node_id == node_id)
        .ok_or(ProgramError::AccountNotInitialized)?;
    let amount = node.pending_rewards;
    if amount == 0 {
        return Err(ProgramError::InsufficientFunds {
            required: 1,
            available: 0,
        });
    }
    node.pending_rewards = 0;
    node.last_claim = current_timestamp();
    Ok(ProcessResult::RewardClaimed { amount })
}

fn unregister_node(
    pool: &mut RewardPool,
    node_id: [u8; 32],
) -> Result<ProcessResult, ProgramError> {
    let idx = pool
        .nodes
        .iter()
        .position(|n| n.node_id == node_id)
        .ok_or(ProgramError::AccountNotInitialized)?;
    let node = pool.nodes.remove(idx);
    pool.total_staked = pool.total_staked.saturating_sub(node.stake);
    Ok(ProcessResult::Unregistered)
}

fn burn_tokens(pool: &mut RewardPool, amount: u64) -> Result<ProcessResult, ProgramError> {
    pool.total_burned = pool
        .total_burned
        .checked_add(amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    Ok(ProcessResult::Burned { amount })
}

fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
