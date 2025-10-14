# XRPL Std Lib Example

This WebAssembly module is for testing the decoder in the simulated host.

## Building

Build using:

```bash
cargo build --target wasm32v1-none
cargo build --target wasm32v1-none --release
```

The resulting WASM file will be located at:

```
./target/wasm32v1-none/release/decoder_tests.wasm
```

## Running with wasm-host-simulator

Run the contract using the wasm-host-simulator application:

```bash
cd ../../../
cargo run --package wasm-host-simulator --bin wasm-host-simulator -- --dir projects/e2e-tests/decoder_tests --project decoder_tests
```

### Note

Please note that the wasm-host-simulator only has mock host functions. Please use the devnet (or a standalone rippled node) to
test with real data.
