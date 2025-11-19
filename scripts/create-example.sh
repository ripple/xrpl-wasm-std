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
    echo "  project-path          Path where the project should be created (e.g., my_project or examples/smart-escrows/my_project)"
    echo ""
    echo "Options:"
    echo "  -d, --description     Project description (default: 'TODO: Add description')"
    echo "  -v, --version         xrpl-wasm-stdlib version (default: latest from crates.io)"
    echo "  --local               Use local path dependency (for development within the repo)"
    echo "  -h, --help           Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 my_project"
    echo "  $0 my_project -d 'A custom smart escrow'"
    echo "  $0 examples/smart-escrows/my_project --local"
    exit 1
}

# Check if no arguments provided
if [ $# -eq 0 ]; then
    usage
fi

# Parse arguments
PROJECT_PATH=""
DESCRIPTION="TODO: Add description"
VERSION=""
USE_LOCAL=false

while [[ $# -gt 0 ]]; do
    case $1 in
        -d|--description)
            DESCRIPTION="$2"
            shift 2
            ;;
        -v|--version)
            VERSION="$2"
            shift 2
            ;;
        --local)
            USE_LOCAL=true
            shift
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

# Determine if we're running from within the repo or standalone
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" 2>/dev/null && pwd || pwd)"
if [ -f "$SCRIPT_DIR/../xrpl-wasm-stdlib/Cargo.toml" ]; then
    REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
    IN_REPO=true
else
    REPO_ROOT=""
    IN_REPO=false
fi

# Convert to absolute path if relative
if [[ "$PROJECT_PATH" != /* ]]; then
    if [ "$IN_REPO" = true ] && [ "$USE_LOCAL" = true ]; then
        # Only use REPO_ROOT if we're in the repo AND using local dependencies
        PROJECT_PATH="$REPO_ROOT/$PROJECT_PATH"
    else
        # Use current working directory for standalone projects
        PROJECT_PATH="$(pwd)/$PROJECT_PATH"
    fi
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

# Determine dependency configuration
if [ "$USE_LOCAL" = true ] && [ "$IN_REPO" = true ]; then
    # Calculate relative path from project to xrpl-wasm-stdlib
    RELATIVE_PATH=$(python3 -c "import os.path; print(os.path.relpath('$REPO_ROOT/xrpl-wasm-stdlib', '$PROJECT_PATH'))")
    DEPENDENCY_LINE="xrpl-wasm-stdlib = { path = \"$RELATIVE_PATH\" }"
elif [ "$USE_LOCAL" = true ] && [ "$IN_REPO" = false ]; then
    echo -e "${YELLOW}Warning: --local flag ignored (not in repository). Using crates.io version.${NC}"
    if [ -n "$VERSION" ]; then
        DEPENDENCY_LINE="xrpl-wasm-stdlib = \"$VERSION\""
    else
        DEPENDENCY_LINE="xrpl-wasm-stdlib = \"0.7.1\""
    fi
else
    # Use crates.io version
    if [ -n "$VERSION" ]; then
        DEPENDENCY_LINE="xrpl-wasm-stdlib = \"$VERSION\""
    else
        DEPENDENCY_LINE="xrpl-wasm-stdlib = \"0.7.1\""
    fi
fi

echo -e "${GREEN}Creating new XRPL WASM example project...${NC}"
echo -e "  Name: ${YELLOW}$PROJECT_NAME${NC}"
echo -e "  Path: ${YELLOW}$PROJECT_PATH${NC}"
echo -e "  Description: ${YELLOW}$DESCRIPTION${NC}"
echo -e "  Dependency: ${YELLOW}$DEPENDENCY_LINE${NC}"
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
$DEPENDENCY_LINE
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

# Create README.md with appropriate instructions
if [ "$USE_LOCAL" = true ] && [ "$IN_REPO" = true ]; then
    # In-repo version with test script
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
else
    # Standalone version
    cat > "$PROJECT_PATH/README.md" << EOF
# $PROJECT_NAME

$DESCRIPTION

## Prerequisites

- Rust toolchain with \`wasm32v1-none\` target
- Node.js 18+

## Quick Setup

### 1. Install Rust and WASM target

\`\`\`shell
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32v1-none
\`\`\`

### 2. Install Node.js dependencies

\`\`\`shell
npm install
\`\`\`

### 3. Build the WASM

\`\`\`shell
cargo build --target wasm32v1-none --release
\`\`\`

Your compiled WASM will be at:

\`\`\`
./target/wasm32v1-none/release/${PROJECT_NAME}.wasm
\`\`\`

### 4. Test your contract

\`\`\`shell
node runTest.js
\`\`\`

This will:

- Connect to WASM Devnet (\`wss://wasm.devnet.rippletest.net:51233\`)
- Create and fund two test wallets (Origin and Destination)
- Create an EscrowCreate transaction with your compiled \`FinishFunction\`
- Finish the escrow, executing your WASM code

Expected result: \`tesSUCCESS\` and "Escrow finished successfully!".

## Testing on the Web UI

You can also test your contract using the web interface:

1. Build your contract: \`cargo build --target wasm32v1-none --release\`
2. Open https://ripple.github.io/xrpl-wasm-stdlib/ui/
3. Upload your WASM file from \`target/wasm32v1-none/release/${PROJECT_NAME}.wasm\`
4. Configure test scenarios and execute

## Next Steps

- Edit \`src/lib.rs\` to implement your smart escrow logic
- See the [Complete Developer Guide](https://ripple.github.io/xrpl-wasm-stdlib/xrpl_wasm_stdlib/guide/index.html)
- Explore [example contracts](https://github.com/ripple/xrpl-wasm-stdlib/tree/main/examples/smart-escrows)
EOF
fi

echo -e "${GREEN}✓ Project created successfully!${NC}"
echo ""
echo "Next steps:"
echo "  1. cd $PROJECT_PATH"
echo "  2. Edit src/lib.rs to implement your logic"
echo "  3. cargo build --target wasm32v1-none --release"
if [ "$USE_LOCAL" = true ] && [ "$IN_REPO" = true ]; then
    echo "  4. CI=1 $REPO_ROOT/scripts/run-tests.sh ${PROJECT_PATH#$REPO_ROOT/}"
else
    echo "  4. node runTest.js"
fi
echo ""
echo -e "${YELLOW}Files created:${NC}"
echo "  - Cargo.toml"
echo "  - src/lib.rs"
echo "  - runTest.js"
echo "  - README.md"
