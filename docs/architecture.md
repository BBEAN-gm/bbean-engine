# Architecture

## Overview

BBEAN Engine is a decentralized compute orchestration system that coordinates browser-based AI inference tasks across a network of WebGPU-enabled nodes on Solana.

## System Components

```mermaid
graph TD
    A[AI Agent] -->|Submit Task| B[Engine Core]
    B -->|Schedule| C[Task Scheduler]
    C -->|Assign| D[Node Registry]
    D -->|Dispatch| E[Browser Node 1]
    D -->|Dispatch| F[Browser Node 2]
    D -->|Dispatch| G[Browser Node N]
    E -->|Proof| H[Brew Validator]
    F -->|Proof| H
    G -->|Proof| H
    H -->|Verify| I[Solana Program]
    I -->|Reward| J[$BBEAN Token]
```

## Core Engine (`bbean-core`)

The core engine handles task lifecycle management:

- **Task Scheduler**: Priority-based task queue with configurable batch processing. Tasks are distributed to nodes based on compute score and reliability metrics.
- **Node Registry**: Tracks connected browser nodes, their capabilities (WebGPU support, model size limits), and performance metrics.
- **Brew Validator**: Implements Proof-of-Brew consensus by validating SHA-256 based compute proofs with configurable difficulty.
- **Runtime Executor**: Manages concurrent task execution with semaphore-based concurrency control and automatic retry logic.

## Network Layer (`bbean-network`)

Handles peer-to-peer communication between the engine and browser nodes:

- **Peer Manager**: Connection lifecycle management with capacity limits and stale peer pruning.
- **WebSocket Transport**: Message delivery with size validation and broadcast support.
- **Protocol**: Typed message format supporting handshake, task assignment, proof submission, and heartbeat operations.

## Solana Program (`bbean-solana`)

On-chain reward distribution and staking:

- **Reward Pool**: Manages staking, reward calculation, and token burns (5% burn rate per task).
- **Instruction Processor**: Handles pool initialization, node registration, proof submission, and reward claims.
- **State Management**: Borsh-serialized on-chain accounts for nodes, tasks, and the reward pool.

## TypeScript SDK (`@bbean/sdk`)

Client library for AI agents to interact with the engine:

- **BbeanClient**: HTTP client with retry logic, connection management, and task polling.
- **TaskBuilder**: Fluent API for constructing and submitting inference tasks.
- **ProofVerifier**: Client-side proof verification using Web Crypto API.

## Data Flow

```mermaid
sequenceDiagram
    participant Agent
    participant Engine
    participant Node
    participant Solana

    Agent->>Engine: submitTask(model, payload)
    Engine->>Engine: validate + enqueue
    Engine->>Node: assignTask(task)
    Node->>Node: run inference (WebGPU)
    Node->>Engine: submitProof(result, hash)
    Engine->>Engine: validateProof()
    Engine->>Solana: submitProof(onchain)
    Solana->>Solana: calculate reward + burn
    Solana->>Node: distribute $BBEAN
    Engine->>Agent: taskComplete(result)
```

## Performance Considerations

The engine is designed to handle high throughput:

- Task queue supports up to 50,000 pending tasks
- Semaphore-based concurrency control limits parallel dispatches
- Node selection uses reliability-weighted scoring
- Batch dequeue reduces lock contention on the priority queue
