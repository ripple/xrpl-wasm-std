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

run_integration_test() {
    local dir="$1"
    local contract_name="$2"
    local wasm_file_release="$3"

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
}

if [[ $# -gt 0 ]]; then
    test_name="$1"
    test_dir="../examples/smart-escrows/$test_name"
    wasm_file_release="../examples/target/wasm32v1-none/release/${contract_name}.wasm"
    run_integration_test "$test_dir" "$test_name" "$wasm_file_release"
    exit 0
fi

# find ../examples -mindepth 2 -name "Cargo.toml" -type f | while read -r cargo_file; do
#     dir=$(dirname "$cargo_file")
#     contract_name=$(basename "$dir")
#     wasm_file_release="../examples/target/wasm32v1-none/release/${contract_name}.wasm"
#     run_integration_test "$dir" "$contract_name" "$wasm_file_release"
# done

find ../e2e-tests -mindepth 1 -name "Cargo.toml" -type f | while read -r cargo_file; do
    dir=$(dirname "$cargo_file")
    contract_name=$(basename "$dir")
    wasm_file_release="../e2e-tests/target/wasm32v1-none/release/${contract_name}.wasm"
    if [[ -f "$dir/run_test.js" ]]; then
        run_integration_test "$dir" "$contract_name" "$wasm_file_release"
    fi
done

echo "✅ End-to-end tests completed successfully!"
