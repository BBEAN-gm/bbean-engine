#!/bin/bash
set -euo pipefail

CONFIG="${BBEAN_CONFIG:-config.json}"
LOG_LEVEL="${BBEAN_LOG:-info}"

export RUST_LOG="bbean=$LOG_LEVEL"

echo "Starting BBEAN node..."
echo "Config: $CONFIG"
echo "Log level: $LOG_LEVEL"

cargo run --release -p bbean-cli -- start
