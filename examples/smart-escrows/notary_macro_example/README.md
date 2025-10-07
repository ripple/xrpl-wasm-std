# NFT Owner Smart Escrow

Smart Escrow example demonstrating compile-time XRPL r-address to hex conversion using procedural macros.

## Overview

This smart escrow unlocks when the account finishing the escrow is the same as the pre-programmaed notary account.
Otherwise, the escrow does not unlock.

## Building

### Build Commands

```bash
cargo build --target wasm32v1-none
cargo build --target wasm32v1-none --release
```

The resulting WASM file will be located at:

```
./target/wasm32v1-none/release/notary_macro_example.wasm
```

## Running with wasm-host-simulator

Run the contract using the wasm-host-simulator application:

```bash
cd ../../../../wasm-host-simulator
cargo run -- --dir ../examples/smart-escrows/notary_macro_example --project notary_macro_example
```
