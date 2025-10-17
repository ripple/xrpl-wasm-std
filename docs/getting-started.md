# Getting Started with xrpl-wasm-std

This guide will help you build your first XRPL smart escrow using the xrpl-wasm-std library.

## Prerequisites

- **Rust toolchain** with `wasm32v1-none` target
- **Node.js 18+** for testing tools
- Basic understanding of Rust and WebAssembly

## Installation

### 1. Setup Development Environment

Install required tools:

```shell
# Install Rust and WebAssembly target
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32v1-none

# Install Node.js dependencies (for testing)
npm install
```

### 2. Create Your Project

```shell
# Create new Rust project
cargo new my-escrow --lib
cd my-escrow
```

### 3. Configure Cargo.toml

```toml
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
```

## Your First Smart Escrow

### Understanding Smart Escrows

Smart escrows are WebAssembly modules that implement conditional logic for XRPL Escrow objects. They:

- **Evaluate conditions** when an EscrowFinish transaction is submitted
- **Return 1** to release the escrow funds
- **Return 0** to keep the escrow locked
- **Have read-only access** to ledger data (except updating escrow data)

### Basic Balance Check Contract

Create `src/lib.rs`:

```rust
#![no_std]
#![no_main]

use xrpl_wasm_std::core::current_tx::escrow_finish::EscrowFinish;
use xrpl_wasm_std::core::ledger_objects::account::get_account_balance;

/// Main entry point - returns 1 to release escrow, 0 to keep locked
#[no_mangle]
pub extern "C" fn finish() -> i32 {
    // Get the transaction sender
    let tx = EscrowFinish;
    let account = match tx.get_account() {
        Ok(acc) => acc,
        Err(_) => return 0, // Error getting account, keep locked
    };

    // Check if balance > 10 XRP (10,000,000 drops)
    match get_account_balance(&account) {
        Ok(balance) => {
            if balance > 10_000_000 {
                1  // Release escrow
            } else {
                0  // Keep locked
            }
        },
        Err(_) => 0, // Error getting balance, keep locked
    }
}
```

### Build Your Contract

```shell
cargo build --target wasm32v1-none --release
```

The compiled WebAssembly module will be at:

```
target/wasm32v1-none/release/my_escrow.wasm
```

## Core Concepts

### Host Functions

The library provides access to XRPL host functions through safe Rust APIs:

- **Reading transaction data** (`EscrowFinish` fields)
- **Accessing ledger objects** (accounts, escrows, oracles, etc.)
- **Computing cryptographic hashes**
- **Updating escrow state** (limited to data field)
- **Tracing for debugging**

### Memory Model

Smart escrows use a constrained memory model:

- **Stack-based**: All allocations are on the stack
- **Fixed buffers**: Predefined sizes for each type (no dynamic allocation)
- **No heap**: Compatible with `no_std` environments
- **Linear memory**: WASM linear memory for data exchange with host

### Error Handling

All fallible operations return `Result<T, Error>`:

```rust
use xrpl_wasm_std::host::Error;

match get_account_balance(&account) {
    Ok(balance) => { /* use balance */ },
    Err(Error::FieldNotFound) => { /* handle missing field */ },
    Err(Error::NoFreeSlots) => { /* handle cache full */ },
    Err(_) => { /* handle other errors */ },
}
```

Common error codes:

- `InternalError` - Internal invariant violation
- `FieldNotFound` - Requested field doesn't exist
- `BufferTooSmall` - Buffer too small for data
- `NoFreeSlots` - No cache slots available
- `PointerOutOfBound` - Memory access violation

## Testing Your Contract

### 1. Using the Test Scripts

From the project root:

```shell
# Run comprehensive tests
./scripts/run-tests.sh examples/my-escrow

# Or run just your specific test
CI=1 ./scripts/run-tests.sh examples/my-escrow
```

### 2. Using the Web UI

Open `ui/index.html` in your browser to:

- Connect to WASM Devnet or local node
- Upload your compiled WASM
- Create test accounts and escrows
- Test your contract interactively

