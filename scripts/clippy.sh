#!/bin/bash
# Clippy linting script
# Mirrors the clippy_linting job from GitHub Actions

set -euo pipefail

# Change to the repository root directory (where this script's grandparent directory is located)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

echo "🔧 Running Clippy linting..."

# Ensure wasm32 target is available
echo "📦 Ensuring wasm32v1-none target is installed..."
rustup target add wasm32v1-none

echo "🔍 Running Clippy on Native Workspace..."
cargo clippy --workspace --all-targets --all-features -- -Dclippy::all

echo "🔍 Running Clippy on WASM Examples Workspace..."
cd examples
cargo clippy --workspace --target wasm32v1-none --all-features -- -Dclippy::all
cd ..

echo "🔍 Running Clippy on E2E Tests Workspace..."
cd e2e-tests
cargo clippy --workspace --target wasm32v1-none --all-features -- -Dclippy::all
cd ..

echo "✅ Clippy linting passed!"
