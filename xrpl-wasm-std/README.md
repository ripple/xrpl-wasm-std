# xrpl-wasm-std Library

The XRPL Standard Library provides safe, type-safe access to XRPL host functions for WebAssembly smart contract development. This `no_std` library offers zero-cost abstractions over raw host function calls and handles memory management, error handling, and type conversions.

## Table of Contents

- [Overview](#overview)
- [Installation](#installation)
- [Core Concepts](#core-concepts)
- [API Reference](#api-reference)
  - [Transaction Access](#transaction-access)
  - [Ledger Object Access](#ledger-object-access)
  - [Type System](#type-system)
  - [Field Access](#field-access)
  - [Cryptographic Functions](#cryptographic-functions)
  - [State Management](#state-management)
  - [Logging and Debugging](#logging-and-debugging)
- [Required Module Exports](#required-module-exports)
- [Usage Examples](#usage-examples)
- [Safety and Constraints](#safety-and-constraints)
- [Error Handling](#error-handling)
- [Best Practices](#best-practices)

## Overview

The xrpl-wasm-std library is designed for developing WebAssembly modules that implement conditional logic for XRPL Escrow objects. It provides:

- **Type-safe access** to transaction and ledger data
- **Memory-safe operations** with no heap allocations
- **Deterministic execution** across all nodes/validators
- **Zero-cost abstractions** over host functions
- **Comprehensive error handling** with custom Result types

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
xrpl-wasm-std = { path = "../xrpl-wasm-std" }

[lib]
crate-type = ["cdylib"]

[profile.release]
opt-level = "s"
lto = true
```

## Core Concepts

### Host Functions

The library provides access to host functions through safe Rust APIs. Host functions allow:

- **Reading** transaction data
- **Accessing** ledger objects
- **Computing** cryptographic hashes
- **Updating** escrow state (limited)
- **Tracing** for debugging

### Memory Model

- **Stack-based**: All allocations are on the stack
- **Fixed buffers**: Predefined sizes for each type
- **No heap**: Compatible with `no_std` environments
- **Linear memory**: WASM linear memory for data exchange

### Error Handling

All fallible operations return `Result<T>` with specific error codes:

```rust,ignore
pub enum Error {
    InternalError = -1,        // Internal invariant violation
    FieldNotFound = -2,        // Requested field doesn't exist
    BufferTooSmall = -3,       // Buffer too small for data
    NoFreeSlots = -8,          // No cache slots available
    PointerOutOfBound = -13,   // Memory access violation
}
```

## API Reference

### Transaction Access

Access fields from the current transaction being processed:

```rust,ignore
use xrpl_wasm_std::core::current_tx::escrow_finish::EscrowFinish;

// Create an instance to access the current EscrowFinish transaction
let tx = EscrowFinish;

// Access common transaction fields
let account = tx.get_account()?;                   // Transaction sender
let fee = tx.get_fee()?;                           // Fee in drops
let sequence = tx.get_sequence()?;                 // Account sequence
let flags = tx.get_flags()?;                       // Transaction flags
let signing_key = tx.get_signing_pub_key()?;       // Signing public key

// Access EscrowFinish-specific fields
let owner = tx.get_owner()?;                       // Escrow creator
let offer_sequence = tx.get_offer_sequence()?;     // EscrowCreate sequence
let condition = tx.get_condition()?;               // Optional crypto-condition
let fulfillment = tx.get_fulfillment()?;           // Optional fulfillment
```

### Ledger Object Access

Access ledger objects like accounts, escrows, and other on-ledger data:

```rust,ignore
use xrpl_wasm_std::core::ledger_objects::{
    current_escrow::get_current_escrow,
    account::get_account_balance,
};

// Get the current escrow object
let escrow = get_current_escrow();

// Access escrow fields
let destination = escrow.get_destination()?;
let amount = escrow.get_amount()?;
let condition = escrow.get_condition()?;
let cancel_after = escrow.get_cancel_after()?;
let finish_after = escrow.get_finish_after()?;

// Get account balance
let balance = get_account_balance(&account)?;  // Returns drops (u64)
```

### Type System

Core types for XRPL data:

```rust,ignore
use xrpl_wasm_std::core::types::*;

// Account identifier (20 bytes)
let account: AccountID = /* ... */;

// Transaction/ledger hash (32 bytes)
let hash: Hash256 = /* ... */;

// Public key (33 bytes compressed)
let pubkey: PublicKey = /* ... */;

// Variable-length data (max 1024 bytes)
let data: Blob = /* ... */;

// XRP amount (drops)
let amount: Amount = /* ... */;

// Transaction type enumeration
let tx_type: TransactionType = TransactionType::EscrowFinish;
```

### Field Access

#### Direct Field Access

Access top-level fields from transactions or objects:

```rust,ignore
use xrpl_wasm_std::host::get_tx_field;
use xrpl_wasm_std::sfield;

// Get a field by its field code
let mut buffer = [0u8; 20];
let len = get_tx_field(sfield::Account, 0, &mut buffer)?;
```

#### Nested Field Access

Access fields within complex objects using locators:

```rust,ignore
use xrpl_wasm_std::core::locator::Locator;
use xrpl_wasm_std::host::get_tx_nested_field;
use xrpl_wasm_std::sfield;

// Build a locator for Memos[0].MemoType
let mut locator = Locator::new();
locator.pack(sfield::Memos);      // Array field
locator.pack(0);                   // Array index
locator.pack(sfield::MemoType);    // Field within object

// Get the nested field
let mut buffer = [0u8; 256];
let len = get_tx_nested_field(&locator.buffer, &mut buffer)?;
```

**Important**: For STArray navigation, omit the intermediate object wrapper:
- ✅ `Memos → [0] → MemoType`
- ❌ `Memos → [0] → Memo → MemoType`

### Keylet Generation

Generate unique identifiers for ledger objects:

```rust,ignore
use xrpl_wasm_std::core::types::keylets::*;

// Account keylet
let account_key = account_keylet(&account_id)?;

// Escrow keylet
let escrow_key = escrow_keylet(&owner, sequence)?;

// Oracle keylet
let oracle_key = oracle_keylet(&owner, document_id)?;

// Credential keylet
let cred_key = credential_keylet(&subject, &issuer, credential_type)?;
```

### Cryptographic Functions

```rust,ignore
use xrpl_wasm_std::core::crypto::compute_sha512_half;

// Compute SHA-512 half (first 32 bytes)
let mut hash = [0u8; 32];
compute_sha512_half(data, &mut hash)?;
```

### State Management

The only allowed state modification:

```rust,ignore
use xrpl_wasm_std::core::ledger_objects::current_escrow::update_data;

// Update the escrow's data field (max 256 bytes)
let new_data = b"execution result";
update_data(new_data)?;
```

### Logging and Debugging

Debug output during development:

```rust,ignore
use xrpl_wasm_std::host::trace::{trace, trace_data, trace_num, DataRepr};

// Simple text trace
trace("Processing escrow finish")?;

// Trace with data
trace_data("Account ID", &account_id, DataRepr::AsHex)?;
trace_data("Message", b"Hello", DataRepr::AsUTF8)?;

// Trace with number
trace_num("Balance", balance as i64)?;
```

## Required Module Exports

Every WASM module must export the following function:

```rust,ignore
#![no_std]
#![no_main]

/// Main entry point - returns 1 to release escrow, 0 to keep locked
/// Can also return any positive value to release, or any negative value to keep locked
#[no_mangle]
pub extern "C" fn finish() -> i32 {
    // Your conditional logic here
    1  // Release escrow (or 0 to keep locked)
}
```

Only `finish()` must be exported.

## Usage Examples

### Basic Balance Check

```rust,ignore
use xrpl_wasm_std::core::current_tx::escrow_finish::EscrowFinish;
use xrpl_wasm_std::core::ledger_objects::account::get_account_balance;

#[no_mangle]
pub extern "C" fn finish() -> i32 {
    // Get transaction sender
    let tx = EscrowFinish;
    let account = match tx.get_account() {
        Ok(acc) => acc,
        Err(_) => return 0,
    };

    // Check if balance > 10 XRP
    match get_account_balance(&account) {
        Ok(balance) => {
            if balance > 10_000_000 {  // 10 XRP in drops
                1  // Release escrow
            } else {
                0  // Keep locked
            }
        },
        Err(_) => 0,
    }
}
```

### Using classic (r...) addresses

Contracts compare 20-byte AccountID values. If you have a classic XRPL address (r...) during development, use the `xrpl-address-macro` crate with the `r_address!` macro to convert it to a `[u8; 20]` constant at compile time. See `projects/notary` for an example of how to use this macro for address comparison inside the WASM.

### Build and run your contract

Build a contract for WASM and run it with the host:

```shell
cargo build --target wasm32v1-none --release
```

```shell
# From the wasm-host-simulator crate:
cargo run -p wasm-host-simulator -- --dir path/to/project --project project_name --function finish
```

### Time-based Release

```rust,ignore
use xrpl_wasm_std::core::ledger_objects::current_escrow::get_current_escrow;
use xrpl_wasm_std::host::host_bindings::get_parent_ledger_time;

#[no_mangle]
pub extern "C" fn finish() -> i32 {
    // Get current time (returns i32 directly)
    let current_time = unsafe { get_parent_ledger_time() };
    if current_time <= 0 {
        return 0; // Error getting time, don't release
    }

    // Get escrow finish_after time
    let escrow = get_current_escrow();
    let finish_after = match escrow.get_finish_after() {
        Ok(Some(time)) => time,
        _ => return 0, // No finish_after set, don't release
    };

    // Release if current time >= finish_after
    if current_time as u32 >= finish_after {
        1  // Release escrow
    } else {
        0  // Keep locked
    }
}
```

### Credential Verification

```rust,ignore
use xrpl_wasm_std::core::types::keylets::credential_keylet;
use xrpl_wasm_std::host::cache_ledger_obj;

#[no_mangle]
pub extern "C" fn finish() -> i32 {
    let tx = EscrowFinish;
    let account = tx.get_account().unwrap_or_default();

    // Generate credential keylet
    let keylet = match credential_keylet(
        &account,
        &issuer_account,
        b"KYC"
    ) {
        Ok(k) => k,
        Err(_) => return 0,
    };

    // Try to load credential (returns slot number if exists)
    match cache_ledger_obj(&keylet.data) {
        Ok(_) => 1,   // Credential exists, release escrow
        Err(_) => 0,  // No credential, keep locked
    }
}
```

## Safety and Constraints

### Memory Safety
- All buffers are stack-allocated with fixed sizes
- No dynamic memory allocation
- Bounds checking on all array access
- Safe abstractions over raw pointers

### Execution Constraints
- **Deterministic**: Same inputs must produce same outputs
- **Read-only**: Only `update_data()` can modify state
- **Resource-limited**: Bounded by computation allowance
- **Isolated**: No network or file system access

### Security Considerations
- Always validate input data
- Handle all error cases explicitly
- Avoid panics in production code
- Test with malicious inputs

## Error Handling

### Best Practices

```rust,ignore
// Prefer early returns for errors
let account = match tx.get_account() {
    Ok(acc) => acc,
    Err(_) => return false,
};

// Or use the ? operator in helper functions
fn check_balance(account: &AccountID) -> Result<bool> {
    let balance = get_account_balance(account)?;
    Ok(balance > 1_000_000)
}

// Handle specific errors
match get_account_balance(&account) {
    Ok(balance) => { /* use balance */ },
    Err(Error::FieldNotFound) => { /* handle missing field */ },
    Err(Error::NoFreeSlots) => { /* handle cache full */ },
    Err(_) => { /* handle other errors */ },
}
```

### Common Error Scenarios

1. **FieldNotFound**: Optional field not present
2. **NoFreeSlots**: Ledger object cache full (max 255 slots)
3. **OutOfBounds**: Buffer too small for data
4. **InternalError**: Unexpected host function failure

## Best Practices

### 1. Minimize Host Function Calls
```rust,ignore
// Bad: Multiple calls for same data
let balance1 = get_account_balance(&account)?;
let balance2 = get_account_balance(&account)?;

// Good: Cache the result
let balance = get_account_balance(&account)?;
// Use balance multiple times
```

### 2. Handle Optional Fields
```rust,ignore
// Check if optional field exists
match tx.get_source_tag()? {
    Some(tag) => { /* use tag */ },
    None => { /* handle absence */ },
}
```

### 3. Use Type-Safe APIs
```rust,ignore
// Prefer high-level APIs
let balance = get_account_balance(&account)?;

// Over raw field access
let mut buffer = [0u8; 8];
get_ledger_obj_field(slot, sfield::Balance, 0, &mut buffer)?;
```

### 4. Fail Safely
```rust,ignore
// Return false on any error to keep escrow locked
match risky_operation() {
    Ok(result) => result,
    Err(_) => false,  // Safe default
}
```

### 5. Test Thoroughly
- Test with success and failure fixtures
- Verify deterministic behavior
- Check edge cases and error conditions
- Test with maximum data sizes

## See Also

- [EscrowFinish Documentation](https://xrpl.org/docs/references/protocol/transactions/types/escrowfinish)
- [Smart Escrow System Design](../DESIGN.md)
- [Example Projects](../projects/)
- [WASM Host Testing Tool](../wasm-host-simulator/README.md)
