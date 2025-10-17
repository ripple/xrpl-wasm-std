# Testing Guide

Comprehensive guide for testing XRPL smart escrows with xrpl-wasm-std.

## Testing Overview

The xrpl-wasm-std project provides multiple testing approaches:

- **Unit testing** - Test individual functions and logic
- **Integration testing** - Test complete WASM modules on XRPL networks
- **End-to-end testing** - Test full escrow lifecycle
- **Interactive testing** - Manual testing with web UI

## Quick Testing

### Run All Tests

```shell
# Complete test suite (mirrors CI)
./scripts/run-all.sh

# Just integration tests
./scripts/run-tests.sh

# Test specific example
./scripts/run-tests.sh examples/smart-escrows/oracle
```

### Test Individual Examples

```shell
# Test from example directory
cd examples/smart-escrows/hello_world
node run_test.js

# Test with custom network
RIPPLED_HOST=localhost RIPPLED_PORT=6006 node run_test.js
```

## Test Architecture

### Test Types

| Test Type             | Location                 | Purpose                | Tools             |
| --------------------- | ------------------------ | ---------------------- | ----------------- |
| **Unit Tests**        | `src/`                   | Function-level testing | `cargo test`      |
| **Integration Tests** | `examples/*/run_test.js` | WASM module testing    | Node.js + XRPL.js |
| **E2E Tests**         | `e2e-tests/`             | Full system testing    | Node.js + XRPL.js |
| **Interactive Tests** | `ui/`                    | Manual testing         | Web UI            |

### Test Networks

| Network            | Endpoint                                 | Use Case                    |
| ------------------ | ---------------------------------------- | --------------------------- |
| **WASM Devnet**    | `wss://wasm.devnet.rippletest.net:51233` | Default integration testing |
| **Local Rippled**  | `ws://localhost:6006`                    | Development testing         |
| **Host Simulator** | In-memory                                | Unit testing (coming soon)  |

## Integration Testing

### Test Structure

Each example includes a `run_test.js` file with this structure:

```javascript
const xrpl = require("xrpl")
const fs = require("fs")
const assert = require("assert")

// Configuration
const WSS_URL =
  process.env.RIPPLED_WSS || "wss://wasm.devnet.rippletest.net:51233"
const WASM_FILE = "target/wasm32v1-none/release/example.wasm"

async function main() {
  console.log("🧪 Starting integration test...")

  // 1. Setup
  const client = new xrpl.Client(WSS_URL)
  await client.connect()

  // 2. Create test accounts
  const { wallet: sender } = await createAndFundWallet(client)
  const { wallet: receiver } = await createAndFundWallet(client)

  // 3. Load WASM
  const wasmBytes = fs.readFileSync(WASM_FILE)
  const wasmHex = wasmBytes.toString("hex").toUpperCase()

  // 4. Create escrow
  const escrowResult = await createEscrow(client, sender, receiver, wasmHex)

  // 5. Test escrow finish
  const finishResult = await finishEscrow(client, sender, escrowResult.sequence)

  // 6. Verify results
  assert.strictEqual(finishResult.meta.TransactionResult, "tesSUCCESS")

  console.log("✅ Test passed!")
  await client.disconnect()
}
```

### Test Helpers

Common testing utilities are provided:

```javascript
// Account management
async function createAndFundWallet(client) {
  const { wallet } = await client.fundWallet()
  return { wallet }
}

// Escrow operations
async function createEscrow(client, sender, receiver, wasmHex) {
  const tx = {
    TransactionType: "EscrowCreate",
    Account: sender.address,
    Destination: receiver.address,
    Amount: "1000000",
    FinishAfter: Math.floor(Date.now() / 1000) + 10,
    FinishFunction: wasmHex,
  }

  const result = await client.submitAndWait(tx, { wallet: sender })
  return {
    sequence: tx.Sequence || result.result.Sequence,
    hash: result.result.hash,
  }
}

async function finishEscrow(client, sender, sequence) {
  const tx = {
    TransactionType: "EscrowFinish",
    Account: sender.address,
    Owner: sender.address,
    OfferSequence: sequence,
  }

  return await client.submitAndWait(tx, { wallet: sender })
}
```

### Running Integration Tests

```shell
# Run from project root
./scripts/run-tests.sh examples/smart-escrows/oracle

# Run from example directory
cd examples/smart-escrows/oracle
npm test

# Run with debug output
DEBUG=1 node run_test.js

# Run with custom network
RIPPLED_WSS=ws://localhost:6006 node run_test.js
```

## End-to-End Testing

### E2E Test Suite

The `e2e-tests/` directory contains comprehensive system tests:

```shell
# Run all E2E tests
./scripts/run-tests.sh e2e-tests

# Run specific E2E test
cd e2e-tests/trace_escrow_account
node ../../tests/run_single_test.js
```

### E2E Test Categories

