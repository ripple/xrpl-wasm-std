# XRPL Std Lib Example

This WebAssembly module demonstrates how to use the XRPL std lib to access every field in an `Escrow` ledger object that
is being finished by a Smart Escrow contract. The WASM program is meant to both execute known host functions required
for accessing the fields of an `Escrow` ledger object, and also validate that field access is working correctly by
asserting each value that craft makes available. In this way, this Smart Escrow can be used as a type of canary that can
indicate if anything in Craft is not operating according to expectations.

## Building

Build using:

```bash
cargo build
cargo build --target wasm32v1-none
cargo build --target wasm32v1-none --release
```

The resulting WASM file will be located at:

```
./target/wasm32v1-none/release/trace_escrow_ledger_object.wasm
```

## Running with wasm-host-simulator

Run the contract using the wasm-host-simulator application:

```bash
cd ../../../
cargo run --package wasm-host-simulator --bin wasm-host-simulator -- --dir projects/e2e-tests/trace_escrow_ledger_object --project trace_escrow_ledger_object
```

### Note

Please note that the wasm-host-simulator only has mock host functions. Please use the devnet (or a standalone rippled node) to
test with a real implementation and real data.
