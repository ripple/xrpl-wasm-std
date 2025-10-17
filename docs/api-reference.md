# API Reference

Complete reference for the xrpl-wasm-std library APIs.

## Table of Contents

- [Transaction Access](#transaction-access)
- [Ledger Object Access](#ledger-object-access)
- [Type System](#type-system)
- [Field Access](#field-access)
- [Keylet Generation](#keylet-generation)
- [Cryptographic Functions](#cryptographic-functions)
- [State Management](#state-management)
- [Logging and Debugging](#logging-and-debugging)
- [Error Handling](#error-handling)
- [Memory Model](#memory-model)

## Transaction Access

Access fields from the current transaction being processed.

### EscrowFinish Transaction

```rust
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

### Transaction Common Fields

All transaction types support these common fields:

| Method                  | Return Type           | Description              |
| ----------------------- | --------------------- | ------------------------ |
| `get_account()`         | `Result<AccountID>`   | Transaction sender       |
| `get_fee()`             | `Result<u64>`         | Transaction fee in drops |
| `get_sequence()`        | `Result<u32>`         | Account sequence number  |
| `get_flags()`           | `Result<u32>`         | Transaction flags        |
| `get_signing_pub_key()` | `Result<PublicKey>`   | Signing public key       |
| `get_source_tag()`      | `Result<Option<u32>>` | Optional source tag      |

### EscrowFinish Specific Fields

| Method                 | Return Type                   | Description            |
| ---------------------- | ----------------------------- | ---------------------- |
| `get_owner()`          | `Result<AccountID>`           | Escrow creator account |
| `get_offer_sequence()` | `Result<u32>`                 | EscrowCreate sequence  |
| `get_condition()`      | `Result<Option<Condition>>`   | Crypto-condition       |
| `get_fulfillment()`    | `Result<Option<Fulfillment>>` | Fulfillment data       |

## Ledger Object Access

Access ledger objects like accounts, escrows, and other on-ledger data.

### Current Escrow

```rust
use xrpl_wasm_std::core::ledger_objects::current_escrow::get_current_escrow;

// Get the current escrow object being processed
let escrow = get_current_escrow();

// Access escrow fields
let destination = escrow.get_destination()?;       // Escrow destination
let amount = escrow.get_amount()?;                 // Escrowed amount
let condition = escrow.get_condition()?;           // Optional condition
let cancel_after = escrow.get_cancel_after()?;     // Cancel timestamp
let finish_after = escrow.get_finish_after()?;     // Finish timestamp
let data = escrow.get_data()?;                     // Custom data field
```

### Account Information

```rust
use xrpl_wasm_std::core::ledger_objects::account::get_account_balance;

// Get account balance in drops
let balance = get_account_balance(&account)?;  // Returns u64 (drops)
```

### General Ledger Objects

```rust
use xrpl_wasm_std::host::cache_ledger_obj;

// Cache a ledger object by keylet
let slot = cache_ledger_obj(&keylet.data)?;

// Access fields from cached object
use xrpl_wasm_std::core::ledger_objects::ledger_object::*;
let account_id = get_account_id_field(slot, sfield::Account)?;
let amount = get_amount_field(slot, sfield::Balance)?;
let blob_data = get_blob_field(slot, sfield::Data)?;
```

### Escrow Fields

| Method               | Return Type                 | Description              |
| -------------------- | --------------------------- | ------------------------ |
| `get_destination()`  | `Result<AccountID>`         | Destination account      |
| `get_amount()`       | `Result<u64>`               | Escrowed amount in drops |
| `get_condition()`    | `Result<Option<Condition>>` | Crypto-condition         |
| `get_cancel_after()` | `Result<Option<u32>>`       | Cancel timestamp         |
| `get_finish_after()` | `Result<Option<u32>>`       | Finish timestamp         |
| `get_data()`         | `Result<Option<Blob>>`      | Custom data field        |

## Type System

Core types for XRPL data representation.

### Basic Types

```rust
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
let amount: u64 = /* ... */;

// Transaction type enumeration
let tx_type: TransactionType = TransactionType::EscrowFinish;
```

### Type Specifications

| Type          | Size                | Description                  |
| ------------- | ------------------- | ---------------------------- |
| `AccountID`   | 20 bytes            | XRPL account identifier      |
| `Hash256`     | 32 bytes            | Transaction or ledger hash   |
| `PublicKey`   | 33 bytes            | Compressed public key        |
| `Blob`        | Variable (max 1024) | Binary data                  |
| `Condition`   | Variable            | Crypto-condition             |
| `Fulfillment` | Variable            | Crypto-condition fulfillment |

### Amount Types

```rust
use xrpl_wasm_std::core::types::amount::*;

// XRP amounts (in drops)
let xrp_amount: u64 = 1_000_000; // 1 XRP

// Token amounts
let token_amount: TokenAmount = /* ... */;

// Currency codes
let currency: CurrencyCode = /* ... */;

// Multi-purpose token ID
let mpt_id: MptId = /* ... */;
```

## Field Access

### Direct Field Access

Access top-level fields from transactions or objects:

```rust
use xrpl_wasm_std::host::get_tx_field;
use xrpl_wasm_std::sfield;

// Get a field by its field code
let mut buffer = [0u8; 20];
let len = get_tx_field(sfield::Account, 0, &mut buffer)?;
let account = AccountID::from_buffer(&buffer[..len])?;
```

### Nested Field Access

Access fields within complex objects using locators:

```rust
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

### STArray Navigation

**Important**: For STArray navigation, omit the intermediate object wrapper:

- ✅ `Memos → [0] → MemoType`
- ❌ `Memos → [0] → Memo → MemoType`

### Field Codes

Common field codes from `sfield` module:

| Field         | Type      | Description             |
| ------------- | --------- | ----------------------- |
| `Account`     | AccountID | Transaction sender      |
| `Destination` | AccountID | Transaction destination |
| `Amount`      | UInt64    | Amount in drops         |
| `Fee`         | UInt64    | Transaction fee         |
| `Sequence`    | UInt32    | Account sequence        |
| `Flags`       | UInt32    | Transaction flags       |
| `Condition`   | Blob      | Crypto-condition        |
| `Fulfillment` | Blob      | Fulfillment data        |

## Keylet Generation

Generate unique identifiers for ledger objects.

### Account Keylets

```rust
use xrpl_wasm_std::core::types::keylets::*;

// Account keylet
let account_key = account_keylet(&account_id)?;

// Account root object access
let slot = cache_ledger_obj(&account_key.data)?;
```

### Object-Specific Keylets

```rust
// Escrow keylet
let escrow_key = escrow_keylet(&owner, sequence)?;

// Oracle keylet
let oracle_key = oracle_keylet(&owner, document_id)?;

// Credential keylet
let cred_key = credential_keylet(&subject, &issuer, credential_type)?;

// NFT keylet
let nft_key = nft_keylet(&nft_id)?;

// Check keylet
let check_key = check_keylet(&sender, sequence)?;
```

### Available Keylet Functions

| Function            | Parameters                      | Description         |
| ------------------- | ------------------------------- | ------------------- |
| `account_keylet`    | `&AccountID`                    | Account root object |
| `escrow_keylet`     | `&AccountID, u32`               | Escrow object       |
| `oracle_keylet`     | `&AccountID, i32`               | Oracle object       |
| `credential_keylet` | `&AccountID, &AccountID, &[u8]` | Credential object   |
| `nft_offer_keylet`  | `&AccountID, u32`               | NFT offer           |
| `check_keylet`      | `&AccountID, u32`               | Check object        |
| `paychan_keylet`    | `&AccountID, &AccountID, u32`   | Payment channel     |

## Cryptographic Functions

### SHA-512 Half

```rust
use xrpl_wasm_std::host::compute_sha512_half;

// Compute SHA-512 half (first 32 bytes)
let data = b"data to hash";
let mut hash = [0u8; 32];
compute_sha512_half(data, &mut hash)?;
```

### Signature Verification

```rust
use xrpl_wasm_std::host::check_sig;

// Verify signature
let message = b"signed message";
let signature = /* signature bytes */;
let public_key = /* public key */;

let is_valid = check_sig(message, &signature, &public_key)?;
if is_valid {
    // Signature is valid
}
```

## State Management

The only allowed state modification is updating the escrow's data field.

### Update Escrow Data

```rust
use xrpl_wasm_std::host::update_data;

// Update the escrow's data field (max 256 bytes)
let new_data = b"execution result";
update_data(new_data)?;
```

**Important**: This is the ONLY state modification allowed in smart escrows. All other ledger access is read-only.

## Logging and Debugging

Debug output during development and testing.

### Basic Tracing

```rust
use xrpl_wasm_std::host::trace::{trace, trace_data, trace_num, DataRepr};

// Simple text trace
trace("Processing escrow finish")?;

// Trace with data
trace_data("Account ID", &account_id, DataRepr::AsHex)?;
trace_data("Message", b"Hello", DataRepr::AsUTF8)?;

// Trace with number
trace_num("Balance", balance as i64)?;
```

### Data Representation Options

| Option                | Description         | Example       |
| --------------------- | ------------------- | ------------- |
| `DataRepr::AsHex`     | Hexadecimal format  | `A1B2C3...`   |
| `DataRepr::AsUTF8`    | UTF-8 string        | `Hello World` |
| `DataRepr::AsAccount` | XRPL address format | `rAccount...` |

### Tracing Functions

| Function        | Parameters              | Description          |
| --------------- | ----------------------- | -------------------- |
| `trace`         | `&str`                  | Simple message       |
| `trace_data`    | `&str, &[u8], DataRepr` | Data with format     |
| `trace_num`     | `&str, i64`             | Numeric value        |
| `trace_account` | `&str, &AccountID`      | Account as r-address |

## Error Handling

### Error Types

```rust
use xrpl_wasm_std::host::Error;

pub enum Error {
    InternalError = -1,        // Internal invariant violation
    FieldNotFound = -2,        // Requested field doesn't exist
    BufferTooSmall = -3,       // Buffer too small for data
    NoFreeSlots = -8,          // No cache slots available
    PointerOutOfBound = -13,   // Memory access violation
    InvalidField = -25,        // Invalid field access
    InvalidParams = -26,       // Invalid function parameters
}
```

### Error Handling Patterns

```rust
// Pattern 1: Early return for errors
let account = match tx.get_account() {
    Ok(acc) => acc,
    Err(_) => return 0, // Safe default: keep locked
};

// Pattern 2: Using ? operator in helper functions
fn check_balance(account: &AccountID) -> Result<bool, Error> {
    let balance = get_account_balance(account)?;
    Ok(balance > 1_000_000)
}

// Pattern 3: Handle specific errors
match get_account_balance(&account) {
    Ok(balance) => { /* use balance */ },
    Err(Error::FieldNotFound) => { /* handle missing field */ },
    Err(Error::NoFreeSlots) => { /* handle cache full */ },
    Err(_) => { /* handle other errors */ },
}
```

### Best Practices

1. **Always handle errors** - Never use `unwrap()` in production
2. **Fail safely** - Return 0 on errors to keep escrow locked
3. **Use specific error handling** when different errors need different responses
4. **Log errors** during development using trace functions

## Memory Model

### Stack-Based Allocation

All allocations are on the stack with fixed buffer sizes:

```rust
// Fixed-size buffers for each type
let mut account_buffer = [0u8; 20];      // AccountID
let mut hash_buffer = [0u8; 32];         // Hash256
let mut pubkey_buffer = [0u8; 33];       // PublicKey
let mut data_buffer = [0u8; 1024];       // Blob (max size)
```

### Memory Constraints

- **No heap allocation** - Compatible with `no_std`
- **Fixed buffer sizes** - Predetermined for each XRPL type
- **Stack only** - All data lives on the call stack
- **Linear memory** - WASM linear memory for host communication

### Buffer Sizes

| Type      | Buffer Size | Description             |
| --------- | ----------- | ----------------------- |
| AccountID | 20 bytes    | Account identifier      |
| Hash256   | 32 bytes    | Transaction/ledger hash |
| PublicKey | 33 bytes    | Compressed public key   |
| Blob      | 1024 bytes  | Variable data (max)     |
| Locator   | 64 bytes    | Field location path     |

### Cache Management

Smart escrows have limited cache slots for ledger objects:

```rust
// Maximum 255 cache slots available
let slot = cache_ledger_obj(&keylet)?; // May fail with NoFreeSlots

// Access cached object by slot number
let balance = get_amount_field(slot, sfield::Balance)?;
```

**Important**: Cache slots are a limited resource. Design your contract to minimize the number of different ledger objects accessed.

## Advanced Usage

### Optional Fields

Many XRPL fields are optional. Handle them appropriately:

```rust
// Optional fields return Option<T>
match escrow.get_condition()? {
    Some(condition) => {
        // Field is present, use condition
    },
    None => {
        // Field is not set
    }
}
```

### Array Access

```rust
use xrpl_wasm_std::host::get_tx_array_len;

// Get array length
let memo_count = get_tx_array_len(sfield::Memos)?;

// Access array elements using locators
for i in 0..memo_count {
    let mut locator = Locator::new();
    locator.pack(sfield::Memos);
    locator.pack(i);
    locator.pack(sfield::MemoData);

    let mut buffer = [0u8; 256];
    let len = get_tx_nested_field(&locator.buffer, &mut buffer)?;
    // Process memo data
}
```

### Working with Addresses

```rust
// Convert classic addresses at compile time
use xrpl_wasm_std::r_address;

const KNOWN_ACCOUNT: [u8; 20] = r_address!("rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh");

// Compare accounts
if account == KNOWN_ACCOUNT {
    // Account matches
}
```

See [Getting Started](getting-started.md) for more examples and [Examples](examples/README.md) for complete use cases.