| Test                  | Purpose                      | Complexity   |
| --------------------- | ---------------------------- | ------------ |
| `float_tests`         | Floating point operations    | Basic        |
| `host_functions_test` | Host function coverage       | Intermediate |
| `keylet_exists`       | Keylet generation and lookup | Intermediate |
| `trace_escrow_*`      | Various escrow scenarios     | Advanced     |

### Creating E2E Tests

1. **Create test directory**:

   ```shell
   mkdir e2e-tests/my-test
   cd e2e-tests/my-test
   ```

2. **Add Cargo.toml**:

   ```toml
   [package]
   name = "my-test"
   version = "0.1.0"
   edition = "2021"

   [dependencies]
   xrpl-wasm-std = { path = "../../xrpl-wasm-std" }

   [lib]
   crate-type = ["cdylib"]
   ```

3. **Implement test logic** in `src/lib.rs`

4. **Add to test runner** in parent `Cargo.toml`

## Interactive Testing

### Web UI Testing

The web UI provides visual testing capabilities:

1. **Start the UI**:

   ```shell
   # Update embedded WASM first
   ./ui/embed-wasm.sh

   # Open in browser
   open ui/index.html
   ```

2. **Testing workflow**:
   - Connect to network (Devnet or local)
   - Generate and fund test accounts
   - Select or upload WASM code
   - Create escrow with custom parameters
   - Test escrow finish conditions
   - View transaction logs and results

3. **UI Features**:
   - **Network Management** - Switch between networks
   - **Account Management** - Create, fund, and manage test accounts
   - **WASM Management** - Upload files or use pre-built examples
   - **Escrow Management** - Create and track escrows
   - **Transaction Logs** - View detailed transaction results

### Manual Testing Scripts

For custom testing scenarios:

```javascript
// custom-test.js
const xrpl = require("xrpl")

async function customTest() {
  const client = new xrpl.Client("wss://wasm.devnet.rippletest.net:51233")
  await client.connect()

  // Your custom test logic here
  // - Create specific account configurations
  // - Set up complex escrow conditions
  // - Test edge cases
  // - Verify specific behaviors

  await client.disconnect()
}

customTest().catch(console.error)
```

## Testing Best Practices

### Test Design

1. **Test positive and negative cases**:

   ```javascript
   // Test successful escrow finish
   const successResult = await finishEscrow(client, sender, sequence)
   assert.strictEqual(successResult.meta.TransactionResult, "tesSUCCESS")

   // Test escrow remains locked when conditions not met
   const failResult = await finishEscrow(client, wrongSender, sequence)
   assert.strictEqual(failResult.meta.TransactionResult, "tecNO_PERMISSION")
   ```

2. **Test edge cases**:

   ```javascript
   // Test with minimal amounts
   await createEscrow(client, sender, receiver, wasmHex, "1") // 1 drop

   // Test with maximum data sizes
   const largeData = "x".repeat(256) // Max data field size

   // Test timing edge cases
   const immediateFinish = Math.floor(Date.now() / 1000) + 1
   ```

3. **Test error conditions**:

   ```javascript
   // Test invalid WASM
   const invalidWasm = "deadbeef"

   // Test malformed transactions
   const badTx = {
     /* incomplete transaction */
   }
   ```

### Test Data Management

1. **Use deterministic test data**:

   ```rust
   // Use fixed test addresses
   const TEST_NOTARY: [u8; 20] = r_address!("rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh");
   const TEST_ORACLE: [u8; 20] = r_address!("rN7n7otQDd6FczFgLdSqtcsAUxDkw6fzRH");
   ```

2. **Create reusable test fixtures**:

   ```javascript
   // tests/fixtures.js
   const testAccounts = {
     notary: "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh",
     oracle: "rN7n7otQDd6FczFgLdSqtcsAUxDkw6fzRH",
     // ... more test accounts
   }
   ```

3. **Use realistic test scenarios**:
   ```javascript
   // Test with realistic amounts and timing
   const escrowAmount = "1000000" // 1 XRP
   const finishAfter = Math.floor(Date.now() / 1000) + 3600 // 1 hour
   ```

### Debugging Tests

1. **Add comprehensive logging**:

   ```rust
   use xrpl_wasm_std::host::trace::{trace, trace_data, DataRepr};

   #[no_mangle]
   pub extern "C" fn finish() -> i32 {
       trace("Test: Starting execution")?;

       let account = match EscrowFinish.get_account() {
           Ok(acc) => {
               trace_data("Test: Account", &acc, DataRepr::AsHex)?;
               acc
           },
           Err(e) => {
               trace(&format!("Test: Error getting account: {:?}", e))?;
               return 0;
           }
       };

       // More test logic with logging...
   }
   ```

2. **Use debug builds during testing**:

   ```shell
   # Build with debug info
   cargo build --target wasm32v1-none

   # Test with debug build
   DEBUG=1 node run_test.js
   ```

3. **Verify WASM exports**:
   ```shell
   # Check that finish function is exported
   ./scripts/check-wasm-exports.sh target/wasm32v1-none/release/my_test.wasm
   ```

## Continuous Integration

### CI Testing Pipeline

The GitHub Actions workflow runs:

