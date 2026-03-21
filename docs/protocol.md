# Proof-of-Brew Protocol

## Overview

Proof-of-Brew is a consensus mechanism designed for validating browser-based AI compute contributions. It ensures that nodes actually performed the requested inference work before receiving rewards.

## Proof Structure

Each proof contains:

| Field       | Type     | Description                              |
|-------------|----------|------------------------------------------|
| task_id     | String   | Unique task identifier                   |
| node_id     | String   | Node that performed the computation      |
| input_hash  | String   | SHA-256 hash of the input payload        |
| output_hash | String   | SHA-256 hash of the inference output     |
| nonce       | u64      | Proof-of-work nonce                      |
| difficulty  | u8       | Number of required leading zero bits     |
| timestamp   | DateTime | When the proof was generated             |

## Validation Process

1. **Difficulty Check**: The proof's difficulty must meet or exceed the engine's configured minimum (default: 16 bits).

2. **Hash Computation**: The validator computes `SHA-256(task_id || node_id || input_hash || output_hash || nonce)`.

3. **Leading Zeros**: The resulting hash must have at least `difficulty` leading zero bits.

4. **IO Verification**: The combined hash of input and output is cross-referenced against the task record.

## Difficulty Adjustment

The difficulty parameter controls the computational cost of generating a valid proof:

- **8 bits**: Development/testing (minimal overhead)
- **16 bits**: Production default (balanced security)
- **24 bits**: High-security mode (significant compute overhead)

## Hash Chain Verification

For sequential tasks, proofs can be chained:

```
proof[n].input_hash = SHA-256(proof[n-1].output_hash)
```

This creates an auditable chain of compute contributions that can be verified independently.

## Reward Calculation

The net reward for each task is calculated as:

```
net_reward = compute_units * reward_rate * (1 - burn_rate)
```

## On-Chain Settlement

Valid proofs are submitted to the Solana program which:

1. Verifies the proof hash meets difficulty requirements
2. Calculates the reward based on compute units
3. Applies the burn rate (currently 5%)
4. Credits net rewards to the node's pending balance
