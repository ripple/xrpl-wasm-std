#!/bin/bash
# Build and test script
# Mirrors the build_and_test job from GitHub Actions

set -euo pipefail

# Change to the repository root directory (where this script's parent directory is located)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

echo "🔧 Running build and test workflow..."

# Parse command line arguments for release mode
RELEASE_ARG=""
if [[ "${1:-}" == "release" ]]; then
    RELEASE_ARG="release"
    echo "🔧 Running in release mode..."
fi

echo "🏗️  Building examples..."
# Use the dedicated build script for consistency
./scripts/build.sh $RELEASE_ARG

echo "🧪 Running native workspace tests..."
# Run tests on the native workspace
cargo test --workspace

echo "✅ Build and test workflow completed successfully!"
