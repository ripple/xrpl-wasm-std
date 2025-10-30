#!/bin/bash

# Gas Benchmark Harness
#
# This script measures and compares gas costs of stdlib helper functions
# between different branches (e.g., main vs optimizations).
#
# Benchmarked functions:
#   - Transaction field access (get_account, get_fee)
#   - Error code matching (match_result_code, match_result_code_optional, etc.)
#   - Result type methods (is_ok, is_err, ok, err)
#   - Hex decoding (decode_hex_32, decode_hex_20)
#
# Usage:
#   ./scripts/benchmark-gas.sh              # Benchmark current branch
#   node tools/gas_benchmark.js --branch=main  # Benchmark specific branch
#   node tools/compare_gas_results.js       # Generate comparison report
#
# Results are stored in .benchmark/ (gitignored)
#
# To add new benchmarks, see e2e-tests/gas_benchmark/README.md

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
TOOLS_DIR="$PROJECT_ROOT/tools"
E2E_TESTS_DIR="$PROJECT_ROOT/e2e-tests"

./scripts/build.sh

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== Gas Benchmark Harness ===${NC}"
echo ""

# Check if rippled is running
echo -e "${BLUE}Checking for local rippled instance...${NC}"
if ! nc -z 127.0.0.1 6006 2>/dev/null; then
    echo -e "${RED}Error: Local rippled instance not found at ws://127.0.0.1:6006${NC}"
    echo "Please start a local rippled instance before running this script."
    exit 1
fi
echo -e "${GREEN}✓ Local rippled instance found${NC}"
echo ""

# Phase 1: Measure gas for optimizations branch
echo -e "${BLUE}Phase 1: Measuring gas for optimizations branch...${NC}"
node "$TOOLS_DIR/gas_benchmark.js"
echo -e "${GREEN}✓ Phase 1 complete${NC}"
echo ""

# Phase 2: Generate comparison report
echo -e "${BLUE}Phase 2: Generating comparison report...${NC}"
node "$TOOLS_DIR/compare_gas_results.js"
echo -e "${GREEN}✓ Phase 2 complete${NC}"
echo ""

echo -e "${GREEN}=== Gas Benchmark Complete ===${NC}"
echo ""
echo "Results saved to:"
echo "  - $PROJECT_ROOT/.benchmark/gas_benchmark_results.json"
echo "  - $PROJECT_ROOT/.benchmark/GAS_BENCHMARK_REPORT.md"
