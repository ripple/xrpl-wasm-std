# Notary Escrow FinishFunction

This WebAssembly module implements a notary-based escrow finish condition. It verifies that only a designated notary
account is allowed to finish the escrow.

### How it works

The contract checks whether the account submitting EscrowFinish matches the embedded notary account. If it matches, it
returns 1 (allow), otherwise 0 (deny).

### Function

`finish() -> i32` — returns 1 to allow finishing the escrow, 0 to reject (deny finishing). On host errors, the function
returns a non-zero error code from the host.

## Prerequisites

- Rust toolchain with `wasm32v1-none` target
- Node.js 18+
- Dependencies installed in `reference/js`:

```shell
cd reference/js
npm install
```

## Step-by-step: Use on WASM Devnet

This guide uses the public Devnet WASM endpoint at `wss://wasm.devnet.rippletest.net:51233` and the helper scripts in
`reference/js`.

### 1. Create a notary account (funded via faucet)

Use the faucet helper script. It prints export lines you can copy/paste.

```shell
cd reference/js
node faucet.js
# Copy the printed export lines into your shell:
# export NOTARY_ADDRESS=...
# export NOTARY_SEED=...
```

Export them for convenience (replace with your printed values):

```shell
export NOTARY_ADDRESS=rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh
```

### 2. Build the notary WASM

The notary address is hardcoded in the source code. To change it, edit `src/lib.rs` and modify the `NOTARY_ACCOUNT` constant.

```shell
cargo build
cargo build --target wasm32v1-none
cargo build --target wasm32v1-none --release
```

Artifact:

```
examples/smart-escrows/notary/target/wasm32v1-none/release/notary.wasm
```

### 3. Deploy an escrow using your FinishFunction on Devnet

Use the helper to deploy an escrow that references your compiled `FinishFunction`.

```shell
cd ../../reference/js
node deploy_sample.js notary
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
node finish_escrow.js $NOTARY_ADDRESS $NOTARY_SEED $OWNER_ADDRESS $OFFER_SEQUENCE
```

Expected result: `tesSUCCESS` and “Escrow finished successfully!”. If you try to finish from a different account, you
should get `tecNO_PERMISSION` due to the notary check.

## Local testing with wasm-host-simulator (optional)

You can also run the WASM locally with the included host emulator:

```shell
cd ../../../../
cargo run --package wasm-host-simulator --bin wasm-host-simulator -- --dir examples/smart-escrows/notary --project notary
```

## Modifying the notary account

The notary account is defined as a constant in `src/lib.rs` using the `r_address!` macro:

```rust
const NOTARY_ACCOUNT: [u8; 20] = r_address!("rN7n7otQDd6FczFgLdSqtcsAUxDkw6fzRH");
```

To use a different notary account, simply edit this line with your desired r-address. The macro validates the address at compile time and converts it to the 20-byte AccountID.

## Notes

- The contract compares raw 20-byte AccountIDs. Classic addresses are converted at compile-time by the `r_address!` macro.
- Make sure the hardcoded notary address in `src/lib.rs` matches the account you'll use in step 4 to submit `EscrowFinish`.
