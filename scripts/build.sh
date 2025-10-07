#!/bin/bash
# Build script - replaces the missing build.sh referenced in e2e-tests
# Based on the build steps from build_and_test job

set -euo pipefail

# Change to the repository root directory (where this script's parent directory is located)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

# Parse command line arguments
RELEASE_MODE=""
if [[ "${1:-}" == "release" ]]; then
    RELEASE_MODE="--release"
    echo "🔧 Building in release mode..."
else
    echo "🔧 Building in debug mode..."
fi

# Ensure wasm32 target is available
echo "📦 Ensuring wasm32v1-none target is installed..."
rustup target add wasm32v1-none

echo "🏗️  Building Native Workspace..."
cargo build --workspace $RELEASE_MODE

echo "🏗️  Building xrpl-wasm-std for WASM..."
cargo build -p xrpl-wasm-std --target wasm32v1-none $RELEASE_MODE
cargo rustc -p xrpl-wasm-std --target wasm32v1-none $RELEASE_MODE -- -D warnings

echo "🏗️  Building WASM Examples Workspace..."
cd examples
echo "🔧 Building examples workspace for WASM"
if [[ -n "$RELEASE_MODE" ]]; then
    # Only build release if specifically requested
    cargo build --workspace --target wasm32v1-none $RELEASE_MODE
else
    # Build both debug and release when no specific mode is requested
    cargo build --workspace --target wasm32v1-none
    cargo build --workspace --target wasm32v1-none --release
fi
cd ..

echo "✅ Build completed successfully!"
