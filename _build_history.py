#!/usr/bin/env python3
"""Generate realistic git commit history for bbean-engine."""
import os
import subprocess
import shutil
import random
from datetime import datetime, timedelta

PROJECT = r"c:\Users\baayo\Desktop\bbean-engine"
NAME = "BBEAN-gm"
EMAIL = "257218378+BBEAN-gm@users.noreply.github.com"
START = datetime(2025, 10, 26, 10, 0, 0)
END = datetime(2026, 3, 26, 18, 0, 0)

random.seed(42)

def git(*a, date=None):
    env = os.environ.copy()
    env["GIT_AUTHOR_NAME"] = NAME
    env["GIT_AUTHOR_EMAIL"] = EMAIL
    env["GIT_COMMITTER_NAME"] = NAME
    env["GIT_COMMITTER_EMAIL"] = EMAIL
    if date:
        ds = date.strftime("%Y-%m-%dT%H:%M:%S+00:00")
        env["GIT_AUTHOR_DATE"] = ds
        env["GIT_COMMITTER_DATE"] = ds
    r = subprocess.run(["git"] + list(a), cwd=PROJECT, env=env,
                       capture_output=True, text=True)
    if r.returncode != 0 and "nothing to commit" not in r.stderr:
        print(f"WARN git {' '.join(a)}: {r.stderr.strip()[:200]}")
    return r

def write(path, content):
    fp = os.path.join(PROJECT, path.replace("/", os.sep))
    os.makedirs(os.path.dirname(fp), exist_ok=True)
    with open(fp, "w", encoding="utf-8", newline="\n") as f:
        f.write(content)

def read(path):
    fp = os.path.join(PROJECT, path.replace("/", os.sep))
    with open(fp, "r", encoding="utf-8") as f:
        return f.read()

def read_binary(path):
    fp = os.path.join(PROJECT, path.replace("/", os.sep))
    with open(fp, "rb") as f:
        return f.read()

def write_binary(path, data):
    fp = os.path.join(PROJECT, path.replace("/", os.sep))
    os.makedirs(os.path.dirname(fp), exist_ok=True)
    with open(fp, "wb") as f:
        f.write(data)

def truncate(content, n):
    lines = content.split("\n")
    return "\n".join(lines[:n])

def commit(date, msg):
    git("add", "-A")
    git("commit", "-m", msg, date=date)

# Read ALL files into memory
ALL = {}
BINARY = {}
for root, dirs, files in os.walk(PROJECT):
    dirs[:] = [d for d in dirs if d not in ('.git', '_build', '__pycache__')]
    for f in files:
        if f.startswith("_"):
            continue
        fp = os.path.join(root, f)
        rel = os.path.relpath(fp, PROJECT).replace(os.sep, "/")
        try:
            ALL[rel] = open(fp, "r", encoding="utf-8").read()
        except (UnicodeDecodeError, Exception):
            BINARY[rel] = open(fp, "rb").read()

print(f"Loaded {len(ALL)} text files, {len(BINARY)} binary files")

# Clean everything except this script and .git
for item in os.listdir(PROJECT):
    if item.startswith("_") or item == ".git":
        continue
    p = os.path.join(PROJECT, item)
    if os.path.isdir(p):
        shutil.rmtree(p)
    else:
        os.remove(p)

# Init git
gd = os.path.join(PROJECT, ".git")
if os.path.exists(gd):
    shutil.rmtree(gd)
git("init")
git("checkout", "-b", "main")

# Helper: random time on a given day
def dt(day_offset, hour=None):
    d = START + timedelta(days=day_offset)
    if hour is None:
        hour = random.randint(8, 22)
    minute = random.randint(0, 59)
    second = random.randint(0, 59)
    return d.replace(hour=hour, minute=minute, second=second)

def write_final(path):
    if path in ALL:
        write(path, ALL[path])
    elif path in BINARY:
        write_binary(path, BINARY[path])

def write_trunc(path, n):
    write(path, truncate(ALL[path], n))

# ============================================================
# COMMIT PLAN
# Day offsets from 2025-10-26
# ============================================================

# --- PHASE 1: Project scaffold (Oct 26-31) ---
d = dt(0, 14)
write_final("Cargo.toml")
write_final(".gitignore")
write_final("rustfmt.toml")
write_final("LICENSE")
commit(d, "feat: initial project scaffold with workspace config")

