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

echo "🏗️  Building examples..."
scripts/build.sh
scripts/build.sh release

echo "🧪 Running integration tests..."
cd tests
if [[ "${CI:-}" == "true" || -n "${CI:-}" ]]; then
    node setup_ledger.js "wss://wasm.devnet.rippletest.net:51233"
else
    node setup_ledger.js
fi
find ../examples -name "Cargo.toml" -type f | while read -r cargo_file; do
    dir=$(dirname "$cargo_file")
    contract_name=$(basename "$dir")
    wasm_file_release="../examples/target/wasm32v1-none/release/${contract_name}.wasm"
    if [[ ! -f "$dir/run_test.js" ]]; then
        echo "❌ Error: Test file run_test.js not found in $dir"
        exit 1
    fi
    echo "🔧 Running integration test for $contract_name in $dir"
    if [[ "${CI:-}" == "true" || -n "${CI:-}" ]]; then
        node ./run_single_test.js "$dir" "$wasm_file_release" "wss://wasm.devnet.rippletest.net:51233"
    else
        node ./run_single_test.js "$dir" "$wasm_file_release"
    fi
done

echo "✅ End-to-end tests completed successfully!"
