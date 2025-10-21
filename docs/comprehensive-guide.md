# XRPL WebAssembly Standard Library - Complete Guide

This comprehensive guide covers everything you need to develop smart escrows using the XRPL WebAssembly Standard Library.

## Table of Contents

- [XRPL WebAssembly Standard Library - Complete Guide](#xrpl-webassembly-standard-library---complete-guide)
  - [Table of Contents](#table-of-contents)
  - [Getting Started](#getting-started)
    - [Prerequisites](#prerequisites)
    - [Installation](#installation)
    - [Your First Contract](#your-first-contract)
    - [Core Concepts](#core-concepts)
      - [Smart Escrow Basics](#smart-escrow-basics)
      - [Contract Structure](#contract-structure)
      - [Host Environment](#host-environment)
  - [API Reference](#api-reference)
    - [Transaction Access](#transaction-access)
      - [EscrowFinish Transaction](#escrowfinish-transaction)
      - [Field Access](#field-access)
    - [Ledger Objects](#ledger-objects)
      - [Account Information](#account-information)
      - [Trust Lines](#trust-lines)
      - [NFT Objects](#nft-objects)
    - [Type System](#type-system)
      - [Core Types](#core-types)
      - [Keylet Generation](#keylet-generation)
    - [Host Functions](#host-functions)
      - [Ledger Access](#ledger-access)
      - [Transaction Fields](#transaction-fields)
      - [Utility Functions](#utility-functions)
    - [Error Handling](#error-handling)
  - [Examples](#examples)
    - [Hello World](#hello-world)
    - [Oracle Example](#oracle-example)
    - [KYC Example](#kyc-example)
    - [Advanced Examples](#advanced-examples)
      - [Multi-Signature Notary](#multi-signature-notary)
      - [NFT Ownership Verification](#nft-ownership-verification)
      - [Time-Based Ledger Sequence](#time-based-ledger-sequence)
  - [Testing and Debugging](#testing-and-debugging)
    - [Test Networks](#test-networks)
    - [Test Using the Web UI](#test-using-the-web-ui)
    - [Performance Optimization](#performance-optimization)
      - [Binary Size Optimization](#binary-size-optimization)
      - [Runtime Optimization](#runtime-optimization)
    - [Troubleshooting](#troubleshooting)
      - [Common Build Issues](#common-build-issues)
      - [Common Runtime Issues](#common-runtime-issues)
      - [Debugging Techniques](#debugging-techniques)
  - [Additional Resources](#additional-resources)
  - [Contributing](#contributing)

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
# Follow the instructions at https://rust-lang.org/tools/install/
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
4. **Use `#![no_std]`** - no standard library available (use ours instead 😉)

#### Host Environment

Smart escrows run in a constrained WebAssembly environment:

- No heap allocation - stack-based memory only
- No file system or network access
- Limited execution time and memory
- Read-only ledger access (except for escrow state updates)

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

The simplest possible smart escrow that demonstrates basic concepts.

**📁 View complete example:** [`examples/smart-escrows/hello_world/`](https://github.com/ripple/xrpl-wasm-std/tree/main/examples/smart-escrows/hello_world/)

**Key learning points:**

- Basic contract structure with `#![no_std]` and `#![no_main]`
- Using `#[no_mangle]` for the entry point function
- Simple error handling with pattern matching
- Trace logging for debugging

**Files:**

- [`src/lib.rs`](https://github.com/ripple/xrpl-wasm-std/blob/main/examples/smart-escrows/hello_world/src/lib.rs) - Main contract code
- [`README.md`](https://github.com/ripple/xrpl-wasm-std/blob/main/examples/smart-escrows/hello_world/README.md) - Detailed explanation
- [`run_test.js`](https://github.com/ripple/xrpl-wasm-std/blob/main/examples/smart-escrows/hello_world/run_test.js) - Integration test

### Oracle Example

A price-based escrow that releases funds when an asset price meets conditions.

**📁 View complete example:** [`examples/smart-escrows/oracle/`](https://github.com/ripple/xrpl-wasm-std/tree/main/examples/smart-escrows/oracle/)

**Key concepts demonstrated:**

- External data integration through oracles
- Price threshold logic
- Error handling for missing oracle data
- Real-world conditional logic

**Files:**

- [`src/lib.rs`](https://github.com/ripple/xrpl-wasm-std/blob/main/examples/smart-escrows/oracle/src/lib.rs) - Oracle price checking logic
- [`README.md`](https://github.com/ripple/xrpl-wasm-std/blob/main/examples/smart-escrows/oracle/README.md) - Oracle integration guide
- [`run_test.js`](https://github.com/ripple/xrpl-wasm-std/blob/main/examples/smart-escrows/oracle/run_test.js) - Price simulation test

### KYC Example

A compliance-focused escrow that requires credential verification.

**📁 View complete example:** [`examples/smart-escrows/kyc/`](https://github.com/ripple/xrpl-wasm-std/tree/main/examples/smart-escrows/kyc/)

**Key concepts demonstrated:**

- Credential-based verification
- Trusted issuer validation
- Signature verification
- Expiration checking
- Compliance patterns

**Files:**

- [`src/lib.rs`](https://github.com/ripple/xrpl-wasm-std/blob/main/examples/smart-escrows/kyc/src/lib.rs) - KYC credential verification
- [`README.md`](https://github.com/ripple/xrpl-wasm-std/blob/main/examples/smart-escrows/kyc/README.md) - Compliance implementation guide
- [`run_test.js`](https://github.com/ripple/xrpl-wasm-std/blob/main/examples/smart-escrows/kyc/run_test.js) - Credential verification test

### Advanced Examples

#### Multi-Signature Notary

**📁 [`examples/smart-escrows/notary/`](https://github.com/ripple/xrpl-wasm-std/tree/main/examples/smart-escrows/notary/)**

- Requires multiple signature approvals
- Implements threshold signing logic
- Demonstrates complex authorization patterns

#### NFT Ownership Verification

**📁 [`examples/smart-escrows/nft_owner/`](https://github.com/ripple/xrpl-wasm-std/tree/main/examples/smart-escrows/nft_owner/)**

- Releases funds based on NFT ownership
- Shows how to query NFT ledger objects
- Demonstrates asset-based conditions

#### Time-Based Ledger Sequence

**📁 [`examples/smart-escrows/ledger_sqn/`](https://github.com/ripple/xrpl-wasm-std/tree/main/examples/smart-escrows/ledger_sqn/)**

- Uses ledger sequence numbers for timing
- Implements time-locked escrows
- Shows sequence-based logic

## Testing and Debugging

### Test Networks

| Network         | Endpoint                                 | Purpose             |
| --------------- | ---------------------------------------- | ------------------- |
| **WASM Devnet** | `wss://wasm.devnet.rippletest.net:51233` | Integration testing |
| **Local Node**  | `ws://localhost:6006`                    | Local Development   |

Follow the instructions [here](https://xrpl.org/docs/infrastructure/installation/build-on-linux-mac-windows) with [this branch](https://github.com/XRPLF/rippled/tree/ripple/se/supported) if you would like to build and run rippled locally.

### Test Using the Web UI

**🌐 Open the web UI:** [https://ripple.github.io/xrpl-wasm-std/ui/](https://ripple.github.io/xrpl-wasm-std/ui/)

The web UI allows you to:

- Upload and test any WASM contract directly
- Configure test transactions and ledger state
- Execute contracts and see results with trace output
- Test on different networks (Devnet, Testnet)
- Debug without local setup

1. **Build your contract:**

   ```shell
   cargo build --target wasm32v1-none --release
   ```

2. **Upload your WASM file:**
   - Open the testing interface in your browser
   - Click "Choose File" and select your `.wasm` file from `target/wasm32v1-none/release/`
   - The contract will be loaded automatically

3. **Test your contract:**
   - Set up test scenarios using the interface
   - Configure transaction data and ledger state
   - Execute and see results with debug output

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
# Detailed binary analysis
wasm-objdump -x target/wasm32v1-none/release/my_escrow.wasm

# Size analysis
wasm-objdump -h target/wasm32v1-none/release/my_escrow.wasm
```

---

## Additional Resources

The XRPL WebAssembly Standard Library is designed to make smart escrow development accessible while maintaining the security and determinism required for production use on the XRPL network.

For additional help:

- Review the examples in `examples/smart-escrows/`
- Check the API documentation generated by `cargo doc`
- Join the XRPL developer community
- Submit issues or questions on GitHub

## Contributing

If you're interested in contributing to the XRPL WebAssembly Standard Library, please see our [CONTRIBUTING.md](https://github.com/ripple/xrpl-wasm-std/blob/main/CONTRIBUTING.md) for detailed guidelines on:

- Development setup and workflow
- Code standards and style guidelines
- Pull request process
- Testing requirements
- Release procedures

We welcome contributions of all kinds, from bug fixes and documentation improvements to new examples and library features!
