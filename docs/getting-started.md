# Getting Started

## Prerequisites

- Rust 1.75+ (install from https://rustup.rs)
- Node.js 18+ (optional, for TypeScript SDK)
- A Solana wallet (for reward claims)

## Build from Source

```bash
git clone https://github.com/BBEAN-gm/bbean-engine.git
cd bbean-engine
cargo build --workspace
```

## Run Tests

```bash
cargo test --workspace
```

## Configuration

Create a `config.json` file:

```json
{
  "port": 9420,
  "host": "0.0.0.0",
  "max_nodes": 10000,
  "proof_difficulty": 16,
  "scheduler": {
    "max_queue_size": 50000,
    "task_timeout_secs": 300,
    "batch_size": 64
  },
  "solana": {
    "rpc_url": "https://api.mainnet-beta.solana.com",
    "commitment": "confirmed"
  }
}
```

## Start a Node

```bash
export BBEAN_CONFIG=config.json
cargo run --release -p bbean-cli -- start
```

## Submit a Task (TypeScript SDK)

```typescript
import { BbeanClient, TaskPriority } from '@bbean/sdk';

const client = new BbeanClient({ endpoint: 'http://localhost:9420' });
await client.connect();

const status = await client
  .task('llama-7b')
  .withPayload('What is decentralized compute?')
  .withPriority(TaskPriority.High)
  .submitAndWait();

console.log(status.result);
```
