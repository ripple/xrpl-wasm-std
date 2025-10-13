# Hello World Escrow

This WebAssembly module implements a simple escrow finish function that emits the text "Hello World" into the trace log
and always returns 1. This example is useful for getting started with a new Smart Escrow.

## Prerequisites

- Rust toolchain with `wasm32v1-none` target
- Node.js 18+
- Dependencies installed in `reference/js`:

```shell
npm install
```

## Step-by-step: Use on WASM Devnet

This guide uses the public Devnet WASM endpoint at `wss://wasm.devnet.rippletest.net:51233` and the helper scripts in
`reference/js`.

### 2. Build the WASM

```shell
cargo build --target wasm32v1-none --release
```

Artifact:

```
examples/smart-escrows/helloworld/target/wasm32v1-none/release/helloworld.wasm
```

### 3. Deploy WASM to Devnet

Use the helper to deploy an escrow that references your compiled `FinishFunction`.

```shell
cd ../../../tests
node run_single_test.js "../examples/smart-escrows/helloworld/" "../examples/target/wasm32v1-none/release/helloworld.wasm" "wss://wasm.devnet.rippletest.net:51233"
```

This will:

- Connect to WASM Devnet
- Create and fund two wallets (Origin and Destination)
- Create an EscrowCreate transaction with your compiled `FinishFunction`
- Print the transaction result, including `tx_json.Sequence`

Record the following from the output:

- Origin (Owner) address: printed as “Account 1 - Address: ...”
- OfferSequence: from the EscrowCreate `tx_json.Sequence`

For convenience:

```shell
export OWNER_ADDRESS=<Account 1 Address printed by deploy script>
export OFFER_SEQUENCE=<Sequence printed in tx_json>
```

### 4. Finish the escrow as the notary

Submit `EscrowFinish` from the notary account you created in step 1:

```shell
node finish_escrow.js $OWNER_ADDRESS $OFFER_SEQUENCE
```

Expected result: `tesSUCCESS` and “Escrow finished successfully!”. If you try to finish from a different account, you
should get `tecNO_PERMISSION` due to the notary check.
