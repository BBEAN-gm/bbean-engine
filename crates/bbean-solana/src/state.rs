use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct RewardPool {
    pub initialized: bool,
    pub authority: [u8; 32],
    pub reward_rate: u64,
    pub max_nodes: u32,
    pub total_staked: u64,
    pub total_distributed: u64,
    pub total_burned: u64,
    pub nodes: Vec<NodeAccount>,
    pub task_history: Vec<TaskRecord>,
}

impl RewardPool {
    pub fn new(authority: [u8; 32]) -> Self {
        Self {
            initialized: false,
            authority,
            reward_rate: 0,
            max_nodes: 0,
            total_staked: 0,
            total_distributed: 0,
            total_burned: 0,
            nodes: Vec::new(),
            task_history: Vec::new(),
        }
    }

    pub fn active_node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn total_tasks(&self) -> usize {
        self.task_history.len()
    }

    pub fn avg_reward_per_task(&self) -> f64 {