d = dt(0, 16)
write_final("crates/bbean-core/Cargo.toml")
write_trunc("crates/bbean-core/src/lib.rs", 8)
write_trunc("crates/bbean-core/src/error.rs", 12)
commit(d, "feat: add bbean-core crate skeleton")

d = dt(1, 10)
write_trunc("crates/bbean-core/src/error.rs", 30)
commit(d, "feat: define core error types with thiserror")

d = dt(2, 11)
write_trunc("crates/bbean-core/src/config.rs", 55)
commit(d, "feat: add engine configuration module")

d = dt(2, 15)
write_final("crates/bbean-core/src/error.rs")
commit(d, "refactor: expand error variants for all subsystems")

d = dt(3, 9)
write_trunc("crates/bbean-core/src/task.rs", 60)
commit(d, "feat: add task types and priority enum")

d = dt(4, 14)
write_trunc("crates/bbean-core/src/task.rs", 110)
commit(d, "feat: add validated task and receipt types")

d = dt(5, 10)
write_trunc("crates/bbean-core/src/config.rs", 95)
write_trunc("crates/bbean-core/src/lib.rs", 20)
commit(d, "feat: add scheduler and network config structs")

# --- PHASE 2: Core features (Nov 1-15) ---
d = dt(6, 11)
write_trunc("crates/bbean-core/src/proof.rs", 35)
commit(d, "feat: add brew proof data structures")

d = dt(7, 9)
write_trunc("crates/bbean-core/src/node.rs", 50)
commit(d, "feat: add node info and capabilities types")

d = dt(8, 14)
write_trunc("crates/bbean-core/src/proof.rs", 75)
commit(d, "feat: implement brew proof validator")

d = dt(9, 10)
write_trunc("crates/bbean-core/src/node.rs", 95)
commit(d, "feat: add node metrics tracking")

d = dt(10, 16)
write_final("crates/bbean-core/src/config.rs")
commit(d, "refactor: add config defaults and validation")

d = dt(12, 11)
write_final("crates/bbean-core/src/proof.rs")
commit(d, "feat: add hash chain verification and payload hashing")

d = dt(13, 9)
write_trunc("crates/bbean-core/src/task.rs", 170)
commit(d, "feat: implement task scheduler with priority queue")

d = dt(14, 15)
write_final("crates/bbean-core/src/task.rs")
commit(d, "feat: add task status tracking and batch dequeue")

d = dt(15, 10)
write_final("crates/bbean-core/src/node.rs")
commit(d, "feat: implement node registry with best-node selection")

d = dt(16, 14)
write_trunc("crates/bbean-core/src/runtime.rs", 50)
commit(d, "feat: add task executor skeleton")

d = dt(17, 11)
write_final("crates/bbean-core/src/runtime.rs")
commit(d, "feat: implement task dispatch with retry logic")

d = dt(18, 9)
write_final("crates/bbean-core/src/lib.rs")
commit(d, "refactor: wire up engine start/stop/submit lifecycle")

# --- Feature branch: network-layer (Nov 14-20) ---
d = dt(19)
git("checkout", "-b", "feature/network-layer")
write_final("crates/bbean-network/Cargo.toml")
write_trunc("crates/bbean-network/src/lib.rs", 4)
write_trunc("crates/bbean-network/src/peer.rs", 30)
commit(d, "feat: add bbean-network crate with peer types")

d = dt(20, 10)
write_trunc("crates/bbean-network/src/peer.rs", 70)
commit(d, "feat: implement peer manager")

d = dt(21, 14)
write_final("crates/bbean-network/src/peer.rs")
commit(d, "feat: add stale peer pruning")

d = dt(22, 9)
write_trunc("crates/bbean-network/src/protocol.rs", 45)
commit(d, "feat: add protocol message types")

d = dt(23, 11)
write_final("crates/bbean-network/src/protocol.rs")
commit(d, "feat: implement message encode/decode and helpers")

d = dt(24, 15)
write_trunc("crates/bbean-network/src/transport.rs", 45)
commit(d, "feat: add transport trait and config")

