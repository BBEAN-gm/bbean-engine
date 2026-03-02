<p align="center">
  <img src="assets/banner.png" alt="BBEAN Engine" width="100%">
</p>

<p align="center">
  <a href="https://github.com/BBEAN-gm/bbean-engine/actions"><img src="https://img.shields.io/github/actions/workflow/status/BBEAN-gm/bbean-engine/ci.yml?branch=main&style=for-the-badge&color=4B3621&label=BUILD" alt="Build"></a>
  <a href="https://github.com/BBEAN-gm/bbean-engine/blob/main/LICENSE"><img src="https://img.shields.io/badge/LICENSE-MIT-4B3621?style=for-the-badge" alt="License"></a>
  <a href="https://github.com/BBEAN-gm/bbean-engine"><img src="https://img.shields.io/badge/RUST-1.75+-4B3621?style=for-the-badge&logo=rust&logoColor=white" alt="Rust"></a>
  <a href="https://github.com/BBEAN-gm/bbean-engine"><img src="https://img.shields.io/badge/SOLANA-MAINNET-4B3621?style=for-the-badge&logo=solana&logoColor=white" alt="Solana"></a>
  <a href="https://bbean.fun"><img src="https://img.shields.io/badge/WEB-BBEAN.FUN-4B3621?style=for-the-badge" alt="Website"></a>
</p>

<p align="center">
  <strong>Decentralized compute orchestration engine for browser-based AI inference on Solana.</strong>
</p>

---

## What is BBEAN Engine?

BBEAN Engine is the core infrastructure behind the BBEAN network -- a decentralized physical infrastructure network (DePIN) that turns idle browser tabs into AI compute nodes. When an AI agent needs inference, BBEAN Engine schedules the task across a mesh of WebGPU-enabled browsers, validates the work through Proof-of-Brew consensus, and settles rewards on Solana.

**Open a tab. Run inference. Earn $BBEAN.**

## Architecture

```mermaid
graph LR
    A[AI Agent] -->|Task| B[BBEAN Engine]
    B -->|Schedule| C[Browser Mesh]
    C -->|Proof| D[Brew Validator]
    D -->|Settle| E[Solana]
    E -->|$BBEAN| C
```

| Component | Crate | Description |
|-----------|-------|-------------|
| Core Engine | `bbean-core` | Task scheduling, node registry, proof validation, runtime executor |
| Network | `bbean-network` | P2P transport, peer management, protocol messages |
| Solana Program | `bbean-solana` | On-chain reward pool, staking, token burns |
| CLI | `bbean-cli` | Node operator tooling |
| TypeScript SDK | `@bbean/sdk` | Client library for AI agents |

## Quick Start

```bash
git clone https://github.com/BBEAN-gm/bbean-engine.git
cd bbean-engine
cargo build --workspace
cargo test --workspace
```

### Run a Node

```bash
cargo run --release -p bbean-cli -- start
```

### Submit a Task (TypeScript)

```typescript
import { BbeanClient, TaskPriority } from '@bbean/sdk';

const client = new BbeanClient({
  endpoint: 'http://localhost:9420',
});

await client.connect();

const result = await client
  .task('llama-7b')
  .withPayload('Explain decentralized compute in one sentence.')
  .withPriority(TaskPriority.High)
  .submitAndWait();

console.log(new TextDecoder().decode(result.result?.output));
```

## Proof-of-Brew

BBEAN uses a novel consensus mechanism called Proof-of-Brew to validate browser compute contributions:

1. Engine issues a challenge with a target difficulty
2. Browser node runs the inference task via WebGPU
3. Node computes `SHA-256(task_id || node_id || input_hash || output_hash || nonce)` until the hash meets the difficulty requirement
4. Proof is submitted and validated on-chain
5. Rewards are distributed minus a 5% burn

See [docs/protocol.md](docs/protocol.md) for the full specification.

## Project Structure

```
bbean-engine/
  crates/
    bbean-core/       Core engine: scheduler, nodes, proofs, runtime
    bbean-network/    P2P networking: peers, transport, protocol
    bbean-solana/     Solana program: rewards, staking, burns
    bbean-cli/        CLI for node operators
  sdk/
    typescript/       TypeScript SDK for AI agents
  tests/              Integration tests
  docs/               Protocol and architecture docs
  scripts/            Setup and deployment scripts
```

## Configuration

Create `config.json` in the project root:

```json
{
  "port": 9420,
  "max_nodes": 10000,
  "proof_difficulty": 16,
  "scheduler": {
    "max_queue_size": 50000,
    "batch_size": 64
  },
  "solana": {
    "rpc_url": "https://api.mainnet-beta.solana.com"
  }
}
```

See [docs/getting-started.md](docs/getting-started.md) for full configuration reference.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup and guidelines.

## Security

See [SECURITY.md](SECURITY.md) for our security policy and how to report vulnerabilities.

## Links

- [Website](https://bbean.fun)
- [Documentation](docs/)

## License

MIT License. See [LICENSE](LICENSE) for details.
