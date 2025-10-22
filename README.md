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
panic = "abort"
```

Create a simple escrow contract:

```rust
use xrpl_wasm_std::core::current_tx::escrow_finish::EscrowFinish;
use xrpl_wasm_std::core::current_tx::traits::TransactionCommonFields;
use xrpl_wasm_std::core::ledger_objects::account_root::get_account_balance;
use xrpl_wasm_std::core::types::amount::token_amount::TokenAmount;
use xrpl_wasm_std::host::Result::{Ok, Err};

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let tx = EscrowFinish;
    let account = match tx.get_account() {
        Ok(acc) => acc,
        Err(_) => return 0,
    };

    // Release escrow if balance > 10 XRP
    match get_account_balance(&account) {
        Ok(Some(TokenAmount::XRP { num_drops })) if num_drops > 10_000_000 => 1, // Release
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

| Section                                                                     | Description                                     |
| --------------------------------------------------------------------------- | ----------------------------------------------- |
| **[Complete Developer Guide](./target/doc/xrpl_wasm_std/guide/index.html)** | Comprehensive guide with working internal links |
| **[Rust API Docs](https://ripple.github.io/xrpl-wasm-std)**                 | Generated API documentation (`cargo doc`)       |

The complete developer guide includes:

- Getting Started - Installation, first contract, core concepts
- API Reference - Complete API documentation and usage patterns
- Examples - Smart escrow examples and tutorials
- Development Guide - Building, testing, and CI setup

## Key Features

- **Type-safe access** to transaction and ledger data
- **Memory-safe operations** with no heap allocations
- **Deterministic execution** across all nodes/validators
- **Zero-cost abstractions** over host functions
- **Comprehensive error handling** with custom Result types

## Examples Overview

- **[hello_world](https://github.com/ripple/xrpl-wasm-std/tree/main/examples/smart-escrows/hello_world/)** - Basic escrow with logging
- **[oracle](https://github.com/ripple/xrpl-wasm-std/tree/main/examples/smart-escrows/oracle/)** - Price-based release using oracle data
- **[kyc](https://github.com/ripple/xrpl-wasm-std/tree/main/examples/smart-escrows/kyc/)** - Credential-based verification
- **[notary](https://github.com/ripple/xrpl-wasm-std/tree/main/examples/smart-escrows/notary/)** - Multi-signature authorization
- **[nft_owner](https://github.com/ripple/xrpl-wasm-std/tree/main/examples/smart-escrows/nft_owner/)** - NFT ownership verification
- **[ledger_sqn](https://github.com/ripple/xrpl-wasm-std/tree/main/examples/smart-escrows/ledger_sqn/)** - Sequence-based release

## Testing UI

There is an interface available at https://ripple.github.io/xrpl-wasm-std/ui/ for local or Devnet testing.

## Safety and Constraints

Smart escrows run in a constrained WebAssembly environment:

- **Read-only ledger access** (except escrow data updates)
- **Deterministic execution** required
- **Resource limits** enforced
- **No network/file system** access

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for detailed guidelines on:

- Development setup and workflow
- Code standards and style guidelines
- Pull request process
- Testing requirements
- Release procedures

We welcome contributions of all kinds!
