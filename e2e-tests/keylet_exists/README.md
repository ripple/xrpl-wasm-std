# XRPL Std Lib Example

This WebAssembly module is an example using the XRPL std lib to determine if an object identified by a keylet exists.

## Building

Build using:

```bash
cargo build --target wasm32v1-none
cargo build --target wasm32v1-none --release
```

The resulting WASM file will be located at:

```
./target/wasm32v1-none/release/keylet_example.wasm
```

## Running with wasm-host-simulator

Run the contract using the wasm-host-simulator application:

```bash
cd ../../../
cargo run -p wasm-host-simulator -- --dir projects/e2e-tests/keylet_exists --project keylet_exists
```

### Note

Please note that the wasm-host-simulator only has mock host functions. Please use the devnet (or a standalone rippled node) to
test with real data.
