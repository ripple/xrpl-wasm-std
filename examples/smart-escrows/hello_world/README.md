# Hello World Escrow

This WebAssembly module implements a simple escrow finish function that emits the text "Hello World" into the trace log
and always returns 1. This example is useful for getting started with a new Smart Escrow.

## Prerequisites

- Rust toolchain with `wasm32v1-none` target
- Node.js 18+

```shell
npm install
```

## Step-by-step: Use on WASM Devnet

This guide uses the public Devnet WASM endpoint at `wss://wasm.devnet.rippletest.net:51233`.

### 2. Build the WASM

```shell
cargo build --target wasm32v1-none --release
```

Artifact:

```
examples/smart-escrows/hello_world/target/wasm32v1-none/release/helloworld.wasm
```

### 3. Deploy WASM to Devnet & Execute

Use the helper to deploy an escrow that references your compiled `FinishFunction` (Note: the following example attempts
to run the script from this project's root folder).

```shell
cd ../../..
CI=1 ./scripts/run-tests.sh examples/smart-escrows/hello_world
```

This will:

- Connect to WASM Devnet
- Create and fund two wallets (Origin and Destination)
- Create an EscrowCreate transaction with your compiled `FinishFunction`
- Print the transaction result, including `tx_json.Sequence`
- Finish the escrow, executing the `helloworld` WASM.

Expected result: `tesSUCCESS` and “Escrow finished successfully!”.
