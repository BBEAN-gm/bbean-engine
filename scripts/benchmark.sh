#!/bin/bash
set -euo pipefail

echo "Running BBEAN Engine benchmarks..."
echo ""

echo "=== Scheduler benchmark ==="
cargo bench -p bbean-core --bench scheduler 2>/dev/null || echo "No scheduler benchmarks found"

echo ""
echo "=== Proof validation benchmark ==="
cargo bench -p bbean-core --bench proof 2>/dev/null || echo "No proof benchmarks found"

echo ""
echo "Benchmarks complete."