1. **Setup** - Install dependencies
2. **Format Check** - Verify code formatting
3. **Lint** - Run Clippy checks
4. **Build** - Compile all examples
5. **Test** - Run integration tests
6. **Audit** - Check host function compatibility

### Local CI Simulation

```shell
# Run the complete CI pipeline locally
./scripts/run-all.sh

# Individual CI steps
./scripts/setup.sh      # Setup environment
./scripts/fmt.sh        # Check formatting
./scripts/clippy.sh     # Run linting
./scripts/build.sh      # Build all examples
./scripts/run-tests.sh  # Run tests
```

### CI Configuration

Environment variables for CI:

| Variable      | Value        | Purpose                  |
| ------------- | ------------ | ------------------------ |
| `CI`          | `true`       | Enable CI mode           |
| `RUSTFLAGS`   | `-Dwarnings` | Treat warnings as errors |
| `RIPPLED_WSS` | Devnet URL   | Test network endpoint    |

## Performance Testing

### Binary Size Testing

```shell
# Check WASM binary sizes
ls -la examples/smart-escrows/*/target/wasm32v1-none/release/*.wasm

# Optimize for minimum size
cargo build --target wasm32v1-none --release
wasm-opt -Oz input.wasm -o output.wasm
```

### Execution Performance

```rust
// Test execution time (in trace output)
use xrpl_wasm_std::host::host_bindings::get_parent_ledger_time;

#[no_mangle]
pub extern "C" fn finish() -> i32 {
    let start_time = unsafe { get_parent_ledger_time() };

    // Your logic here

    let end_time = unsafe { get_parent_ledger_time() };
    trace(&format!("Execution time: {} seconds", end_time - start_time))?;

    1
}
```

### Resource Usage Testing

```rust
// Test cache slot usage
let mut slots_used = 0;
for i in 0..10 {
    match cache_ledger_obj(&keylets[i]) {
        Ok(_) => slots_used += 1,
        Err(Error::NoFreeSlots) => {
            trace(&format!("Cache full after {} slots", slots_used))?;
            break;
        }
        Err(e) => return 0,
    }
}
```

## Troubleshooting Tests

### Common Test Failures

| Issue                  | Symptoms              | Solution                    |
| ---------------------- | --------------------- | --------------------------- |
| **Network connection** | Connection timeout    | Check network and firewall  |
| **Insufficient funds** | `tecUNFUNDED_PAYMENT` | Ensure accounts are funded  |
| **WASM too large**     | Upload failure        | Optimize binary size        |
| **Invalid WASM**       | Runtime error         | Check exports and format    |
| **Timing issues**      | `tecEXPIRED`          | Adjust `FinishAfter` timing |

### Debug Tools

```shell
# Check WASM binary
wasm-objdump -x my_contract.wasm

# Validate WASM
wasm-validate my_contract.wasm

# Check binary size
ls -la my_contract.wasm

# Test network connectivity
curl -s https://wasm.devnet.rippletest.net:51234

# Check Node.js environment
node --version
npm list xrpl
```

### Test Environment Issues

1. **Clean test environment**:

   ```shell
   # Clean build artifacts
   cargo clean

   # Reinstall Node.js dependencies
   rm -rf node_modules package-lock.json
   npm install
   ```

2. **Reset test state**:

   ```shell
   # Use fresh test accounts for each test run
   # Avoid reusing accounts that might have stale state
   ```

3. **Network-specific issues**:

   ```shell
   # Try different test network
   RIPPLED_WSS=wss://s.altnet.rippletest.net:51233 node run_test.js

   # Check network status
   curl -s https://testnet.xrpl.org | grep -i status
   ```

## Advanced Testing

### Property-Based Testing

```javascript
// Generate random test inputs
function generateRandomAccount() {
  return xrpl.Wallet.generate().address
}

function generateRandomAmount() {
  return Math.floor(Math.random() * 1000000) + 1
}

// Test with many random inputs
for (let i = 0; i < 100; i++) {
  const amount = generateRandomAmount()
  const result = await testWithAmount(amount)
  // Verify invariants hold for all inputs
}
```

### Fuzzing

```rust
// Add fuzzing targets for critical functions
#[cfg(fuzzing)]
pub fn fuzz_target(data: &[u8]) {
    if data.len() >= 20 {
        let account = AccountID::from_slice(&data[0..20]);
        let _ = validate_account(&account);
    }
}
```

### Stress Testing

```javascript
// Test with high transaction volume
async function stressTest() {
  const promises = []

  for (let i = 0; i < 100; i++) {
    promises.push(createAndTestEscrow())
  }

  const results = await Promise.all(promises)

  // Verify all succeeded
  results.forEach((result) => {
    assert.strictEqual(result.meta.TransactionResult, "tesSUCCESS")
  })
}
```

## Next Steps

- **[Building Guide](building.md)** - Development environment setup
- **[API Reference](../api-reference.md)** - Complete API documentation
- **[Examples](../examples/README.md)** - Study testing patterns in examples
- **[Contributing](contributing.md)** - How to contribute tests
