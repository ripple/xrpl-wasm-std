# Trace Escrow Account

This WebAssembly module traces all fields of the AccountRoot ledger object that holds a Smart Escrow.

### How it works

The contract traces all fields of the account root, then returns 1 (allow) if all traces were successful.

### Function

`finish() -> i32` — returns 1 to allow finishing the escrow, 0 to reject (deny finishing). On host errors, the function
returns a non-zero error code from the host.

## Step-by-step: Use on WASM Devnet

This guide uses the public Devnet WASM endpoint at `wss://wasm.devnet.rippletest.net:51233`.

### 1. Install dependencies

```shell
npm install
```

### 2. Build the notary WASM

```shell
cargo build
cargo build --target wasm32v1-none
cargo build --target wasm32v1-none --release
```

This will produce the following artifact:

```
./e2e-tests/target/wasm32v1-none/release/trace_escrow_account.wasm
```

### 3. Deploy WASM to Devnet

Use the helper to deploy an escrow that references your compiled `FinishFunction` (Note: the following example attempts
to run the script from this project's root folder).

```shell
cd ../..
CI=true ./scripts/run-tests.sh e2e-tests/trace_escrow_account
```

This will:

- Connect to WASM Devnet
- Create and fund two wallets (Origin and Destination)
- Create an EscrowCreate transaction with your compiled `FinishFunction` (trace_escrow_account.wasm)
- Print the transaction result (expected result: `tesSUCCESS` and “Escrow finished successfully!”)
  No newline at end of file