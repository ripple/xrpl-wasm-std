# trace_escrow_ledger_object e2e-test

This WebAssembly module traces every field of an Escrow ledger object to validate functionality between this library and
xrpld.

### 1. Install dependencies

```shell
npm install
```

### 2. Build the WASM

```shell
cargo build --target wasm32v1-none --release
```

### 3a. Deploy and test Locally

```shell
cd ../..
./scripts/run-tests.sh e2e-tests/trace_escrow_ledger_object
```

### 3b. Deploy and test on Devnet

```shell
cd ../..
CI=1 ./scripts/run-tests.sh e2e-tests/trace_escrow_ledger_object
```

This will:

- Connect to WASM Devnet
- Create and fund two wallets (Origin and Destination)
- Create an EscrowCreate transaction with your compiled `FinishFunction`
- Finish the escrow, executing the `helloworld` WASM

Expected result: `tesSUCCESS` and “Escrow finished successfully!”.