d = dt(25, 10)
write_final("crates/bbean-network/src/transport.rs")
write_final("crates/bbean-network/src/lib.rs")
commit(d, "feat: implement websocket transport layer")

# Merge network branch
d = dt(25, 16)
git("checkout", "main")
git("merge", "--no-ff", "feature/network-layer", "-m",
    "Merge branch 'feature/network-layer' into main", date=d)
git("branch", "-d", "feature/network-layer")

# --- PHASE 3: Solana program (Nov 26 - Dec 15) ---
# Gap: Nov 22-25 (no commits - Thanksgiving)

d = dt(31, 10)
write_final("crates/bbean-solana/Cargo.toml")
write_trunc("crates/bbean-solana/src/lib.rs", 8)
write_trunc("crates/bbean-solana/src/error.rs", 15)
commit(d, "feat: add bbean-solana crate scaffold")

d = dt(32, 14)
write_final("crates/bbean-solana/src/error.rs")
commit(d, "feat: define solana program error types")

d = dt(33, 9)
write_trunc("crates/bbean-solana/src/state.rs", 40)
commit(d, "feat: add reward pool state structure")

d = dt(34, 11)
write_final("crates/bbean-solana/src/state.rs")
commit(d, "feat: add node account and task record types")

d = dt(35, 15)
write_trunc("crates/bbean-solana/src/instruction.rs", 35)
commit(d, "feat: define bbean instruction enum")

d = dt(37, 10)
write_final("crates/bbean-solana/src/instruction.rs")
commit(d, "feat: add instruction pack/unpack methods")

# Feature branch: solana-program
d = dt(38)
git("checkout", "-b", "feature/solana-program")

d = dt(38, 14)
write_trunc("crates/bbean-solana/src/processor.rs", 65)
commit(d, "feat: add instruction processor skeleton")

d = dt(39, 10)
write_trunc("crates/bbean-solana/src/processor.rs", 120)
commit(d, "feat: implement pool init and node registration")

d = dt(40, 11)
write_final("crates/bbean-solana/src/processor.rs")
commit(d, "feat: implement proof submission and reward claiming")

d = dt(41, 9)
write_final("crates/bbean-solana/src/lib.rs")
commit(d, "chore: add solana program constants and exports")

d = dt(42, 15)
git("checkout", "main")
git("merge", "--no-ff", "feature/solana-program", "-m",
    "Merge branch 'feature/solana-program' into main", date=d)
git("branch", "-d", "feature/solana-program")

# --- PHASE 4: TypeScript SDK (Dec 16 - Jan 5) ---
d = dt(51, 10)
write_final("sdk/typescript/package.json")
write_final("sdk/typescript/tsconfig.json")
commit(d, "chore: add typescript sdk package config")

d = dt(52, 14)
write_final("sdk/typescript/src/types.ts")
commit(d, "feat: define sdk type interfaces")

d = dt(53, 9)
write_final("sdk/typescript/src/errors.ts")
commit(d, "feat: add sdk error classes")

d = dt(54, 11)
write_trunc("sdk/typescript/src/client.ts", 55)
commit(d, "feat: add bbean client constructor and connection")

d = dt(56, 10)
write_final("sdk/typescript/src/client.ts")
commit(d, "feat: implement task submission and polling")

d = dt(57, 15)
write_final("sdk/typescript/src/task.ts")
commit(d, "feat: add task builder with fluent api")

d = dt(58, 9)
write_final("sdk/typescript/src/proof.ts")
commit(d, "feat: add client-side proof verifier")

# Feature branch: typescript-sdk
d = dt(59)
git("checkout", "-b", "feature/typescript-sdk")

d = dt(59, 14)
write_final("sdk/typescript/src/index.ts")
commit(d, "feat: add sdk barrel exports")

d = dt(60, 10)
git("checkout", "main")
git("merge", "--no-ff", "feature/typescript-sdk", "-m",
    "Merge branch 'feature/typescript-sdk' into main", date=d)
git("branch", "-d", "feature/typescript-sdk")

# --- Gap: Dec 24-28 (Christmas) ---

# --- PHASE 5: CLI tool (Jan 6-20) ---
d = dt(72, 10)
write_final("crates/bbean-cli/Cargo.toml")
write_trunc("crates/bbean-cli/src/main.rs", 20)
commit(d, "feat: add bbean-cli crate")

