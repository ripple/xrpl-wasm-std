# xrpl-wasm-std Library

The XRPL Standard Library provides safe, type-safe access to XRPL host functions for WebAssembly smart contract development. This `no_std` library offers zero-cost abstractions over raw host function calls and handles memory management, error handling, and type conversions.

## Quick Start

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

Create a simple escrow contract:

```rust
#![no_std]
#![no_main]

use xrpl_wasm_std::core::current_tx::escrow_finish::EscrowFinish;
use xrpl_wasm_std::core::ledger_objects::account::get_account_balance;

#[no_mangle]
pub extern "C" fn finish() -> i32 {
    let tx = EscrowFinish;
    let account = match tx.get_account() {
        Ok(acc) => acc,
        Err(_) => return 0,
    };

    // Release escrow if balance > 10 XRP
    match get_account_balance(&account) {
        Ok(balance) if balance > 10_000_000 => 1, // Release
        _ => 0, // Keep locked
    }
}
```

Build and test:

```shell
cargo build --target wasm32v1-none --release
./scripts/run-tests.sh examples/your-project
```

## Documentation

| Section                                                  | Description                                   |
| -------------------------------------------------------- | --------------------------------------------- |
| **[Getting Started](docs/getting-started.md)**           | Installation, first contract, core concepts   |
| **[API Reference](docs/api-reference.md)**               | Complete API documentation and usage patterns |
| **[Examples](docs/examples/README.md)**                  | Smart escrow examples and tutorials           |
| **[Development](docs/development/building.md)**          | Building, testing, and CI setup               |
| **[Rust API Docs](target/doc/xrpl_wasm_std/index.html)** | Generated API documentation (`cargo doc`)     |

## Key Features

- **Type-safe access** to transaction and ledger data
- **Memory-safe operations** with no heap allocations
- **Deterministic execution** across all nodes/validators
- **Zero-cost abstractions** over host functions
- **Comprehensive error handling** with custom Result types

## Examples Overview

- **[hello_world](examples/smart-escrows/hello_world/)** - Basic escrow with logging
- **[oracle](examples/smart-escrows/oracle/)** - Price-based release using oracle data
- **[kyc](examples/smart-escrows/kyc/)** - Credential-based verification
- **[notary](examples/smart-escrows/notary/)** - Multi-signature authorization
- **[nft_owner](examples/smart-escrows/nft_owner/)** - NFT ownership verification
- **[ledger_sqn](examples/smart-escrows/ledger_sqn/)** - Sequence-based release

## Testing Tools

- **[Testing UI](ui/)** - Web interface for contract testing
- **[E2E Tests](e2e-tests/)** - Integration test suite
- **[Scripts](scripts/)** - Local CI pipeline

## Safety and Constraints

Smart escrows run in a constrained WebAssembly environment:

- **Read-only ledger access** (except escrow data updates)
- **Deterministic execution** required
- **Resource limits** enforced
- **No network/file system** access

## Contributing

See [Development Guide](docs/development/building.md) for build setup and [Testing Guide](docs/development/testing.md) for running tests.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
