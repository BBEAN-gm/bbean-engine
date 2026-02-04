#!/bin/bash
set -euo pipefail

echo "Setting up BBEAN Engine development environment..."

if ! command -v cargo &> /dev/null; then
    echo "Rust is not installed. Install from https://rustup.rs"
    exit 1
fi

echo "Building workspace..."
cargo build --workspace

echo "Running tests..."
cargo test --workspace

if command -v node &> /dev/null; then
    echo "Building TypeScript SDK..."
    cd sdk/typescript
    npm install
    npm run build
    cd ../..
else
    echo "Node.js not found, skipping SDK build"
fi

echo "Setup complete."
