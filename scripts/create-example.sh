#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to display usage
usage() {
    echo "Usage: $0 <project-path> [options]"
    echo ""
    echo "Creates a new XRPL WASM example project with the standard structure."
    echo ""
    echo "Arguments:"
    echo "  project-path          Path where the project should be created (e.g., examples/smart-escrows/my_project)"
    echo ""
    echo "Options:"
    echo "  -d, --description     Project description (default: 'TODO: Add description')"
    echo "  -h, --help           Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 examples/smart-escrows/my_project"
    echo "  $0 examples/smart-escrows/my_project -d 'A custom smart escrow'"
    echo "  $0 examples/my-category/my_project"
    exit 1
}

# Check if no arguments provided
if [ $# -eq 0 ]; then
    usage
fi

# Parse arguments
PROJECT_PATH=""
DESCRIPTION="TODO: Add description"

while [[ $# -gt 0 ]]; do
    case $1 in
        -d|--description)
            DESCRIPTION="$2"
            shift 2
            ;;
        -h|--help)
            usage
            ;;
        -*)
            echo -e "${RED}Error: Unknown option $1${NC}"
            usage
            ;;
        *)
            if [ -z "$PROJECT_PATH" ]; then
                PROJECT_PATH="$1"
            else
                echo -e "${RED}Error: Multiple project paths specified${NC}"
                usage
            fi
            shift
            ;;
    esac
done

# Validate project path
if [ -z "$PROJECT_PATH" ]; then
    echo -e "${RED}Error: Project path is required${NC}"
    usage
fi

# Get the script directory and repository root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Convert to absolute path if relative
if [[ "$PROJECT_PATH" != /* ]]; then
    PROJECT_PATH="$REPO_ROOT/$PROJECT_PATH"
fi

# Extract project name from path
PROJECT_NAME=$(basename "$PROJECT_PATH")

# Validate project name (must be valid Rust crate name)
if ! [[ "$PROJECT_NAME" =~ ^[a-zA-Z][a-zA-Z0-9_]*$ ]]; then
    echo -e "${RED}Error: Project name must start with a letter and contain only letters, numbers, and underscores${NC}"
    exit 1
fi

# Check if project already exists
if [ -d "$PROJECT_PATH" ]; then
    echo -e "${RED}Error: Project directory already exists: $PROJECT_PATH${NC}"
    exit 1
fi

# Calculate relative path from project to xrpl-wasm-stdlib
PROJECT_DIR=$(dirname "$PROJECT_PATH")
RELATIVE_PATH=$(python3 -c "import os.path; print(os.path.relpath('$REPO_ROOT/xrpl-wasm-stdlib', '$PROJECT_PATH'))")

echo -e "${GREEN}Creating new XRPL WASM example project...${NC}"
echo -e "  Name: ${YELLOW}$PROJECT_NAME${NC}"
echo -e "  Path: ${YELLOW}$PROJECT_PATH${NC}"
echo -e "  Description: ${YELLOW}$DESCRIPTION${NC}"
echo ""

# Create project directory structure
mkdir -p "$PROJECT_PATH/src"

# Create Cargo.toml
cat > "$PROJECT_PATH/Cargo.toml" << EOF
[package]
name = "$PROJECT_NAME"
version = "1.0.0"
edition = "2024"
description = "$DESCRIPTION"
license = "ISC"

[lib]
crate-type = ["cdylib"]

[dependencies]
xrpl-wasm-stdlib = { path = "$RELATIVE_PATH" }
EOF

# Create src/lib.rs
cat > "$PROJECT_PATH/src/lib.rs" << 'EOF'
#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_wasm_stdlib::host::trace::trace;

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let _ = trace("TODO: Implement your logic here");

    // Return 1 to allow the escrow to finish
    // Return 0 to deny the escrow finish
    1
}
EOF

# Create runTest.js
cat > "$PROJECT_PATH/runTest.js" << 'EOF'
async function test(testContext) {
  const { deploy, finish, submit, sourceWallet, destWallet } = testContext

  const offerSequence = await deploy(sourceWallet, destWallet, finish)

  const tx = {
    TransactionType: "EscrowFinish",
    Account: sourceWallet.address,
    Owner: sourceWallet.address,
    OfferSequence: parseInt(offerSequence),
    ComputationAllowance: 1000000,
  }

  const response = await submit(tx, sourceWallet)

  if (response.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to finish escrow:",
      response.result.meta.TransactionResult,
    )
    process.exit(1)
  }
}

module.exports = { test }
EOF

# Create README.md
cat > "$PROJECT_PATH/README.md" << EOF
# $PROJECT_NAME

$DESCRIPTION

## Prerequisites

- Rust toolchain with \`wasm32v1-none\` target
- Node.js 18+

## Step-by-step: Use on WASM Devnet

This guide uses the public Devnet WASM endpoint at \`wss://wasm.devnet.rippletest.net:51233\`.

### 1. Install dependencies

\`\`\`shell
npm install
\`\`\`

### 2. Build the WASM

\`\`\`shell
cargo build --target wasm32v1-none --release
\`\`\`

Artifact:

\`\`\`
./target/wasm32v1-none/release/${PROJECT_NAME}.wasm
\`\`\`

### 3. Deploy and test on Devnet

Use the test script to deploy an escrow and test the FinishFunction.

\`\`\`shell
cd $REPO_ROOT
CI=1 ./scripts/run-tests.sh ${PROJECT_PATH#$REPO_ROOT/}
\`\`\`

This will:

- Connect to WASM Devnet
- Create and fund two wallets (Origin and Destination)
- Create an EscrowCreate transaction with your compiled \`FinishFunction\`
- Finish the escrow, executing the WASM

Expected result: \`tesSUCCESS\` and "Escrow finished successfully!".
EOF

echo -e "${GREEN}✓ Project created successfully!${NC}"
echo ""
echo "Next steps:"
echo "  1. cd $PROJECT_PATH"
echo "  2. Edit src/lib.rs to implement your logic"
echo "  3. cargo build --target wasm32v1-none --release"
echo "  4. CI=1 $REPO_ROOT/scripts/run-tests.sh ${PROJECT_PATH#$REPO_ROOT/}"
echo ""
echo -e "${YELLOW}Files created:${NC}"
echo "  - Cargo.toml"
echo "  - src/lib.rs"
echo "  - runTest.js"
echo "  - README.md"
