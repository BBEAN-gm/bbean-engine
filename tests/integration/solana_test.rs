use bbean_solana::instruction::BbeanInstruction;
use bbean_solana::processor::{process_instruction, ProcessResult};
use bbean_solana::state::RewardPool;

#[test]
fn test_initialize_pool() {
    let mut pool = RewardPool::new([0u8; 32]);
    let ix = BbeanInstruction::InitializePool {
        reward_rate: 100,
        max_nodes: 1000,
    };
    let result = process_instruction("test", ix, &mut pool).unwrap();
    assert!(matches!(result, ProcessResult::Initialized));
    assert!(pool.initialized);
    assert_eq!(pool.reward_rate, 100);
}

#[test]
fn test_register_node() {
    let mut pool = RewardPool::new([0u8; 32]);
    pool.initialized = true;
    pool.max_nodes = 10;

    let node_id = [1u8; 32];
    let ix = BbeanInstruction::RegisterNode {
        node_id,
        stake_amount: 2_000_000_000,
    };
    let result = process_instruction("test", ix, &mut pool).unwrap();
    assert!(matches!(result, ProcessResult::Registered { .. }));
    assert_eq!(pool.nodes.len(), 1);
}

#[test]
fn test_stake_below_minimum() {
    let mut pool = RewardPool::new([0u8; 32]);
    pool.initialized = true;
    pool.max_nodes = 10;

    let ix = BbeanInstruction::RegisterNode {
        node_id: [1u8; 32],
        stake_amount: 100,
    };
    assert!(process_instruction("test", ix, &mut pool).is_err());
}

#[test]
fn test_double_init() {
    let mut pool = RewardPool::new([0u8; 32]);
    pool.initialized = true;
    let ix = BbeanInstruction::InitializePool {
        reward_rate: 100,
        max_nodes: 10,
    };
    assert!(process_instruction("test", ix, &mut pool).is_err());
}
