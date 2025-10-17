# Hello World Smart Escrow

A beginner-friendly smart escrow for the XRPL that demonstrates basic contract structure and logging.

## Overview

This smart escrow serves as the perfect starting point for XRPL WebAssembly development. It implements the simplest possible escrow logic: emit "Hello World" to the trace log and always release the escrow.

- Always releases the escrow (returns 1)
- Demonstrates basic contract structure
- Shows how to use trace logging
- Minimal code for learning purposes

The Rust code demonstrates the fundamental structure of an XRPL smart escrow and basic debugging techniques.

## Functionality

### Core Components

- **Entry Point**: Standard `finish()` function required by all smart escrows
- **Trace Logging**: Demonstrates how to output debug information
- **Unconditional Release**: Always returns 1 to release the escrow

### Key Functions

- `finish()`: Main entry point that logs "Hello World" and returns 1

## Code Structure

```rust
#![no_std]
#![no_main]

use xrpl_wasm_std::host::trace::trace;

#[no_mangle]
pub extern "C" fn finish() -> i32 {
    // Log a message for debugging
    let _ = trace("Hello World from smart escrow!");

    // Always release the escrow
    1
}
```

## Building

### Prerequisites

- Rust with `wasm32v1-none` target
  - This is necessary for blockchain deployments because WebAssembly does not require a specific vendor (e.g., `apple`) or operating system (e.g., `darwin`), so both are `unknown`
- XRPL standard library (dependency)
- Node.js 18+ (for testing)

### Build Commands

```bash
# Debug build
cargo build --target wasm32v1-none

# Release build (optimized)
cargo build --target wasm32v1-none --release
```

The compiled WASM will be at:

```
target/wasm32v1-none/release/hello_world.wasm
```

## Testing

### Integration Test

Run the comprehensive integration test:

```bash
# From the repository root
./scripts/run-tests.sh examples/smart-escrows/hello_world

# Or with explicit CI mode
CI=1 ./scripts/run-tests.sh examples/smart-escrows/hello_world
```

### What the Test Does

The integration test (`run_test.js`) performs a complete escrow lifecycle:

1. **Connect** to WASM Devnet (`wss://wasm.devnet.rippletest.net:51233`)
2. **Create and fund** two test accounts (sender and receiver)
3. **Deploy escrow** with the compiled WASM as `FinishFunction`
4. **Execute finish** transaction to test the smart escrow
5. **Verify results** - expects `tesSUCCESS` and trace output

### Expected Results

- **Transaction Result**: `tesSUCCESS`
- **Trace Output**: "Hello World from smart escrow!"
- **Escrow Status**: Successfully finished and funds released

### Manual Testing

You can also test using the [Testing UI](../../../ui/):

1. Open `ui/index.html` in your browser
2. Connect to WASM Devnet
3. Generate test accounts
4. Load the hello world WASM (pre-embedded)
5. Create and test an escrow

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        XRP Ledger                               │
│                                                                 │
│  ┌─────────────────┐              ┌─────────────────────────┐   │
│  │   EscrowFinish  │              │    Smart Escrow         │   │
│  │   Transaction   │              │    (WASM Module)        │   │
│  │                 │              │                         │   │
│  │ ┌─────────────┐ │              │ finish() function:      │   │
│  │ │ FinishFunc  │ │─────────────►│                         │   │
│  │ │ (WASM)      │ │              │ 1. Log "Hello World"    │   │
│  │ └─────────────┘ │              │                         │   │
│  └─────────────────┘              │ 2. Return 1 (release)   │   │
│                                   │                         │   │
│                                   └─────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

## Learning Objectives

This example teaches:

1. **Basic Structure** - How to set up a smart escrow project
2. **Entry Point** - The required `finish()` function signature
3. **Compilation** - Building Rust to WebAssembly for XRPL
4. **Logging** - Using trace functions for debugging
5. **Testing** - Integration testing with real XRPL networks
6. **Deployment** - How escrows reference WASM code

## Use Cases

While this example always releases escrows, it demonstrates patterns useful for:

- **Debugging** - Understanding trace output and execution flow
- **Template** - Starting point for more complex escrow logic
- **Learning** - Understanding the smart escrow development workflow
- **Testing** - Verifying build and deployment infrastructure

## Next Steps

After mastering this example, explore:

1. **[KYC Example](../kyc/)** - Credential-based authorization
2. **[Oracle Example](../oracle/)** - Price-based conditional logic
3. **[Notary Example](../notary/)** - Multi-signature authorization
4. **[API Reference](../../../docs/api-reference.md)** - Complete API documentation

## Security Considerations

Even simple examples should follow security best practices:

- **Always handle errors** gracefully (this example uses `let _ =` for trace)
- **Return safe defaults** - 0 keeps escrow locked, 1 releases it
- **Test thoroughly** - Verify behavior on test networks first
- **Keep it simple** - Complex logic increases bug risk

## Troubleshooting

### Build Issues

- **Target missing**: `rustup target add wasm32v1-none`
- **Crate type error**: Check `Cargo.toml` has `crate-type = ["cdylib"]`
- **Large binary**: Use `--release` flag for size optimization

### Runtime Issues

- **Function not found**: Ensure `#[no_mangle]` on `finish()` function
- **No trace output**: Check that testing environment supports trace logs
- **Test failures**: Verify network connectivity to WASM Devnet

### Common Mistakes

- Forgetting `#![no_std]` and `#![no_main]` attributes
- Using `unwrap()` instead of proper error handling
- Not exporting the `finish()` function correctly

## References

- **[Getting Started Guide](../../../docs/getting-started.md)** - Complete setup instructions
- **[Smart Escrows XLS](https://github.com/XRPLF/XRPL-Standards/discussions/270)** - Specification
- **[XRPL Documentation](https://xrpl.org/docs.html)** - General XRPL concepts