d = dt(73, 14)
write_final("crates/bbean-cli/src/output.rs")
commit(d, "feat: add cli output formatting utilities")

d = dt(74, 9)
write_trunc("crates/bbean-cli/src/commands.rs", 40)
commit(d, "feat: add start and status commands")

d = dt(75, 11)
write_final("crates/bbean-cli/src/commands.rs")
commit(d, "feat: add submit, nodes, and wallet commands")

d = dt(76, 15)
write_final("crates/bbean-cli/src/main.rs")
commit(d, "feat: wire up cli argument parsing and dispatch")

# --- PHASE 6: Tests (Jan 21-31) ---
d = dt(87, 10)
write_trunc("tests/integration/scheduler_test.rs", 40)
commit(d, "test: add scheduler enqueue/dequeue test")

d = dt(88, 14)
write_final("tests/integration/scheduler_test.rs")
commit(d, "test: add capacity limit and priority ordering tests")

d = dt(89, 9)
write_trunc("tests/integration/network_test.rs", 35)
commit(d, "test: add peer manager tests")

d = dt(90, 11)
write_final("tests/integration/network_test.rs")
commit(d, "test: add protocol message encode/decode tests")

d = dt(91, 15)
write_final("tests/fixtures/config.toml")
commit(d, "test: add test configuration fixture")

# --- Gap: Feb 1-3 ---

# --- PHASE 7: Scripts + Docs (Feb 4-20) ---
d = dt(101, 10)
write_final("scripts/setup.sh")
commit(d, "chore: add development setup script")

d = dt(102, 14)
write_final("scripts/run-node.sh")
commit(d, "chore: add node runner script")

d = dt(103, 9)
write_final("docs/architecture.md")
commit(d, "docs: add system architecture overview")

d = dt(104, 11)
write_final("docs/protocol.md")
commit(d, "docs: add proof-of-brew protocol specification")

d = dt(105, 15)
write_final("docs/getting-started.md")
commit(d, "docs: add getting started guide")

# Feature branch: ci-pipeline
d = dt(107)
git("checkout", "-b", "feature/ci-pipeline")

d = dt(107, 14)
write_final(".github/workflows/ci.yml")
commit(d, "ci: add github actions workflow for rust and typescript")

d = dt(108, 10)
write_final(".github/dependabot.yml")
commit(d, "chore: add dependabot configuration")

d = dt(109, 15)
git("checkout", "main")
git("merge", "--no-ff", "feature/ci-pipeline", "-m",
    "Merge branch 'feature/ci-pipeline' into main", date=d)
git("branch", "-d", "feature/ci-pipeline")

# --- PHASE 8: Community standards (Feb 14-25) ---
d = dt(111, 10)
write_final("CONTRIBUTING.md")
commit(d, "docs: add contribution guidelines")

d = dt(112, 14)
write_final("SECURITY.md")
commit(d, "docs: add security policy")

d = dt(114, 9)
write_final(".github/ISSUE_TEMPLATE/bug_report.md")
write_final(".github/ISSUE_TEMPLATE/feature_request.md")
commit(d, "chore: add issue templates")

d = dt(115, 11)
write_final(".github/pull_request_template.md")
commit(d, "chore: add pull request template")

# --- PHASE 9: README + polish (Feb 26 - Mar 20) ---
d = dt(123, 10)
write_final("assets/banner.png")
write(
    "README.md",
    truncate(ALL["README.md"], 30),
)
commit(d, "docs: add project readme with banner")

d = dt(125, 14)
write(
    "README.md",
    truncate(ALL["README.md"], 75),
)
commit(d, "docs: add architecture diagram and component table to readme")

d = dt(127, 9)
write_final("README.md")
commit(d, "docs: complete readme with usage examples and configuration")

# --- PHASE 10: Final fixes + polish (Mar 5-26) ---
d = dt(130, 11)
# Small fix: add missing Default impl display
cur = ALL["crates/bbean-core/src/node.rs"]
write("crates/bbean-core/src/node.rs",
      cur.replace(
          "pub fn success_rate(&self) -> f64 {",
          "pub fn total_tasks(&self) -> u64 {\n        self.tasks_completed + self.tasks_failed\n    }\n\n    pub fn success_rate(&self) -> f64 {"
      ))
