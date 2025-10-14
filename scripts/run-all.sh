#!/bin/bash
# Run all tests script
# Runs all the test jobs in sequence, mirroring the complete GitHub Actions workflow
# Usage: ./scripts/run-all.sh [--e2e]
#   --e2e: Include end-to-end tests (skipped by default)

set -euo pipefail

# Parse command line arguments
RUN_E2E=false
for arg in "$@"; do
    case $arg in
        --e2e|--with-e2e)
            RUN_E2E=true
            shift
            ;;
        *)
            echo "Unknown option: $arg"
            echo "Usage: $0 [--e2e]"
            exit 1
            ;;
    esac
done

# Change to the repository root directory (where this script's parent directory is located)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

echo "🚀 Running complete Build and Test suite..."
if [[ "$RUN_E2E" == "true" ]]; then
    echo "   (including e2e tests)"
else
    echo "   (skipping e2e tests - use --e2e to include them)"
fi
echo ""

# Track start time
start_time=$(date +%s)

# Function to run a script and report results
run_script() {
    local script_name="$1"
    local script_path="./scripts/$script_name"
    echo "=================================================="
    echo "🔧 Running $script_name..."
    echo "=================================================="
    if [[ -x "$script_path" ]]; then
        if "$script_path"; then
            echo "✅ $script_name completed successfully"
        else
            echo "❌ $script_name failed"
            exit 1
        fi
    else
        echo "❌ Script $script_path not found or not executable"
        exit 1
    fi
    echo ""
}

# Run all test scripts in order
# Note: pre-commit checks are handled by GitHub Actions, not locally
run_script "/setup.sh"
run_script "/clippy.sh"
run_script "/fmt.sh"
run_script "/host-function-audit.sh"
run_script "/check-wasm-exports.sh"
run_script "/build-and-test.sh"
run_script "/run-markdown.sh"

# Run e2e tests only if requested
if [[ "$RUN_E2E" == "true" ]]; then
    run_script "/e2e-tests.sh"
else
    echo "=================================================="
    echo "⏭️  Skipping e2e-tests.sh (use --e2e to include)"
    echo "=================================================="
    echo ""
fi

# Calculate and display total time
end_time=$(date +%s)
duration=$((end_time - start_time))
minutes=$((duration / 60))
seconds=$((duration % 60))

echo "=================================================="
echo "🎉 All tests completed successfully!"
echo "⏱️  Total time: ${minutes}m ${seconds}s"
echo "=================================================="
