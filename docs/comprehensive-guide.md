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
  - [Smart Contract Functions](#smart-contract-functions)
    - [Parameters](#parameters)
      - [Using the #[wasm_export] Macro](#using-the-wasm_export-macro)
      - [Manual Parameter Extraction](#manual-parameter-extraction)
    - [Contract Data](#contract-data)
      - [Storage Operations](#storage-operations)
      - [Best Practices](#best-practices-1)
    - [Emit Transactions](#emit-transactions)
      - [Transaction Emission](#transaction-emission)
      - [Example Usage](#example-usage)
    - [Emit Events](#emit-events)
      - [Event System](#event-system)
      - [Event Best Practices](#event-best-practices-1)
    - [Complete Token Contract Example](#complete-token-contract-example)
  - [API Reference](#api-reference)
    - [Transaction Access](#transaction-access)
      - [EscrowFinish Transaction](#escrowfinish-transaction)
      - [Field Access](#field-access)
    - [Ledger Objects](#ledger-objects)
      - [Account Information](#account-information)
      - [NFT Objects](#nft-objects)
    - [Type System](#type-system)
      - [Core Types](#core-types)
      - [Keylet Generation](#keylet-generation)
    - [Host Functions](#host-functions)
      - [Ledger Access](#ledger-access)
      - [Transaction Fields](#transaction-fields)
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
   git clone https://github.com/XRPLF/xrpl-wasm-stdlib.git
   cd xrpl-wasm-stdlib
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

use xrpl_wasm_stdlib::core::current_tx::escrow_finish::EscrowFinish;
use xrpl_wasm_stdlib::core::current_tx::traits::TransactionCommonFields;
use xrpl_wasm_stdlib::core::ledger_objects::account_root::get_account_balance;
use xrpl_wasm_stdlib::core::types::amount::Amount;
use xrpl_wasm_stdlib::host::Result::{Ok, Err};

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let tx = EscrowFinish;

    // Get the account trying to finish the escrow
    let account = match tx.get_account() {
        Ok(acc) => acc,
        Err(_) => return 0, // Invalid transaction
    };

    // Check account balance
    match get_account_balance(&account) {
        Ok(Some(Amount::XRP { num_drops })) if num_drops > 10_000_000 => 1, // Release (>10 XRP)
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
xrpl-wasm-stdlib = { path = "../xrpl-wasm-stdlib" }

[lib]
crate-type = ["cdylib"]

[profile.release]
opt-level = "s"
lto = true
panic = "abort"

# Build the contract
cargo build --target wasm32v1-none --release

# Test with provided tools
node ../examples/smart-escrows/hello_world/runTest.js
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

## Smart Contract Functions

Beyond basic escrow logic, XRPL smart contracts support advanced features including parameter handling, persistent storage, transaction emission, and event logging. This section covers these powerful capabilities.

### Parameters

Contracts can receive two types of parameters: **function parameters** (passed when calling a specific contract function) and **instance parameters** (set when the contract is deployed and remain constant).

#### Using the #[wasm_export] Macro

The `#[wasm_export]` macro simplifies parameter handling by automatically extracting both instance and function parameters. This eliminates boilerplate code and makes contracts cleaner.

**Quick Reference:**

| Scenario | Macro Syntax |
|----------|-------------|
| Function params only | `#[wasm_export]`<br/>`fn my_func(param1: u32, param2: AccountID) -> i32` |
| With custom exit | `#[wasm_export(exit = my_exit)]`<br/>`fn my_func(param: u32) -> i32` |
| With instance params | `#[wasm_export(instance(owner: AccountID, limit: u64))]`<br/>`fn my_func(param: u32) -> i32` |
| Both exit and instance | `#[wasm_export(exit = my_exit, instance(owner: AccountID))]`<br/>`fn my_func(param: u32) -> i32` |

**Setup:**

Add the macro dependency:

```toml
[dependencies]
xrpl-wasm-std = { path = "../xrpl-wasm-std" }
xrpl-wasm-macros = { path = "../xrpl-wasm-macros" }
```

Import the macro:

```rust
use xrpl_wasm_macros::wasm_export;
```

**Basic Usage:**

The macro automatically extracts function parameters based on your function signature:

```rust
use xrpl_wasm_macros::wasm_export;
use xrpl_wasm_std::core::types::account_id::AccountID;

#[wasm_export]
fn transfer(from: AccountID, to: AccountID, amount: u64) -> i32 {
    // Parameters are automatically extracted and available
    // No manual get_function_param() calls needed!
    
    // Your contract logic here
    
    0
}
```

The macro generates the wrapper code that:
1. Creates a `#[no_mangle] pub extern "C"` function
2. Extracts each parameter using `get_function_param()`
3. Calls your internal function with the extracted values

**Instance Parameters:**

Declare instance parameters that persist across all function calls:

```rust
#[wasm_export(instance(owner: AccountID, max_supply: u64))]
fn mint(recipient: AccountID, amount: u64) -> i32 {
    // Both instance parameters (owner, max_supply) and 
    // function parameters (recipient, amount) are available
    
    // Validate using instance parameter
    if amount > max_supply {
        return -1;
    }
    
    // Check authorization using instance parameter
    // ... (check if caller is owner)
    
    0
}
```

Instance parameters are:
- Extracted once at the start of each function call
- Available to all functions that declare them
- Set when the contract is deployed and remain constant

**Custom Exit Handler:**

Provide a custom error handler for parameter extraction failures:

```rust
use xrpl_wasm_std::host::trace::{trace, trace_num};

// Define your custom exit function
fn my_exit(message: &str, code: i32) -> i32 {
    trace(message);
    trace_num("Error code:", code as i64);
    code
}

#[wasm_export(exit = my_exit)]
fn transfer(from: AccountID, to: AccountID, amount: u64) -> i32 {
    // If parameter extraction fails, my_exit() will be called
    // with an error message and code
    
    0
}
```

Without a custom exit handler, the macro uses `expect()` which will panic on parameter errors.

**Complete Example with Instance Parameters and Custom Exit:**

```rust
use xrpl_wasm_macros::wasm_export;
use xrpl_wasm_std::core::types::account_id::AccountID;
use xrpl_wasm_std::host::trace::{trace, trace_num};

fn contract_exit(message: &str, code: i32) -> i32 {
    trace("Contract error:");
    trace(message);
    trace_num("Error code:", code as i64);
    code
}

#[wasm_export(
    exit = contract_exit,
    instance(owner: AccountID, max_supply: u64, fee_rate: u32)
)]
fn transfer(from: AccountID, to: AccountID, amount: u64) -> i32 {
    // All parameters are automatically available:
    // - owner (instance param)
    // - max_supply (instance param)
    // - fee_rate (instance param)
    // - from (function param)
    // - to (function param)
    // - amount (function param)
    
    trace("Transfer initiated");
    
    // Validate amount
    if amount == 0 {
        return contract_exit("Amount must be greater than 0", -1);
    }
    
    // Check supply limit
    if amount > max_supply {
        return contract_exit("Amount exceeds max supply", -2);
    }
    
    // Your transfer logic here
    
    0
}
```

#### Manual Parameter Extraction

If you prefer not to use macros, you can manually extract parameters:

**When to Use Manual Extraction:**

- **Learning/Understanding**: When you want to understand how parameters work under the hood
- **Fine-grained Control**: When you need custom error handling for specific parameters
- **Minimal Dependencies**: When you want to avoid macro dependencies
- **Debugging**: When you need to trace exactly what's happening at each step

**Function Parameters:**

```rust
use xrpl_wasm_std::core::params::function::get_function_param;
use xrpl_wasm_std::core::types::account_id::AccountID;

#[unsafe(no_mangle)]
pub extern "C" fn transfer() -> i32 {
    // Extract first parameter (AccountID)
    let from = match get_function_param::<AccountID>(0) {
        Ok(acc) => acc,
        Err(err) => {
            trace_num("Failed to get 'from' parameter:", err as i64);
            return -1;
        }
    };
    
    // Extract second parameter (AccountID)
    let to = match get_function_param::<AccountID>(1) {
        Ok(acc) => acc,
        Err(err) => {
            trace_num("Failed to get 'to' parameter:", err as i64);
            return -1;
        }
    };
    
    // Extract third parameter (u64)
    let amount = match get_function_param::<u64>(2) {
        Ok(amt) => amt,
        Err(err) => {
            trace_num("Failed to get 'amount' parameter:", err as i64);
            return -1;
        }
    };
    
    // Your contract logic here
    
    0
}
```

**Instance Parameters:**

```rust
use xrpl_wasm_std::core::params::instance::get_instance_param;

#[unsafe(no_mangle)]
pub extern "C" fn mint() -> i32 {
    // Extract instance parameters (set at deployment)
    let owner = match get_instance_param::<AccountID>(0) {
        Ok(acc) => acc,
        Err(err) => {
            trace_num("Failed to get owner:", err as i64);
            return -1;
        }
    };
    
    let max_supply = match get_instance_param::<u64>(1) {
        Ok(supply) => supply,
        Err(err) => {
            trace_num("Failed to get max_supply:", err as i64);
            return -1;
        }
    };
    
    // Extract function parameters
    let recipient = match get_function_param::<AccountID>(0) {
        Ok(acc) => acc,
        Err(err) => return -1,
    };
    
    let amount = match get_function_param::<u64>(1) {
        Ok(amt) => amt,
        Err(err) => return -1,
    };
    
    // Validate against instance parameters
    if amount > max_supply {
        trace("Amount exceeds max supply");
        return -2;
    }
    
    // Your minting logic here
    
    0
}
```

### Contract Data

Smart contracts can store and retrieve persistent data that survives across function calls. This enables stateful applications like token balances, allowances, and configuration settings.

#### Storage Operations

**Store Data:**

```rust
use xrpl_wasm_std::core::data::codec::set_data;
use xrpl_wasm_std::core::types::account_id::AccountID;

// Store a u64 balance for an account
let account = AccountID(/* account bytes */);
let balance: u64 = 1000000;

match set_data::<u64>(&account, "balance", balance) {
    Ok(_) => {
        trace("Balance stored successfully");
    }
    Err(e) => {
        trace_num("Failed to store balance:", e as i64);
        return e;
    }
}
```

**Retrieve Data:**

```rust
use xrpl_wasm_std::core::data::codec::get_data;

// Retrieve a balance, default to 0 if not found
let balance = get_data::<u64>(&account, "balance").unwrap_or(0);

// Or handle missing data explicitly
let balance = match get_data::<u64>(&account, "balance") {
    Some(bal) => bal,
    None => {
        trace("No balance found for account");
        return -1;
    }
};
```

**Supported Data Types:**

- `u32`, `u64` - Unsigned integers
- `i32`, `i64` - Signed integers
- `AccountID` - XRPL account identifiers
- `[u8; N]` - Fixed-size byte arrays
- Custom types implementing the required traits

**Storage Keys:**

Keys are strings that identify stored values. Use descriptive keys:

```rust
set_data::<u64>(&account, "balance", balance)?;
set_data::<u64>(&account, "allowance", allowance)?;
set_data::<u32>(&account, "nonce", nonce)?;
```

#### Best Practices

**1. Efficient Storage Operations:**

```rust
// Good: Read once, modify, write once
let mut balance = get_data::<u64>(&account, "balance").unwrap_or(0);
balance += amount;
set_data::<u64>(&account, "balance", balance)?;

// Bad: Multiple reads/writes
let balance = get_data::<u64>(&account, "balance").unwrap_or(0);
set_data::<u64>(&account, "balance", balance + amount)?;
```

**2. Handle Missing Data:**

```rust
// Use unwrap_or for defaults
let balance = get_data::<u64>(&account, "balance").unwrap_or(0);

// Or handle explicitly when needed
let balance = match get_data::<u64>(&account, "balance") {
    Some(bal) => bal,
    None => {
        trace("Account not initialized");
        return -1;
    }
};
```

**3. Atomic Updates:**

```rust
// Update multiple related values together
let sender_balance = get_data::<u64>(&from, "balance").unwrap_or(0);
let receiver_balance = get_data::<u64>(&to, "balance").unwrap_or(0);

if sender_balance < amount {
    return -1; // Check before any writes
}

// Update both balances
set_data::<u64>(&from, "balance", sender_balance - amount)?;
set_data::<u64>(&to, "balance", receiver_balance + amount)?;
```

### Emit Transactions

Smart contracts can emit transactions to be executed on the XRP Ledger. This allows contracts to trigger payments, trust lines, NFT operations, and other XRPL transactions.

#### Transaction Emission

**Basic Structure:**

```rust
use xrpl_wasm_std::core::emit::tx_codec::{TxEmitBuffer, tx_add};
use xrpl_wasm_std::core::types::account_id::AccountID;
use xrpl_wasm_std::core::types::amount::Amount;
use xrpl_wasm_std::sfield;

let mut buf = TxEmitBuffer::new();

// Add transaction type
if tx_add::<u16>(&mut buf, sfield::TransactionType, &/* transaction type */).is_err() {
    return -1;
}

// Add account field
let account = AccountID(/* account bytes */);
if tx_add::<AccountID>(&mut buf, sfield::Account, &account).is_err() {
    return -1;
}

// Add destination
let destination = AccountID(/* destination bytes */);
if tx_add::<AccountID>(&mut buf, sfield::Destination, &destination).is_err() {
    return -1;
}

// Add amount
let amount = Amount::xrp_drops(1000000);
if tx_add::<Amount>(&mut buf, sfield::Amount, &amount).is_err() {
    return -1;
}

// Emit the transaction
if buf.emit().is_err() {
    return -1;
}
```

**Transaction Types:**

Common transaction type codes:

```rust
const TT_PAYMENT: u16 = 0;
const TT_ESCROW_CREATE: u16 = 1;
const TT_ESCROW_FINISH: u16 = 2;
const TT_ACCOUNT_SET: u16 = 3;
const TT_TRUST_SET: u16 = 20;
const TT_OFFER_CREATE: u16 = 7;
// ... and more
```

#### Example Usage

**Emit a Payment Transaction:**

```rust
use xrpl_wasm_std::core::emit::tx_codec::{TxEmitBuffer, tx_add};
use xrpl_wasm_std::core::types::account_id::AccountID;
use xrpl_wasm_std::core::types::amount::Amount;
use xrpl_wasm_std::sfield;

const TT_PAYMENT: u16 = 0;

fn emit_payment(from: &AccountID, to: &AccountID, amount: u64) -> i32 {
    let mut buf = TxEmitBuffer::new();
    
    // Transaction type: Payment
    if tx_add::<u16>(&mut buf, sfield::TransactionType, &TT_PAYMENT).is_err() {
        return -1;
    }
    
    // Source account
    if tx_add::<AccountID>(&mut buf, sfield::Account, from).is_err() {
        return -1;
    }
    
    // Destination account
    if tx_add::<AccountID>(&mut buf, sfield::Destination, to).is_err() {
        return -1;
    }
    
    // Amount to send
    let xrp_amount = Amount::xrp_drops(amount);
    if tx_add::<Amount>(&mut buf, sfield::Amount, &xrp_amount).is_err() {
        return -1;
    }
    
    // Emit the transaction
    if buf.emit().is_err() {
        trace("Failed to emit transaction");
        return -1;
    }
    
    trace("Payment transaction emitted successfully");
    0
}
```

**Best Practices:**

1. **Always Check Return Values** - Every `tx_add()` and `emit()` call can fail
2. **Validate Before Emitting** - Check all parameters before constructing the transaction
3. **Use Correct Types** - Ensure field types match XRPL specifications
4. **Emit Once** - Call `emit()` only once per transaction buffer

### Emit Events

Events allow smart contracts to log important state changes and actions. These events can be monitored by off-chain applications to track contract activity.

#### Event System

**Basic Event Emission:**

```rust
use xrpl_wasm_std::core::event::codec_v3::{EventBuffer, event_add};
use xrpl_wasm_std::core::types::account_id::AccountID;

let mut event = EventBuffer::new();

// Add event fields
let account = AccountID(/* account bytes */);
if event_add::<AccountID>(&mut event, "from", &account).is_err() {
    return -1;
}

let amount: u64 = 1000000;
if event_add::<u64>(&mut event, "amount", &amount).is_err() {
    return -1;
}

// Emit the event with a name
if event.emit("Transfer").is_err() {
    return -1;
}
```

**Supported Field Types:**

- `u32`, `u64` - Numeric values
- `i32`, `i64` - Signed integers  
- `AccountID` - Account identifiers
- `[u8; N]` - Fixed-size byte arrays
- Strings (as byte arrays)

**Complete Event Example:**

```rust
use xrpl_wasm_std::core::event::codec_v3::{EventBuffer, event_add};
use xrpl_wasm_std::core::types::account_id::AccountID;
use xrpl_wasm_std::host::trace::trace;

fn emit_transfer_event(from: &AccountID, to: &AccountID, amount: u64) -> i32 {
    let mut buf = EventBuffer::new();
    
    // Sender account
    if event_add::<AccountID>(&mut buf, "from", from).is_err() {
        trace("Failed to add 'from' field");
        return -1;
    }
    
    // Receiver account
    if event_add::<AccountID>(&mut buf, "to", to).is_err() {
        trace("Failed to add 'to' field");
        return -1;
    }
    
    // Transfer amount
    if event_add::<u64>(&mut buf, "amount", &amount).is_err() {
        trace("Failed to add 'amount' field");
        return -1;
    }
    
    // Emit the event
    if buf.emit("Transfer").is_err() {
        trace("Failed to emit event");
        return -1;
    }
    
    trace("Transfer event emitted successfully");
    0
}
```

#### Event Best Practices

1. **Descriptive Names** - Use clear event names like "Transfer", "Approval", "StateChanged"
2. **Essential Data Only** - Include only necessary information to keep events small
3. **Error Handling** - Always check return values from `event_add()` and `emit()`
4. **Consistent Schema** - Keep event structure consistent across calls
5. **Document Events** - Document what each event means and when it's emitted

### Complete Token Contract Example

Here's a complete example combining parameters, storage, transactions, and events:

```rust
#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_wasm_macros::wasm_export;
use xrpl_wasm_std::core::types::account_id::AccountID;
use xrpl_wasm_std::core::data::codec::{get_data, set_data};
use xrpl_wasm_std::core::event::codec_v3::{EventBuffer, event_add};
use xrpl_wasm_std::host::trace::{trace, trace_num};

// Custom exit handler for contract errors
fn contract_exit(message: &str, code: i32) -> i32 {
    trace("Contract error:");
    trace(message);
    trace_num("Error code:", code as i64);
    code
}

/// Transfer tokens from one account to another
#[wasm_export(exit = contract_exit, instance(owner: AccountID, max_supply: u64))]
fn transfer(from: AccountID, to: AccountID, amount: u64) -> i32 {
    trace("Starting transfer");
    
    // Validate amount against max supply
    if amount > max_supply {
        return contract_exit("Amount exceeds max supply", -1);
    }
    
    // Check sender balance
    let sender_balance = match get_data::<u64>(&from, "balance") {
        Some(bal) => bal,
        None => {
            return contract_exit("Sender has no balance", -2);
        }
    };
    
    if sender_balance < amount {
        return contract_exit("Insufficient balance", -3);
    }
    
    // Get receiver balance (default to 0 if not found)
    let receiver_balance = get_data::<u64>(&to, "balance").unwrap_or(0);
    
    // Update balances
    if let Err(e) = set_data::<u64>(&from, "balance", sender_balance - amount) {
        return e;
    }
    
    if let Err(e) = set_data::<u64>(&to, "balance", receiver_balance + amount) {
        return e;
    }
    
    // Emit transfer event
    let mut event = EventBuffer::new();
    event_add::<AccountID>(&mut event, "from", &from).ok();
    event_add::<AccountID>(&mut event, "to", &to).ok();
    event_add::<u64>(&mut event, "amount", &amount).ok();
    event.emit("Transfer").ok();
    
    trace_num("Transfer completed, amount:", amount as i64);
    0
}

/// Get balance for an account
#[wasm_export(exit = contract_exit)]
fn balance_of(account: AccountID) -> i32 {
    let balance = get_data::<u64>(&account, "balance").unwrap_or(0);
    trace_num("Balance:", balance as i64);
    0
}

/// Mint new tokens (owner only)
#[wasm_export(exit = contract_exit, instance(owner: AccountID, max_supply: u64))]
fn mint(recipient: AccountID, amount: u64) -> i32 {
    // Get current caller from transaction
    use xrpl_wasm_std::core::current_tx::contract_call::{ContractCall, get_current_contract_call};
    use xrpl_wasm_std::core::current_tx::traits::TransactionCommonFields;
    
    let contract_call: ContractCall = get_current_contract_call();
    let caller = contract_call.get_account().unwrap();
    
    // Only owner can mint
    if caller != owner {
        return contract_exit("Only owner can mint", -1);
    }
    
    // Check against max supply
    if amount > max_supply {
        return contract_exit("Mint amount exceeds max supply", -2);
    }
    
    // Get current balance
    let current_balance = get_data::<u64>(&recipient, "balance").unwrap_or(0);
    
    // Update balance
    if let Err(e) = set_data::<u64>(&recipient, "balance", current_balance + amount) {
        return e;
    }
    
    // Emit mint event
    let mut event = EventBuffer::new();
    event_add::<AccountID>(&mut event, "to", &recipient).ok();
    event_add::<u64>(&mut event, "amount", &amount).ok();
    event.emit("Mint").ok();
    
    trace_num("Minted tokens:", amount as i64);
    0
}
```

**Key Features Demonstrated:**

1. **Parameter Handling** - Uses `#[wasm_export]` macro for clean parameter extraction
2. **Instance Parameters** - Owner and max_supply persist across all calls
3. **Storage Operations** - Reads and writes token balances efficiently
4. **Event Emission** - Logs Transfer and Mint events
5. **Error Handling** - Custom exit handler with descriptive messages
6. **Authorization** - Checks caller permissions for mint function

---

## API Reference

> **⚠️ IMPORTANT:** The code examples in this API reference section are **illustrative only** and may not compile due to API changes.
>
> **For working, tested code that is guaranteed to compile and run correctly, please refer to the [complete examples](#examples).**
>
> The examples below demonstrate concepts and patterns, but the actual API may have changed. Always refer to the working examples for copy-pastable code.

### Transaction Access

The XRPL WASM Standard Library provides type-safe access to transaction data through the `current_tx` module.

#### EscrowFinish Transaction

```rust ignore
use xrpl_wasm_stdlib::core::current_tx::escrow_finish::EscrowFinish;

let tx = EscrowFinish;

// Get the account finishing the escrow
let account = tx.get_account().unwrap();

// Get the destination account (receives funds if released)
let destination = tx.get_destination().unwrap();

// Get the escrow sequence number
let escrow_sequence = tx.get_escrow_sequence().unwrap();
```

#### Field Access

```rust
use xrpl_wasm_stdlib::core::current_tx::escrow_finish::EscrowFinish;
use xrpl_wasm_stdlib::core::current_tx::traits::TransactionCommonFields;
use xrpl_wasm_stdlib::sfield;

let tx = EscrowFinish;

// Access transaction fields using trait methods
let fee_amount = tx.get_fee().ok(); // Returns Amount
let account_id = tx.get_account().ok(); // Returns AccountID
let sequence = tx.get_sequence().ok(); // Returns u32

// EscrowFinish-specific fields (when using EscrowFinishFields trait)
// let owner = tx.get_owner().ok();
// let offer_sequence = tx.get_offer_sequence().ok();
```

### Ledger Objects

Access current ledger state through the `ledger_objects` module.

#### Account Information

```rust
use xrpl_wasm_stdlib::core::ledger_objects::account_root::get_account_balance;
use xrpl_wasm_stdlib::core::types::account_id::AccountID;
use xrpl_wasm_stdlib::sfield;

let account = AccountID::from([0u8; 20]); // Replace with real account

// Get XRP balance (in drops) - returns Option<Amount>
let balance = get_account_balance(&account);

// Use host functions to get account fields directly
// (Note: Specific helper functions may vary based on current API)
```

#### NFT Objects

```rust ignore
// NFT functionality uses the NFToken type
use xrpl_wasm_stdlib::core::types::nft::NFToken;
use xrpl_wasm_stdlib::core::types::account_id::AccountID;

let owner = AccountID::from([0u8; 20]);
let nft_id_bytes = [0u8; 32]; // 32-byte NFT identifier
let nft_token = NFToken::new(nft_id_bytes);

// Check ownership
let is_owned = nft_token.is_owned_by(&owner);

// Get NFT metadata
let nft_flags = nft_token.flags()?;
let transfer_fee = nft_token.transfer_fee()?;
let issuer = nft_token.issuer()?;
let taxon = nft_token.taxon()?;
let token_sequence = nft_token.token_sequence()?;

// Check individual flags efficiently (no additional host calls)
if nft_flags.is_burnable() {
    // NFT can be burned by issuer
}
if nft_flags.is_transferable() {
    // NFT can be transferred
}

// Get NFT URI
let uri = nft_token.uri(&owner)?;
```

### Type System

#### Core Types

```rust ignore
use xrpl_wasm_stdlib::core::types::{
    account_id::AccountID,           // 20-byte XRPL account identifier
    amount::Amount, // Token amounts (XRP, IOU, MPT)
};
use xrpl_wasm_stdlib::types::NFT;      // [u8; 32] NFT identifier

// Create AccountID from r-address (if r_address macro exists)
// let account = xrpl_wasm_stdlib::r_address!("rN7n7otQDd6FczFgLdSqtcsAUxDkw6fzRH");

// Create from raw bytes
let account = AccountID::from([0u8; 20]);

// NFT as byte array
let nft: NFT = [0u8; 32];

// Note: High-level string parsing functions may not be available
// Use the working examples for guaranteed compilable code
```

#### Keylet Generation

Keylets are used to locate objects in the ledger:

```rust ignore
use xrpl_wasm_stdlib::core::types::keylets::{
    account_keylet,
    line_keylet,
    escrow_keylet,
    oracle_keylet,
};
use xrpl_wasm_stdlib::core::types::account_id::AccountID;
use xrpl_wasm_stdlib::core::types::amount::asset::Asset;

let account = AccountID::from([0u8; 20]);
let sequence = 12345i32;

// Account keylet
let keylet = account_keylet(&account);

// Trust line keylet (requires Asset types)
let asset1 = Asset::XRP(XrpAsset {});
let asset2 = Asset::IOU(IouAsset::new(issuer, currency));
let keylet = line_keylet(&account, &asset1, &asset2);

// Escrow keylet
let keylet = escrow_keylet(&account, sequence);

// Oracle keylet
let document_id = 1i32;
let keylet = oracle_keylet(&account, document_id);
```

### Host Functions

Low-level host function access through the `host` module.

#### Ledger Access

```rust
// Use the high-level trait methods instead of low-level host functions
use xrpl_wasm_stdlib::core::ledger_objects::account_root::AccountRoot;
use xrpl_wasm_stdlib::core::ledger_objects::traits::AccountFields;
use xrpl_wasm_stdlib::core::types::account_id::AccountID;
use xrpl_wasm_stdlib::core::types::keylets::account_keylet;
use xrpl_wasm_stdlib::host::cache_ledger_obj;
use xrpl_wasm_stdlib::host::Error;

// The correct approach is to use the trait methods
fn main() {
    let account = AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
    let account_keylet = account_keylet(&account).unwrap_or_panic();
    let slot = unsafe { cache_ledger_obj(account_keylet.as_ptr(), account_keylet.len(), 0) };
    if slot < 0 {
        return;
    }

    let account_root = AccountRoot { slot_num: slot };
    let balance = account_root.balance();  // Returns Option<Amount>
    let sequence = account_root.sequence(); // Returns u32
}
```

#### Transaction Fields

```rust
// Use the high-level trait methods instead of low-level host functions
use xrpl_wasm_stdlib::core::current_tx::escrow_finish::EscrowFinish;
use xrpl_wasm_stdlib::core::current_tx::traits::{TransactionCommonFields, EscrowFinishFields};

fn main() {
    let tx = EscrowFinish;

    // Access common transaction fields
    let account = tx.get_account(); // AccountID
    let fee = tx.get_fee(); // Amount
    let sequence = tx.get_sequence(); // u32

    // Access EscrowFinish-specific fields
    let owner = tx.get_owner(); // AccountID
    let offer_sequence = tx.get_offer_sequence(); // u32
    let condition = tx.get_condition(); // Option<Condition>
}
```

### Error Handling

The library uses custom `Result` types for comprehensive error handling:

```rust
use xrpl_wasm_stdlib::core::current_tx::escrow_finish::EscrowFinish;
use xrpl_wasm_stdlib::core::current_tx::traits::TransactionCommonFields;
use xrpl_wasm_stdlib::core::ledger_objects::account_root::{get_account_balance, AccountRoot};
use xrpl_wasm_stdlib::core::ledger_objects::traits::AccountFields;
use xrpl_wasm_stdlib::core::types::account_id::AccountID;
use xrpl_wasm_stdlib::core::types::amount::Amount;
use xrpl_wasm_stdlib::core::types::keylets::account_keylet;
use xrpl_wasm_stdlib::host::{cache_ledger_obj, Error, Result};
use xrpl_wasm_stdlib::host::Result::{Ok, Err};

fn process_escrow() -> Result<i32> {
    let tx = EscrowFinish;

    // Chain operations with ?
    let account = match tx.get_account() {
        Ok(acc) => acc,
        Err(e) => return Err(e), // Invalid transaction
    };

    let balance = get_account_balance(&account);

    // Handle specific errors - create AccountRoot to access account fields
    let account_keylet = match account_keylet(&account) {
        Ok(keylet) => keylet,
        Err(e) => return Err(e), // Invalid account
    };

    let slot = unsafe { cache_ledger_obj(account_keylet.as_ptr(), account_keylet.len(), 0) };
    if slot < 0 {
        return Err(Error::from_code(slot));
    }

    let account_root = AccountRoot { slot_num: slot };
    match account_root.sequence() {
        Ok(sequence) => {
            // Use sequence
        },
        Err(e) => {
            // Handle missing field or other error
            return Err(e);
        },
    }

    return Ok(match balance {
        Ok(Some(Amount::XRP { num_drops })) if num_drops > 10_000_000 => 1,
        _ => 0,
    })
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

**📁 View complete example:** [`examples/smart-escrows/hello_world/`](https://github.com/ripple/xrpl-wasm-stdlib/tree/main/examples/smart-escrows/hello_world/)

**Key learning points:**

- Basic contract structure with `#![no_std]` and `#![no_main]`
- Using `#[unsafe(no_mangle)]` for the entry point function
- Simple error handling with pattern matching
- Trace logging for debugging

**Files:**

- [`src/lib.rs`](https://github.com/ripple/xrpl-wasm-stdlib/blob/main/examples/smart-escrows/hello_world/src/lib.rs) - Main contract code
- [`README.md`](https://github.com/ripple/xrpl-wasm-stdlib/blob/main/examples/smart-escrows/hello_world/README.md) - Detailed explanation
- [`runTest.js`](https://github.com/ripple/xrpl-wasm-stdlib/blob/main/examples/smart-escrows/hello_world/runTest.js) - Integration test

### Oracle Example

A price-based escrow that releases funds when an asset price meets conditions.

**📁 View complete example:** [`examples/smart-escrows/oracle/`](https://github.com/ripple/xrpl-wasm-stdlib/tree/main/examples/smart-escrows/oracle/)

**Key concepts demonstrated:**

- External data integration through oracles
- Price threshold logic
- Error handling for missing oracle data
- Real-world conditional logic

**Files:**

- [`src/lib.rs`](https://github.com/ripple/xrpl-wasm-stdlib/blob/main/examples/smart-escrows/oracle/src/lib.rs) - Oracle price checking logic
- [`README.md`](https://github.com/ripple/xrpl-wasm-stdlib/blob/main/examples/smart-escrows/oracle/README.md) - Oracle integration guide
- [`runTest.js`](https://github.com/ripple/xrpl-wasm-stdlib/blob/main/examples/smart-escrows/oracle/runTest.js) - Price simulation test

### KYC Example

A compliance-focused escrow that requires credential verification.

**📁 View complete example:** [`examples/smart-escrows/kyc/`](https://github.com/ripple/xrpl-wasm-stdlib/tree/main/examples/smart-escrows/kyc/)

**Key concepts demonstrated:**

- Credential-based verification
- Trusted issuer validation
- Signature verification
- Expiration checking
- Compliance patterns

**Files:**

- [`src/lib.rs`](https://github.com/ripple/xrpl-wasm-stdlib/blob/main/examples/smart-escrows/kyc/src/lib.rs) - KYC credential verification
- [`README.md`](https://github.com/ripple/xrpl-wasm-stdlib/blob/main/examples/smart-escrows/kyc/README.md) - Compliance implementation guide
- [`runTest.js`](https://github.com/ripple/xrpl-wasm-stdlib/blob/main/examples/smart-escrows/kyc/runTest.js) - Credential verification test

### Advanced Examples

#### Multi-Signature Notary

**📁 [`examples/smart-escrows/notary/`](https://github.com/ripple/xrpl-wasm-stdlib/tree/main/examples/smart-escrows/notary/)**

- Requires multiple signature approvals
- Implements threshold signing logic
- Demonstrates complex authorization patterns

#### NFT Ownership Verification

**📁 [`examples/smart-escrows/nft_owner/`](https://github.com/ripple/xrpl-wasm-stdlib/tree/main/examples/smart-escrows/nft_owner/)**

- Releases funds based on NFT ownership
- Shows how to query NFT ledger objects
- Demonstrates asset-based conditions

#### Time-Based Ledger Sequence

**📁 [`examples/smart-escrows/ledger_sqn/`](https://github.com/ripple/xrpl-wasm-stdlib/tree/main/examples/smart-escrows/ledger_sqn/)**

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

**🌐 Open the web UI:** [https://ripple.github.io/xrpl-wasm-stdlib/ui/](https://ripple.github.io/xrpl-wasm-stdlib/ui/)

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

```rust ignore
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

```rust ignore
// Good: Call once, use cached result
let account = tx.get_account();
let balance = get_account_balance(&account);
// Create AccountRoot to access account fields
let account_keylet = account_keylet(&account);
let slot = cache_ledger_obj(&account_keylet);
let account_root = AccountRoot { slot_num: slot };
let sequence = account_root.sequence();

// Bad: Multiple calls for same data
let balance = get_account_balance(&tx.get_account());
// Bad: Multiple calls - should cache the account and keylet
let account_keylet = account_keylet(&tx.get_account());
let slot = cache_ledger_obj(&account_keylet);
let account_root = AccountRoot { slot_num: slot };
let sequence = account_root.sequence();
```

**Efficient ledger object access:**

```rust ignore
// Cache ledger objects for multiple field access using traits
let account = AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
let account_keylet = account_keylet(&account).unwrap_or_panic();
let slot = unsafe { cache_ledger_obj(account_keylet.as_ptr(), account_keylet.len(), 0) };
let account_root = AccountRoot { slot_num: slot };

// Use trait methods to access fields efficiently
let balance = account_root.balance();        // Option<Amount>
let sequence = account_root.sequence();      // u32
let owner_count = account_root.owner_count(); // u32
```

**Memory usage optimization:**

```rust ignore
// Use stack-based allocation
let mut accounts = [AccountID::default(); 10];

// Reuse buffers for transaction fields
let mut buffer = [0u8; 64];
let len1 = unsafe { get_tx_field(sfield::Account, buffer[..20].as_mut_ptr(), 20) };
let len2 = unsafe { get_tx_field(sfield::Destination, buffer[20..40].as_mut_ptr(), 20) };
```

### Troubleshooting

#### Common Build Issues

| Issue                            | Solution                                             |
| -------------------------------- | ---------------------------------------------------- |
| `wasm32v1-none` target not found | `rustup target add wasm32v1-none`                    |
| Link errors                      | Check `crate-type = ["cdylib"]` in Cargo.toml        |
| Binary too large                 | Use release profile optimizations                    |
| Missing exports                  | Ensure `#[unsafe(no_mangle)]` on `finish()` function |
| Compilation errors               | Check `#![no_std]` and avoid std library usage       |

#### Common Runtime Issues

| Issue                    | Cause                   | Solution                                    |
| ------------------------ | ----------------------- | ------------------------------------------- |
| Function not found       | WASM export missing     | Check `#[unsafe(no_mangle)]` on entry point |
| Memory access violation  | Buffer overflow         | Verify buffer sizes and bounds              |
| Cache full (NoFreeSlots) | Too many cached objects | Minimize `cache_ledger_obj` calls           |
| Field not found          | Missing ledger field    | Handle `FieldNotFound` errors               |
| Invalid field data       | Malformed field         | Validate input data                         |

#### Debugging Techniques

**Add trace statements:**

```rust
use xrpl_wasm_stdlib::host::trace::{trace, trace_data, trace_num, DataRepr};
use xrpl_wasm_stdlib::core::current_tx::escrow_finish::EscrowFinish;
use xrpl_wasm_stdlib::core::current_tx::traits::TransactionCommonFields;
use xrpl_wasm_stdlib::host::Result::{Ok, Err};

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    trace("Contract starting").ok();

    let tx = EscrowFinish;
    let account = match tx.get_account() {
        Ok(acc) => {
            trace_data("Account", &acc.0, DataRepr::AsHex).ok();
            acc
        },
        Err(e) => {
            trace_num("Error getting account: {:?}", e as i64).ok();
            return 0;
        }
    };

    // More logic with tracing...
    1 // Return 1 to complete the function
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

If you're interested in contributing to the XRPL WebAssembly Standard Library, please see our [CONTRIBUTING.md](https://github.com/ripple/xrpl-wasm-stdlib/blob/main/CONTRIBUTING.md) for detailed guidelines on:

- Development setup and workflow
- Code standards and style guidelines
- Pull request process
- Testing requirements
- Release procedures

We welcome contributions of all kinds, from bug fixes and documentation improvements to new examples and library features!
