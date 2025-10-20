# XRPL WebAssembly Standard Library - Complete Guide

This comprehensive guide covers everything you need to develop smart escrows using the XRPL WebAssembly Standard Library.

## Table of Contents

- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Your First Contract](#your-first-contract)
  - [Core Concepts](#core-concepts)
- [API Reference](#api-reference)
  - [Transaction Access](#transaction-access)
  - [Ledger Objects](#ledger-objects)
  - [Type System](#type-system)
  - [Host Functions](#host-functions)
  - [Error Handling](#error-handling)
- [Examples](#examples)
  - [Hello World](#hello-world)
  - [Oracle Example](#oracle-example)
  - [KYC Example](#kyc-example)
  - [Advanced Examples](#advanced-examples)
- [Development Guide](#development-guide)
  - [Build System](#build-system)
  - [Testing](#testing)
  - [Performance Optimization](#performance-optimization)
  - [Troubleshooting](#troubleshooting)
- [Contributing](#contributing)
  - [Code Standards](#code-standards)
  - [Pull Request Process](#pull-request-process)
  - [Development Workflow](#development-workflow)

---

## Getting Started

### Prerequisites

Before building smart escrows, ensure you have:

1. **Rust toolchain** (stable or nightly)
2. **WASM target** (`wasm32v1-none`)
3. **Node.js** (for testing tools)
4. **Basic understanding** of XRPL concepts

**Quick setup:**

```shell
# Run the automated setup script
./scripts/setup.sh

# Or install manually:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32v1-none
npm install
```

### Installation

1. **Clone the repository:**

   ```shell
   git clone https://github.com/XRPLF/xrpl-wasm-std.git
   cd xrpl-wasm-std
   ```

2. **Run setup script:**

   ```shell
   ./scripts/setup.sh
   ```

3. **Verify installation:**
   ```shell
   ./scripts/run-tests.sh examples/smart-escrows/hello_world
   ```

### Your First Contract

Let's create a simple escrow that releases funds when an account balance exceeds 10 XRP:

```rust
#![no_std]
#![no_main]

use xrpl_wasm_std::core::current_tx::escrow_finish::EscrowFinish;
use xrpl_wasm_std::core::ledger_objects::account::get_account_balance;

#[no_mangle]
pub extern "C" fn finish() -> i32 {
    let tx = EscrowFinish;

    // Get the account trying to finish the escrow
    let account = match tx.get_account() {
        Ok(acc) => acc,
        Err(_) => return 0, // Invalid transaction
    };

    // Check account balance
    match get_account_balance(&account) {
        Ok(balance) if balance > 10_000_000 => 1, // Release (>10 XRP)
        _ => 0, // Keep locked
    }
}
```

**Build and test:**

```shell
# Create new project
mkdir my-escrow && cd my-escrow

# Add the contract code above to src/lib.rs
# Configure Cargo.toml:

[package]
name = "my-escrow"
version = "0.1.0"
edition = "2021"

[dependencies]
xrpl-wasm-std = { path = "../xrpl-wasm-std" }

[lib]
crate-type = ["cdylib"]

[profile.release]
opt-level = "s"
lto = true

# Build the contract
cargo build --target wasm32v1-none --release

# Test with provided tools
node ../examples/smart-escrows/hello_world/run_test.js
```

### Core Concepts

#### Smart Escrow Basics

Smart escrows are **conditional payment contracts** that:

- Lock XRP between parties
- Execute custom logic to determine release conditions
- Run deterministically across all network validators
- Have read-only access to ledger data

#### Contract Structure

Every smart escrow must:

1. **Export a `finish()` function** with signature `extern "C" fn finish() -> i32`
2. **Return 1 to release** funds or **0 to keep locked**
3. **Be deterministic** - same inputs always produce same outputs
4. **Use `#![no_std]`** - no standard library available

#### Host Environment

Smart escrows run in a constrained WebAssembly environment:

- **No heap allocation** - stack-based memory only
- **No file system or network** access
- **Limited execution time** and memory
- **Read-only ledger access** (except for escrow state updates)

---

## API Reference

### Transaction Access

The XRPL WASM Standard Library provides type-safe access to transaction data through the `current_tx` module.

#### EscrowFinish Transaction

```rust
use xrpl_wasm_std::core::current_tx::escrow_finish::EscrowFinish;

let tx = EscrowFinish;

// Get the account finishing the escrow
let account = tx.get_account()?;

// Get the destination account (receives funds if released)
let destination = tx.get_destination()?;

// Get the escrow sequence number
let escrow_sequence = tx.get_escrow_sequence()?;
```

#### Field Access

```rust
use xrpl_wasm_std::core::current_tx::CurrentTx;
use xrpl_wasm_std::sfield;

// Generic field access
let amount = tx.get_field_value::<u64>(sfield::Amount)?;
let account_id = tx.get_field_id(sfield::Account)?;

// Check if optional field exists
if tx.has_field(sfield::DestinationTag) {
    let tag = tx.get_field_value::<u32>(sfield::DestinationTag)?;
}
```

### Ledger Objects

Access current ledger state through the `ledger_objects` module.

#### Account Information

```rust
use xrpl_wasm_std::core::ledger_objects::account::{
    get_account_balance, get_account_info, get_account_root
};
use xrpl_wasm_std::types::AccountID;

let account = AccountID::from([0u8; 20]); // Replace with real account

// Get XRP balance (in drops)
let balance = get_account_balance(&account)?;

// Get full account root object
let account_root = get_account_root(&account)?;

// Extract specific account info
let sequence = get_account_info(&account, sfield::Sequence)?;
let owner_count = get_account_info(&account, sfield::OwnerCount)?;
```

#### Trust Lines

```rust
use xrpl_wasm_std::core::ledger_objects::trustline::{
    get_trustline, get_trustline_balance
};
use xrpl_wasm_std::types::{AccountID, Currency};

let account = AccountID::from([0u8; 20]);
let issuer = AccountID::from([1u8; 20]);
let currency = Currency::from_str("USD").unwrap();

// Get trust line balance
let balance = get_trustline_balance(&account, &issuer, &currency)?;

// Get full trust line object
let trustline = get_trustline(&account, &issuer, &currency)?;
```

#### NFT Objects

```rust
use xrpl_wasm_std::core::ledger_objects::nft::{
    get_nft_page, check_nft_ownership
};
use xrpl_wasm_std::types::{AccountID, NFTID};

let owner = AccountID::from([0u8; 20]);
let nft_id = NFTID::from([0u8; 32]);

// Check if account owns specific NFT
let owns_nft = check_nft_ownership(&owner, &nft_id)?;

// Get NFT page containing the NFT
let nft_page = get_nft_page(&owner, &nft_id)?;
```

### Type System

#### Core Types

```rust
use xrpl_wasm_std::types::{
    AccountID,    // 20-byte XRPL account identifier
    Amount,       // XRP amount in drops
    Currency,     // 3-character currency code
    Hash256,      // 32-byte hash
    NFTID,        // 32-byte NFT identifier
    Keylet,       // Ledger object locator
};

// Create AccountID from r-address
let account = xrpl_wasm_std::r_address!("rN7n7otQDd6FczFgLdSqtcsAUxDkw6fzRH");

// Create from raw bytes
let account = AccountID::from([0u8; 20]);

// Currency from string
let currency = Currency::from_str("USD")?;

// Hash from hex string
let hash = Hash256::from_hex("1234567890abcdef...")?;
```

#### Keylet Generation

Keylets are used to locate objects in the ledger:

```rust
use xrpl_wasm_std::core::keylets::{
    account_root_keylet,
    trustline_keylet,
    escrow_keylet,
    nft_page_keylet,
};

// Account root keylet
let keylet = account_root_keylet(&account);

// Trust line keylet
let keylet = trustline_keylet(&account, &issuer, &currency);

// Escrow keylet
let keylet = escrow_keylet(&account, &destination, sequence);

// NFT page keylet (simplified)
let keylet = nft_page_keylet(&account, &nft_id);
```

### Host Functions

Low-level host function access through the `host` module.

#### Ledger Access

```rust
use xrpl_wasm_std::host::ledger::{
    cache_ledger_obj, get_field_from_slot, does_obj_exist
};
use xrpl_wasm_std::sfield;

// Cache a ledger object for efficient access
let slot = cache_ledger_obj(&keylet)?;

// Extract fields from cached object
let balance = get_field_from_slot::<u64>(slot, sfield::Balance)?;
let sequence = get_field_from_slot::<u32>(slot, sfield::Sequence)?;

// Check if object exists without caching
if does_obj_exist(&keylet)? {
    // Object exists in ledger
}
```

#### Transaction Fields

```rust
use xrpl_wasm_std::host::transaction::{
    get_field, get_field_id, has_field
};

// Get transaction field value
let mut buffer = [0u8; 32];
let length = get_field(sfield::Account, 0, &mut buffer)?;
let account = AccountID::from_slice(&buffer[..length])?;

// Check if field exists
if has_field(sfield::DestinationTag) {
    let tag = get_field::<u32>(sfield::DestinationTag, 0)?;
}
```

#### Utility Functions

```rust
use xrpl_wasm_std::host::trace::{trace, trace_data, DataRepr};
use xrpl_wasm_std::host::crypto::{sha256, ripemd160};

// Debug tracing (disabled in production)
trace("Processing escrow finish")?;
trace_data("Account", &account_bytes, DataRepr::AsHex)?;

// Cryptographic functions
let hash = sha256(&input_data)?;
let address_hash = ripemd160(&pubkey_hash)?;
```

### Error Handling

The library uses custom `Result` types for comprehensive error handling:

```rust
use xrpl_wasm_std::types::{WasmResult, WasmError};

fn process_escrow() -> WasmResult<i32> {
    let tx = EscrowFinish;

    // Chain operations with ?
    let account = tx.get_account()?;
    let balance = get_account_balance(&account)?;

    // Handle specific errors
    match get_account_info(&account, sfield::Sequence) {
        Ok(sequence) => {
            // Use sequence
        },
        Err(WasmError::FieldNotFound) => {
            // Handle missing field
            return Ok(0);
        },
        Err(WasmError::ObjectNotFound) => {
            // Account doesn't exist
            return Ok(0);
        },
        Err(e) => return Err(e),
    }

    Ok(if balance > 10_000_000 { 1 } else { 0 })
}
```

**Common error patterns:**

- `WasmError::ObjectNotFound` - Ledger object doesn't exist
- `WasmError::FieldNotFound` - Required field missing
- `WasmError::InvalidField` - Field data malformed
- `WasmError::BufferTooSmall` - Output buffer insufficient
- `WasmError::CacheSlotNotFound` - Cached object evicted

---

## Examples

### Hello World

The simplest possible smart escrow that demonstrates basic concepts:

**Location:** `examples/smart-escrows/hello_world/`

```rust
#![no_std]
#![no_main]

use xrpl_wasm_std::core::current_tx::escrow_finish::EscrowFinish;
use xrpl_wasm_std::host::trace::trace;

#[no_mangle]
pub extern "C" fn finish() -> i32 {
    // Log execution (for debugging)
    let _ = trace("Hello World smart escrow executing");

    let tx = EscrowFinish;

    // Get transaction details
    match tx.get_account() {
        Ok(account) => {
            let _ = trace("Successfully retrieved account");
            1 // Always release escrow
        },
        Err(_) => {
            let _ = trace("Failed to retrieve account");
            0 // Keep escrow locked
        }
    }
}
```

**Key learning points:**

- Basic contract structure with `#![no_std]` and `#![no_main]`
- Using `#[no_mangle]` for the entry point function
- Simple error handling with pattern matching
- Trace logging for debugging

### Oracle Example

A price-based escrow that releases funds when an asset price meets conditions:

**Location:** `examples/smart-escrows/oracle/`

```rust
#![no_std]
#![no_main]

use xrpl_wasm_std::core::current_tx::escrow_finish::EscrowFinish;
use xrpl_wasm_std::core::ledger_objects::oracle::get_oracle_data;
use xrpl_wasm_std::types::{AccountID, OracleID};

#[no_mangle]
pub extern "C" fn finish() -> i32 {
    let tx = EscrowFinish;

    // Get oracle account from transaction memo or hardcoded
    let oracle_account = match get_oracle_account(&tx) {
        Ok(acc) => acc,
        Err(_) => return 0,
    };

    // Fetch current price from oracle
    let oracle_id = OracleID::for_account(&oracle_account);
    let current_price = match get_oracle_data(&oracle_id, "XRPUSD") {
        Ok(price) => price,
        Err(_) => return 0, // No price data available
    };

    // Release if XRP price > $0.50
    if current_price > 50_000 { // Price in micro-dollars
        1 // Release escrow
    } else {
        0 // Keep locked
    }
}

fn get_oracle_account(tx: &EscrowFinish) -> Result<AccountID, ()> {
    // Implementation depends on how oracle account is specified
    // Could be in memo, destination tag, or hardcoded
    Ok(AccountID::from([0u8; 20])) // Placeholder
}
```

**Key concepts demonstrated:**

- External data integration through oracles
- Price threshold logic
- Error handling for missing oracle data
- Real-world conditional logic

### KYC Example

A compliance-focused escrow that requires credential verification:

**Location:** `examples/smart-escrows/kyc/`

```rust
#![no_std]
#![no_main]

use xrpl_wasm_std::core::current_tx::escrow_finish::EscrowFinish;
use xrpl_wasm_std::core::ledger_objects::credential::{
    get_credential, verify_credential_signature
};
use xrpl_wasm_std::types::{AccountID, CredentialID};

// Trusted KYC provider account
const KYC_PROVIDER: AccountID = AccountID::from_raw([
    0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0,
    0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0,
    0x12, 0x34, 0x56, 0x78
]);

#[no_mangle]
pub extern "C" fn finish() -> i32 {
    let tx = EscrowFinish;

    // Get the account attempting to finish escrow
    let account = match tx.get_account() {
        Ok(acc) => acc,
        Err(_) => return 0,
    };

    // Check if account has valid KYC credential
    match verify_kyc_credential(&account) {
        Ok(true) => 1,  // KYC verified, release escrow
        _ => 0,          // No valid KYC, keep locked
    }
}

fn verify_kyc_credential(account: &AccountID) -> Result<bool, ()> {
    // Look for KYC credential from trusted provider
    let credential_id = CredentialID::for_subject_and_issuer(account, &KYC_PROVIDER);

    let credential = match get_credential(&credential_id) {
        Ok(cred) => cred,
        Err(_) => return Ok(false), // No credential found
    };

    // Verify credential signature and expiration
    let is_valid = verify_credential_signature(&credential, &KYC_PROVIDER)?;
    let is_not_expired = check_credential_expiration(&credential)?;

    Ok(is_valid && is_not_expired)
}

fn check_credential_expiration(credential: &Credential) -> Result<bool, ()> {
    // Check if credential has expired
    // Implementation depends on credential format
    Ok(true) // Placeholder
}
```

**Key concepts demonstrated:**

- Credential-based verification
- Trusted issuer validation
- Signature verification
- Expiration checking
- Compliance patterns

### Advanced Examples

#### Multi-Signature Notary

**Location:** `examples/smart-escrows/notary/`

- Requires multiple signature approvals
- Implements threshold signing logic
- Demonstrates complex authorization patterns

#### NFT Ownership Verification

**Location:** `examples/smart-escrows/nft_owner/`

- Releases funds based on NFT ownership
- Shows how to query NFT ledger objects
- Demonstrates asset-based conditions

#### Time-Based Ledger Sequence

**Location:** `examples/smart-escrows/ledger_sqn/`

- Uses ledger sequence numbers for timing
- Implements time-locked escrows
- Shows sequence-based logic

---

## Development Guide

### Build System

#### Project Structure

```text
my-escrow/
├── Cargo.toml           # Package configuration
├── src/
│   └── lib.rs          # Main contract code
├── README.md           # Documentation
├── run_test.js         # Integration test
└── target/
    └── wasm32v1-none/
        └── release/
            └── my_escrow.wasm
```

#### Build Configuration

**Essential `Cargo.toml` settings:**

```toml
[package]
name = "my-escrow"
version = "0.1.0"
edition = "2021"

[dependencies]
xrpl-wasm-std = { path = "../../../xrpl-wasm-std" }

[lib]
crate-type = ["cdylib"]  # Required for WASM library

[profile.release]
opt-level = "s"          # Optimize for size
lto = true              # Link-time optimization
panic = "abort"         # Reduce binary size
codegen-units = 1       # Better optimization
```

#### Build Commands

```shell
# Debug build (larger, with debug info)
cargo build --target wasm32v1-none

# Release build (optimized for size)
cargo build --target wasm32v1-none --release

# Build all examples
./scripts/build.sh

# Build and test everything
./scripts/build-and-test.sh
```

### Testing

#### Automated Testing

```shell
# Run all tests (equivalent to CI)
./scripts/run-all.sh

# Run integration tests only
./scripts/run-tests.sh

# Test specific example
./scripts/run-tests.sh examples/smart-escrows/oracle

# Test with custom environment
CI=1 RIPPLED_HOST=localhost ./scripts/run-tests.sh
```

#### Manual Testing with UI

1. **Build your contract:**

   ```shell
   cargo build --target wasm32v1-none --release
   ```

2. **Update the web UI:**

   ```shell
   ./ui/embed-wasm.sh
   ```

3. **Open the testing interface:**
   ```shell
   open ui/index.html
   ```

The web UI allows you to:

- Load different WASM contracts
- Set up test transactions and ledger state
- Execute contracts and see results
- Debug trace output and errors

#### Test Networks

| Network         | Endpoint                                 | Purpose             |
| --------------- | ---------------------------------------- | ------------------- |
| **WASM Devnet** | `wss://wasm.devnet.rippletest.net:51233` | Integration testing |
| **Local Node**  | `ws://localhost:6006`                    | Development         |
| **Testnet**     | `wss://s.altnet.rippletest.net:51233`    | Staging             |

#### Custom Test Configuration

```javascript
// run_test.js
const { spawn } = require("child_process")

// Configure test parameters
const CONFIG = {
  wasmPath: "./target/wasm32v1-none/release/my_escrow.wasm",
  rippledHost: process.env.RIPPLED_HOST || "wasm.devnet.rippletest.net",
  rippledPort: process.env.RIPPLED_PORT || "51233",
  testAccount: "rN7n7otQDd6FczFgLdSqtcsAUxDkw6fzRH",
  // Add custom test scenarios
}

// Run integration test
async function runTest() {
  // Test implementation
}
```

### Performance Optimization

#### Binary Size Optimization

**Cargo.toml optimizations:**

```toml
[profile.release]
opt-level = "s"           # Optimize for size over speed
lto = true               # Link-time optimization
panic = "abort"          # Remove panic handling code
codegen-units = 1        # Single codegen unit for better optimization
strip = true             # Strip debug symbols
```

**Code patterns for smaller binaries:**

```rust
// Use fixed-size arrays instead of vectors
let mut buffer = [0u8; 32];  // Stack allocation

// Minimize string usage
const ERROR_MSG: &str = "Error"; // Use constants

// Efficient error handling
match operation() {
    Ok(result) => result,
    Err(_) => return 0,  // Simple error path
}
```

#### Runtime Optimization

**Minimize host function calls:**

```rust
// Good: Call once, use cached result
let account = tx.get_account()?;
let balance = get_account_balance(&account)?;
let info = get_account_info(&account, sfield::Sequence)?;

// Bad: Multiple calls for same data
let balance = get_account_balance(&tx.get_account()?)?;
let sequence = get_account_info(&tx.get_account()?, sfield::Sequence)?;
```

**Efficient ledger object access:**

```rust
// Cache ledger objects for multiple field access
let slot = cache_ledger_obj(&account_keylet)?;
let balance = get_field_from_slot::<u64>(slot, sfield::Balance)?;
let sequence = get_field_from_slot::<u32>(slot, sfield::Sequence)?;
let owner_count = get_field_from_slot::<u32>(slot, sfield::OwnerCount)?;
```

**Memory usage optimization:**

```rust
// Use stack-based allocation
let mut accounts = [AccountID::default(); 10];

// Reuse buffers
let mut buffer = [0u8; 64];
let len1 = get_field(sfield::Account, 0, &mut buffer[..20])?;
let len2 = get_field(sfield::Destination, 0, &mut buffer[20..40])?;
```

### Troubleshooting

#### Common Build Issues

| Issue                            | Solution                                       |
| -------------------------------- | ---------------------------------------------- |
| `wasm32v1-none` target not found | `rustup target add wasm32v1-none`              |
| Link errors                      | Check `crate-type = ["cdylib"]` in Cargo.toml  |
| Binary too large                 | Use release profile optimizations              |
| Missing exports                  | Ensure `#[no_mangle]` on `finish()` function   |
| Compilation errors               | Check `#![no_std]` and avoid std library usage |

#### Common Runtime Issues

| Issue                    | Cause                   | Solution                            |
| ------------------------ | ----------------------- | ----------------------------------- |
| Function not found       | WASM export missing     | Check `#[no_mangle]` on entry point |
| Memory access violation  | Buffer overflow         | Verify buffer sizes and bounds      |
| Cache full (NoFreeSlots) | Too many cached objects | Minimize `cache_ledger_obj` calls   |
| Field not found          | Missing ledger field    | Handle `FieldNotFound` errors       |
| Invalid field data       | Malformed field         | Validate input data                 |

#### Debugging Techniques

**Add trace statements:**

```rust
use xrpl_wasm_std::host::trace::{trace, trace_data, DataRepr};

#[no_mangle]
pub extern "C" fn finish() -> i32 {
    trace("Contract starting")?;

    let account = match EscrowFinish.get_account() {
        Ok(acc) => {
            trace_data("Account", &acc.as_bytes(), DataRepr::AsHex)?;
            acc
        },
        Err(e) => {
            trace(&format!("Error getting account: {:?}", e))?;
            return 0;
        }
    };

    // More logic with tracing...
}
```

**Inspect WASM binary:**

```shell
# Check exports
./scripts/check-wasm-exports.sh target/wasm32v1-none/release/my_escrow.wasm

# Detailed binary analysis
wasm-objdump -x target/wasm32v1-none/release/my_escrow.wasm

# Size analysis
wasm-objdump -h target/wasm32v1-none/release/my_escrow.wasm
```

**Test incrementally:**

```rust
// Start with minimal working contract
#[no_mangle]
pub extern "C" fn finish() -> i32 {
    1  // Always release for testing
}

// Add functionality step by step
#[no_mangle]
pub extern "C" fn finish() -> i32 {
    let tx = EscrowFinish;
    match tx.get_account() {
        Ok(_) => 1,
        Err(_) => 0,
    }
}

// Continue building complexity...
```

---

## Contributing

### Code Standards

#### Rust Code Style

**Follow standard Rust conventions:**

```rust
// Use descriptive names
fn verify_oracle_price_threshold(
    oracle_account: &AccountID,
    currency_pair: &str,
    threshold: u64
) -> WasmResult<bool> {
    // Implementation
}

// Prefer explicit error handling
match get_account_balance(&account) {
    Ok(balance) => {
        // Handle success case
    },
    Err(WasmError::ObjectNotFound) => {
        // Handle specific error
        return Ok(false);
    },
    Err(e) => return Err(e),
}

// Use meaningful constants
const MIN_BALANCE_THRESHOLD: u64 = 10_000_000; // 10 XRP in drops
const KYC_PROVIDER_ACCOUNT: AccountID = AccountID::from_raw([...]);
```

**Code organization:**

```rust
// Group imports logically
use xrpl_wasm_std::core::current_tx::escrow_finish::EscrowFinish;
use xrpl_wasm_std::core::ledger_objects::{
    account::{get_account_balance, get_account_info},
    oracle::get_oracle_data,
};
use xrpl_wasm_std::types::{AccountID, WasmResult};

// Use modules for complex contracts
mod price_verification {
    pub fn verify_price_condition(/* ... */) -> WasmResult<bool> {
        // Price verification logic
    }
}

mod credential_check {
    pub fn verify_kyc_credential(/* ... */) -> WasmResult<bool> {
        // KYC verification logic
    }
}
```

#### JavaScript Code Style

**For test scripts and tools:**

```javascript
// Use modern JavaScript features
const { WebSocket } = require("ws")
const fs = require("fs").promises

// Descriptive function names
async function executeSmartEscrowTest(wasmPath, testConfig) {
  // Test implementation
}

// Proper error handling
try {
  const result = await runTest()
  console.log("Test passed:", result)
} catch (error) {
  console.error("Test failed:", error.message)
  process.exit(1)
}

// Configuration objects
const TEST_CONFIG = {
  rippledEndpoint: "wss://wasm.devnet.rippletest.net:51233",
  timeout: 30000,
  retryAttempts: 3,
}
```

#### Documentation Standards

**README.md structure for examples:**

```markdown
# Example Name

Brief description of what this example demonstrates.

## Functionality

- Key feature 1
- Key feature 2
- Key feature 3

## Building

\`\`\`shell
cargo build --target wasm32v1-none --release
\`\`\`

## Testing

\`\`\`shell
./scripts/run-tests.sh examples/smart-escrows/example-name
\`\`\`

## Code Walkthrough

### Core Logic

Explain the main contract logic...

### Error Handling

Explain error handling approach...

## Learning Objectives

- Objective 1
- Objective 2

## Next Steps

- Suggestion 1
- Suggestion 2
```

### Pull Request Process

#### 1. Development Setup

```shell
# Fork and clone repository
git clone https://github.com/YOUR-USERNAME/xrpl-wasm-std.git
cd xrpl-wasm-std

# Set up development environment
./scripts/setup.sh

# Create feature branch
git checkout -b feature/your-feature-name
```

#### 2. Making Changes

```shell
# Make your changes
# Edit files, add features, fix bugs

# Test your changes
./scripts/run-all.sh

# Ensure code quality
./scripts/fmt.sh
./scripts/clippy.sh
```

#### 3. Submitting PR

```shell
# Commit your changes
git add .
git commit -m "Add descriptive commit message

- Change 1
- Change 2
- Change 3"

# Push to your fork
git push origin feature/your-feature-name

# Create pull request on GitHub
```

#### 4. PR Requirements

**All PRs must:**

- Pass all existing tests
- Include tests for new functionality
- Follow code style guidelines
- Update documentation as needed
- Include clear commit messages

**For new examples:**

- Follow the example README template
- Include comprehensive code comments
- Add integration test (`run_test.js`)
- Test on WASM devnet

**For library changes:**

- Update API documentation
- Add unit tests where applicable
- Consider backward compatibility
- Update changelog

### Development Workflow

#### Local Testing Workflow

```shell
# 1. Setup and verify environment
./scripts/setup.sh
./scripts/run-tests.sh examples/smart-escrows/hello_world

# 2. Development cycle
while developing:
    # Make changes
    vim src/lib.rs

    # Check formatting and linting
    ./scripts/fmt.sh && ./scripts/clippy.sh

    # Build
    cargo build --target wasm32v1-none --release

    # Test
    ./scripts/run-tests.sh examples/your-project

    # Debug if needed
    ./ui/embed-wasm.sh && open ui/index.html

# 3. Final verification
./scripts/run-all.sh
```

#### Adding New Examples

1. **Create example directory:**

   ```shell
   mkdir -p examples/smart-escrows/my-example
   cd examples/smart-escrows/my-example
   ```

2. **Set up basic structure:**

   ```shell
   # Create Cargo.toml, src/lib.rs, README.md, run_test.js
   # Use existing examples as templates
   ```

3. **Implement and test:**

   ```shell
   # Build and test iteratively
   cargo build --target wasm32v1-none --release
   node run_test.js
   ```

4. **Update documentation:**
   ```shell
   # Add to examples overview
   # Update main README if significant
   ```

#### Continuous Integration

**Local CI simulation:**

```shell
# Run the same checks as CI
./scripts/run-all.sh

# Individual CI steps
./scripts/setup.sh
./scripts/fmt.sh
./scripts/clippy.sh
./scripts/build-and-test.sh
```

**CI automatically runs:**

- Code formatting check (`cargo fmt --check`)
- Linting (`cargo clippy`)
- Build all examples
- Integration tests on WASM devnet
- Documentation generation

**Contributing to CI:**

- CI scripts are in `.github/workflows/`
- Local scripts mirror CI exactly
- Test changes locally before pushing

#### Release Process

**For maintainers:**

1. **Version bumping:**

   ```shell
   # Update version in Cargo.toml
   # Update CHANGELOG.md
   # Tag release
   git tag v0.x.y
   ```

2. **Documentation updates:**

   ```shell
   # Ensure all docs are current
   # Rebuild and deploy documentation
   cargo +nightly doc --no-deps --workspace
   ```

3. **Testing:**
   ```shell
   # Full test suite on all supported networks
   ./scripts/run-all.sh
   ```

The XRPL WebAssembly Standard Library is designed to make smart escrow development accessible while maintaining the security and determinism required for production use on the XRPL network.

For additional help:

- Review the examples in `examples/smart-escrows/`
- Check the API documentation generated by `cargo doc`
- Join the XRPL developer community
- Submit issues or questions on GitHub