### 3. Manual Testing on Devnet

```javascript
// Example using XRPL.js to test your contract
const xrpl = require("xrpl")
const fs = require("fs")

const client = new xrpl.Client("wss://wasm.devnet.rippletest.net:51233")
await client.connect()

// Load your compiled WASM
const wasmBytes = fs.readFileSync("target/wasm32v1-none/release/my_escrow.wasm")
const wasmHex = wasmBytes.toString("hex").toUpperCase()

// Create escrow with your WASM as FinishFunction
const escrowTx = {
  TransactionType: "EscrowCreate",
  Account: senderWallet.address,
  Destination: receiverWallet.address,
  Amount: "1000000", // 1 XRP
  FinishAfter: Math.floor(Date.now() / 1000) + 10, // 10 seconds
  FinishFunction: wasmHex,
}

// Submit and wait for validation
const result = await client.submitAndWait(escrowTx, { wallet: senderWallet })
```

## Advanced Examples

### Time-based Release

```rust
use xrpl_wasm_std::core::ledger_objects::current_escrow::get_current_escrow;
use xrpl_wasm_std::host::host_bindings::get_parent_ledger_time;

#[no_mangle]
pub extern "C" fn finish() -> i32 {
    // Get current ledger time
    let current_time = unsafe { get_parent_ledger_time() };
    if current_time <= 0 {
        return 0; // Error getting time
    }

    // Get escrow finish_after time
    let escrow = get_current_escrow();
    let finish_after = match escrow.get_finish_after() {
        Ok(Some(time)) => time,
        _ => return 0, // No finish_after set
    };

    // Release if current time >= finish_after
    if current_time as u32 >= finish_after {
        1  // Release escrow
    } else {
        0  // Keep locked
    }
}
```

### Using Classic Addresses

For development with known XRPL addresses, use the `r_address!` macro:

```rust
use xrpl_wasm_std::r_address;

const NOTARY: [u8; 20] = r_address!("rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh");

#[no_mangle]
pub extern "C" fn finish() -> i32 {
    let tx = EscrowFinish;
    let account = tx.get_account().unwrap_or_default();

    // Only allow release if transaction is signed by notary
    if account == NOTARY {
        1  // Release
    } else {
        0  // Keep locked
    }
}
```

## Best Practices

### 1. Handle Errors Gracefully

```rust
// Prefer early returns for errors
let account = match tx.get_account() {
    Ok(acc) => acc,
    Err(_) => return 0, // Safe default: keep locked
};
```

### 2. Minimize Host Function Calls

```rust
// Cache results instead of repeated calls
let balance = get_account_balance(&account)?;
// Use balance multiple times
```

### 3. Fail Safely

```rust
// Always return 0 on unexpected errors to keep escrow locked
match risky_operation() {
    Ok(result) => result,
    Err(_) => 0,  // Safe default
}
```

### 4. Use Debugging Tools

```rust
use xrpl_wasm_std::host::trace::{trace, trace_data, DataRepr};

// Debug your contract execution
trace("Starting balance check")?;
trace_data("Account", &account, DataRepr::AsHex)?;
```

## Next Steps

- **[API Reference](api-reference.md)** - Complete API documentation
- **[Examples](examples/README.md)** - Study more complex examples
- **[Development Guide](development/building.md)** - Advanced build configuration
- **[Rust API Docs](../target/doc/xrpl_wasm_std/index.html)** - Generated documentation

## Troubleshooting

### Build Issues

```shell
# Ensure correct target is installed
rustup target add wasm32v1-none

# Clean and rebuild
cargo clean
cargo build --target wasm32v1-none --release
```

### Runtime Issues

- Check that your `finish()` function is exported with `#[no_mangle]`
- Ensure all error cases return 0 (safe default)
- Use trace functions to debug execution flow
- Verify WebAssembly module exports with `check-wasm-exports.sh`

### Testing Issues

- Ensure Node.js dependencies are installed: `npm install`
- Check network connectivity to WASM Devnet
- Verify WASM file is under size limits (typically <64KB)
