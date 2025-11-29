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
        if self.task_history.is_empty() {
            return 0.0;
        }
        self.total_distributed as f64 / self.task_history.len() as f64
    }

    pub fn burn_ratio(&self) -> f64 {
        let total = self.total_distributed + self.total_burned;
        if total == 0 {
            return 0.0;
        }
        self.total_burned as f64 / total as f64
    }
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct NodeAccount {
    pub node_id: [u8; 32],
    pub stake: u64,
    pub pending_rewards: u64,
    pub tasks_completed: u64,
    pub registered_at: u64,
    pub last_claim: u64,
}

impl NodeAccount {
    pub fn is_eligible_for_claim(&self) -> bool {
        self.pending_rewards > 0
    }

    pub fn uptime_secs(&self) -> u64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        now.saturating_sub(self.registered_at)
    }

    pub fn avg_reward_per_task(&self) -> f64 {
        if self.tasks_completed == 0 {
            return 0.0;
        }
        self.pending_rewards as f64 / self.tasks_completed as f64
    }
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct TaskRecord {
    pub task_id: [u8; 32],
    pub proof_hash: [u8; 32],
    pub compute_units: u64,
    pub reward: u64,
    pub timestamp: u64,
}