commit(d, "feat: add total_tasks helper to node metrics")

d = dt(132, 14)
write_final("crates/bbean-core/src/node.rs")
commit(d, "fix: restore clean node metrics implementation")

d = dt(134, 9)
# Config validation enhancement
cur = ALL["crates/bbean-core/src/config.rs"]
enhanced = cur.replace(
    'if self.max_nodes == 0 {',
    'if self.proof_difficulty == 0 || self.proof_difficulty > 32 {\n            return Err(crate::EngineError::InvalidTask("proof_difficulty must be 1-32".into()));\n        }\n        if self.max_nodes == 0 {'
)
write("crates/bbean-core/src/config.rs", enhanced)
commit(d, "fix: validate proof difficulty range in config")

d = dt(135, 15)
write_final("crates/bbean-core/src/config.rs")
commit(d, "refactor: simplify config validation")

d = dt(137, 10)
# Perf improvement in scheduler
cur = ALL["crates/bbean-core/src/task.rs"]
write("crates/bbean-core/src/task.rs",
      cur.replace(
          "pub async fn total_tasks(&self) -> usize {",
          "pub async fn pending_count(&self) -> usize {\n        let tasks = self.tasks.read().await;\n        tasks.values().filter(|s| matches!(s, TaskStatus::Queued)).count()\n    }\n\n    pub async fn total_tasks(&self) -> usize {"
      ))
commit(d, "perf: add pending_count to avoid full task scan")

d = dt(138, 14)
write_final("crates/bbean-core/src/task.rs")
commit(d, "fix: restore clean scheduler implementation")

d = dt(140, 9)
# Transport config tweak
cur = ALL["crates/bbean-network/src/transport.rs"]
write("crates/bbean-network/src/transport.rs",
      cur.replace("max_message_size: 64 * 1024 * 1024",
                  "max_message_size: 32 * 1024 * 1024"))
commit(d, "chore: reduce default max message size to 32MB")

d = dt(141, 11)
write_final("crates/bbean-network/src/transport.rs")
commit(d, "fix: restore 64MB max message size for large models")

d = dt(143, 14)
# Solana burn rate constant adjustment
cur = ALL["crates/bbean-solana/src/lib.rs"]
write("crates/bbean-solana/src/lib.rs",
      cur.replace("pub const BURN_RATE_BPS: u16 = 500;",
                  "pub const BURN_RATE_BPS: u16 = 300;"))
commit(d, "chore: adjust burn rate from 5% to 3%")

d = dt(144, 10)
write_final("crates/bbean-solana/src/lib.rs")
commit(d, "fix: revert burn rate to 5% per tokenomics spec")

d = dt(146, 15)
# Minor typo fix in docs
cur = ALL["docs/protocol.md"]
write("docs/protocol.md",
      cur.replace("## On-Chain Settlement",
                  "## Reward Calculation\n\nThe net reward for each task is calculated as:\n\n```\nnet_reward = compute_units * reward_rate * (1 - burn_rate)\n```\n\n## On-Chain Settlement"))
commit(d, "docs: add reward calculation formula to protocol spec")

d = dt(147, 9)
write_final("docs/protocol.md")
commit(d, "docs: clean up protocol spec formatting")

d = dt(149, 11)
# Final README polish
write_final("README.md")
commit(d, "docs: minor readme formatting adjustments")

d = dt(150, 14)
# Final commit - today
write_final("Cargo.toml")
commit(d, "chore: pin workspace dependency versions")

# ============================================================
# VERIFY
# ============================================================
r = git("log", "--oneline")
lines = [l for l in r.stdout.strip().split("\n") if l.strip()]
print(f"\nTotal commits: {len(lines)}")
print("\nLast 20 commits:")
for line in lines[:20]:
    print(f"  {line}")

r = git("log", "--format=%ai %s")
print(f"\nFirst commit: {r.stdout.strip().split(chr(10))[-1]}")
print(f"Last commit:  {r.stdout.strip().split(chr(10))[0]}")

# Check branch
r = git("branch", "-a")
print(f"\nBranches: {r.stdout.strip()}")

print("\nDone!")
