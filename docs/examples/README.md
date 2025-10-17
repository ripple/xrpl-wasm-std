# Smart Escrow Examples

This collection demonstrates various smart escrow use cases defined in the [Smart Escrows XLS proposal](https://github.com/XRPLF/XRPL-Standards/discussions/270).

## Examples Overview

| Example                                                      | Description                           | Complexity   | Key Concepts                   |
| ------------------------------------------------------------ | ------------------------------------- | ------------ | ------------------------------ |
| **[hello_world](../../examples/smart-escrows/hello_world/)** | Basic escrow with logging             | Beginner     | Entry point, tracing           |
| **[oracle](../../examples/smart-escrows/oracle/)**           | Price-based release using oracle data | Intermediate | Oracle integration, keylets    |
| **[kyc](../../examples/smart-escrows/kyc/)**                 | Credential-based verification         | Intermediate | Credentials, authorization     |
| **[notary](../../examples/smart-escrows/notary/)**           | Multi-signature authorization         | Intermediate | Address validation, signatures |
| **[nft_owner](../../examples/smart-escrows/nft_owner/)**     | NFT ownership verification            | Advanced     | NFT queries, ownership checks  |
| **[ledger_sqn](../../examples/smart-escrows/ledger_sqn/)**   | Sequence-based release                | Beginner     | Ledger sequence, timing        |

## Quick Start Guide

### 1. Choose Your Starting Point

**New to XRPL smart escrows?** Start with `hello_world`
**Want price-based logic?** Try `oracle`
**Need identity verification?** Check `kyc`
**Building multi-sig escrows?** Use `notary`

### 2. Build and Test

```shell
# Navigate to any example
cd examples/smart-escrows/hello_world

# Build the contract
cargo build --target wasm32v1-none --release

# Run integration tests
cd ../../..
./scripts/run-tests.sh examples/smart-escrows/hello_world
```

### 3. Explore the Code

Each example includes:

- **`src/lib.rs`** - Main contract logic
- **`Cargo.toml`** - Dependencies and build config
- **`README.md`** - Detailed documentation
- **`run_test.js`** - Integration test script

## Example Categories

### Basic Examples

#### hello_world - Your First Smart Escrow

```rust
#[no_mangle]
pub extern "C" fn finish() -> i32 {
    trace("Hello World from smart escrow!")?;
    1  // Always release the escrow
}
```

**Learn**: Entry points, logging, basic structure

#### ledger_sqn - Time-Based Release

```rust
#[no_mangle]
pub extern "C" fn finish() -> i32 {
    let current_seq = get_ledger_sqn();
    let target_seq = 50_000_000; // Release at ledger 50M

    if current_seq >= target_seq { 1 } else { 0 }
}
```

**Learn**: Ledger access, conditional logic

### Authentication Examples

#### kyc - Credential Verification

```rust
#[no_mangle]
pub extern "C" fn finish() -> i32 {
    let tx = EscrowFinish;
    let account = tx.get_account()?;

    // Check if account has KYC credential
    let keylet = credential_keylet(&account, &ISSUER, b"KYC")?;
    match cache_ledger_obj(&keylet.data) {
        Ok(_) => 1,   // Has credential, release
        Err(_) => 0,  // No credential, keep locked
    }
}
```

**Learn**: Credential system, keylet generation, authorization

#### notary - Multi-Signature Authorization

```rust
const NOTARY: [u8; 20] = r_address!("rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh");

#[no_mangle]
pub extern "C" fn finish() -> i32 {
    let tx = EscrowFinish;
    let signer = tx.get_account()?;

    // Only allow release if signed by trusted notary
    if signer == NOTARY { 1 } else { 0 }
}
```

**Learn**: Address constants, signature validation

### Advanced Data Access

#### oracle - Price-Based Conditions

```rust
#[no_mangle]
pub extern "C" fn finish() -> i32 {
    // Get oracle price data
    let oracle_keylet = oracle_keylet(&ORACLE_OWNER, DOCUMENT_ID)?;
    let slot = cache_ledger_obj(&oracle_keylet.data)?;

    // Extract price from PriceDataSeries
    let price = get_price_from_oracle(slot)?;

    // Release if price > threshold
    if price > 1 { 1 } else { 0 }
}
```

**Learn**: Oracle integration, nested field access, data parsing

#### nft_owner - NFT Ownership Checks

```rust
#[no_mangle]
pub extern "C" fn finish() -> i32 {
    let tx = EscrowFinish;
    let account = tx.get_account()?;

    // Check if account owns specific NFT
    let nft = get_nft(&NFT_ID)?;
    let owner = nft.get_owner()?;

    if owner == account { 1 } else { 0 }
}
```

**Learn**: NFT queries, ownership validation

## Code Patterns

### Error Handling Patterns

```rust
// Pattern 1: Early return on error
let account = match tx.get_account() {
    Ok(acc) => acc,
    Err(_) => return 0, // Safe default
};

// Pattern 2: Using ? operator in helpers
fn check_condition() -> Result<bool, Error> {
    let data = get_required_data()?;
    Ok(data.is_valid())
}

// Pattern 3: Graceful degradation
match get_optional_data() {
    Ok(data) => process_data(data),
    Err(_) => use_fallback_logic(),
}
```

### Data Access Patterns

```rust
// Pattern 1: Simple field access
let balance = get_account_balance(&account)?;

// Pattern 2: Nested field extraction
let mut locator = Locator::new();
locator.pack(sfield::Memos);
locator.pack(0);
locator.pack(sfield::MemoData);
let memo = get_tx_nested_field(&locator.buffer, &mut buffer)?;

// Pattern 3: Cache and reuse
let slot = cache_ledger_obj(&keylet)?;
let field1 = get_field_from_slot(slot, sfield::Field1)?;
let field2 = get_field_from_slot(slot, sfield::Field2)?;
```

### Debugging Patterns

```rust
// Pattern 1: Trace execution flow
trace("Starting validation")?;
let result = validate_condition()?;
trace(&format!("Validation result: {}", result))?;

// Pattern 2: Trace data values
trace_data("Account", &account, DataRepr::AsHex)?;
trace_num("Balance", balance as i64)?;

// Pattern 3: Conditional debugging
#[cfg(feature = "debug")]
trace_data("Debug info", &debug_data, DataRepr::AsUTF8)?;
```

## Testing Your Examples

### Local Testing

```shell
# Test specific example
./scripts/run-tests.sh examples/smart-escrows/oracle

# Test all examples
./scripts/run-tests.sh examples/smart-escrows
```

### Using the Web UI

1. Open `ui/index.html` in your browser
2. Connect to WASM Devnet or local node
3. Select a pre-built example or upload your WASM
4. Create test accounts and escrows
5. Test different scenarios interactively

### Integration Test Structure

Each example includes an integration test (`run_test.js`):

```javascript
// Example test structure
const test = async () => {
  // Setup accounts and client
  const client = new xrpl.Client(WSS_URL)
  await client.connect()

  // Create and fund test accounts
  const { wallet: sender } = await createAndFundWallet(client)
  const { wallet: receiver } = await createAndFundWallet(client)

  // Deploy escrow with WASM
  const escrowTx = {
    TransactionType: "EscrowCreate",
    Account: sender.address,
    Destination: receiver.address,
    Amount: "1000000",
    FinishFunction: wasmHex,
  }

  // Test escrow finish
  const finishTx = {
    TransactionType: "EscrowFinish",
    Account: sender.address,
    Owner: sender.address,
    OfferSequence: escrowSequence,
  }

  // Verify results
  assert(result.meta.TransactionResult === "tesSUCCESS")
}
```

## Building Custom Examples

### 1. Start with a Template

Copy the structure from `hello_world`:

```shell
cp -r examples/smart-escrows/hello_world examples/smart-escrows/my-example
cd examples/smart-escrows/my-example
```

### 2. Modify the Contract

Edit `src/lib.rs` for your use case:

```rust
#![no_std]
#![no_main]

use xrpl_wasm_std::core::current_tx::escrow_finish::EscrowFinish;

#[no_mangle]
pub extern "C" fn finish() -> i32 {
    // Your custom logic here
    1  // or 0
}
```

### 3. Update Configuration

Edit `Cargo.toml`:

```toml
[package]
name = "my-example"
version = "0.1.0"
edition = "2021"

[dependencies]
xrpl-wasm-std = { path = "../../../xrpl-wasm-std" }

[lib]
crate-type = ["cdylib"]
```

### 4. Create Tests

Create or modify `run_test.js` for your specific test cases.

### 5. Document Your Example

Create a comprehensive `README.md` following the pattern from other examples.

## Advanced Topics

### Performance Optimization

- **Minimize host function calls** - Cache results when possible
- **Use efficient data structures** - Fixed-size arrays over dynamic allocation
- **Optimize field access** - Group related field accesses together

### Security Considerations

- **Validate all inputs** - Never trust external data
- **Handle errors gracefully** - Always return safe defaults
- **Avoid state mutations** - Only update escrow data when necessary
- **Test edge cases** - Include malicious input scenarios

### Debugging Tips

- **Use trace functions** liberally during development
- **Test with known data** using the `r_address!` macro
- **Verify WASM exports** with `check-wasm-exports.sh`
- **Monitor cache usage** to avoid `NoFreeSlots` errors

## Next Steps

1. **Study the examples** that match your use case
2. **Build and test** them locally
3. **Modify incrementally** to learn the APIs
4. **Create your own** following the patterns
5. **Contribute back** by sharing useful examples

## Common Issues

### Build Problems

- Ensure `wasm32v1-none` target is installed
- Check that `Cargo.toml` has correct crate type
- Verify all dependencies are available

### Runtime Errors

- Check that `finish()` function is exported with `#[no_mangle]`
- Ensure error handling returns 0 for safe defaults
- Verify WASM module is under size limits

### Test Failures

- Confirm network connectivity to test endpoints
- Check that Node.js dependencies are installed
- Verify account funding and transaction parameters

For more help, see:

- **[Getting Started](../getting-started.md)** - Basic setup and first contract
- **[API Reference](../api-reference.md)** - Complete API documentation
- **[Development Guide](../development/building.md)** - Advanced build configuration
