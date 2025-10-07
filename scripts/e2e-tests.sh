#!/bin/bash
# End-to-end tests script
# Mirrors the e2e-tests job from GitHub Actions

set -euo pipefail

# Change to the repository root directory (where this script's grandparent directory is located)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

echo "🔧 Running end-to-end tests..."

# Ensure wasm32 target is available
echo "📦 Ensuring wasm32v1-none target is installed..."
rustup target add wasm32v1-none

echo "🏗️  Building projects..."
scripts/build.sh
scripts/build.sh release

echo "🧪 Running integration tests..."
find projects -name "Cargo.toml" -type f | while read -r cargo_file; do
    dir=$(dirname "$cargo_file")
    contract_name=$(basename "$dir")
    if [ -d "$dir/fixtures" ]; then
        echo "🔧 Running integration test for $contract_name in $dir"
        cargo run --package wasm-host-simulator --bin wasm-host-simulator -- -p "$contract_name" --dir $dir || exit 1
    fi
done

echo "✅ End-to-end tests completed successfully!"
