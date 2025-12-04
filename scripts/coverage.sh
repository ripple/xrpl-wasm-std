#!/bin/bash
set -e

# Coverage script for xrpl-wasm-stdlib
#
# This script runs the e2e-tests with coverage instrumentation to measure
# which parts of xrpl-wasm-stdlib are exercised by the integration tests.
#
# Requirements:
#   - cargo-llvm-cov: Install with `cargo install cargo-llvm-cov`
#
# Usage:
#   ./scripts/coverage.sh
#
# Output:
#   - HTML report: target/llvm-cov/html/index.html
#   - LCOV report: target/llvm-cov/lcov.info
#   - Console summary showing coverage percentages

echo "=== xrpl-wasm-stdlib Coverage Report ==="
echo ""

# Check if cargo-llvm-cov is installed
if ! command -v cargo-llvm-cov &> /dev/null; then
    echo "Error: cargo-llvm-cov is not installed"
    echo "Install it with: cargo install cargo-llvm-cov"
    exit 1
fi

echo "Running coverage for e2e-tests..."
echo ""

# Change to e2e-tests directory
cd e2e-tests

# Clean previous coverage data
cargo llvm-cov clean --workspace

# Run tests with coverage instrumentation and display summary in terminal
# --workspace: Include all workspace members
# --ignore-filename-regex: Exclude test files from coverage report
# --summary-only: Show only the summary in terminal (no HTML/LCOV generation)
echo "Running tests with coverage instrumentation..."
echo ""
cargo llvm-cov \
   --all-features \
    --workspace \
    --ignore-filename-regex "e2e-tests/.*" \
    -- --nocapture

echo ""
echo "=== Coverage Summary ==="
echo ""
echo "The above shows coverage for xrpl-wasm-stdlib exercised by e2e-tests."
echo ""
echo "To generate HTML report, run:"
echo "  cd e2e-tests && cargo llvm-cov --workspace --html --ignore-filename-regex 'e2e-tests/.*'"
echo ""
echo "To generate LCOV report, run:"
echo "  cd e2e-tests && cargo llvm-cov --workspace --lcov --output-path ../target/llvm-cov/lcov.info --ignore-filename-regex 'e2e-tests/.*'"
