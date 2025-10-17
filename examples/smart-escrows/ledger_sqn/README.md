# Ledger Sequence Escrow

This WebAssembly module implements a simple ledger sequence-based escrow finish condition. It checks whether the current ledger sequence number is greater than 5 before allowing the escrow to be finished.

## How it works

The contract retrieves the current ledger sequence number and checks if it's greater than 5. Since ledger sequences start from 1 and increment with each ledger, this condition is almost always met in practice, making this a basic demonstration example.

## Function

`finish() -> i32` — returns 1 if ledger sequence > 5 (allow), 0 otherwise (deny). On host errors, the function returns a non-zero error code from the host.

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
examples/smart-escrows/ledger_sqn/target/wasm32v1-none/release/ledger_sqn.wasm
```

### 3. Deploy WASM to Devnet & Execute

Use the helper to deploy an escrow that references your compiled `FinishFunction` (Note: the following example attempts to run the script from this project's root folder).

```shell
cd ../../..
CI=1 ./scripts/run-tests.sh examples/smart-escrows/ledger_sqn
```

This will:

- Connect to WASM Devnet
- Create and fund two wallets (Origin and Destination)
- Create an EscrowCreate transaction with your compiled `FinishFunction`
- Print the transaction result, including `tx_json.Sequence`
- Finish the escrow, executing the `ledger_sqn` WASM

Expected result: `tesSUCCESS` and "Escrow finished successfully!" (since ledger sequence will be > 5).

## Notes

- This is a basic demonstration example - the condition (ledger sequence > 5) is almost always true
- In practice, you might use higher sequence numbers or time-based conditions for more realistic scenarios
- The contract demonstrates how to access ledger state information from within WASM
- Useful as a starting point for building more complex time or block-based escrow conditions
