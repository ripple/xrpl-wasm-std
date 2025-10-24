# Atomic Swap 2: Data Field-Based Smart Escrow

A smart escrow for atomic swaps with built-in timing validation and two-phase execution.

## What is it?

This escrow enables atomic swaps using a two-phase execution model with automatic timing control. The escrow stores state in its data field and executes in two phases:

1. **Phase 1**: Validates the referenced escrow exists and sets a timing deadline
2. **Phase 2**: Checks if we're still within the deadline and completes the swap

This approach provides built-in timing coordination and prevents stale swap attempts.

## When to use it?

- **Timed exchanges** that must complete within a deadline
- **Coordinated settlements** requiring automatic timing validation
- **Sequential workflows** where you need state persistence between executions
- **Deadline-sensitive trades** that should expire after a certain time

## How it works

**Setup**: Create an escrow with the first escrow's keylet in the data field.

**Phase 1**: First `EscrowFinish` validates the referenced escrow exists, appends timing data, and returns "wait".

**Phase 2**: Second `EscrowFinish` checks if we're within the deadline and completes if valid.

## Step-by-Step Transaction Guide

This guide shows how to manually create and execute a data field-based atomic swap using the WASM Devnet or direct transaction submission.

### Step 1: Create First Escrow (Regular Escrow)

**EscrowCreate Transaction:**

```json
{
  "TransactionType": "EscrowCreate",
  "Account": "rAlice...",
  "Destination": "rBob...",
  "Amount": "1000000",
  "CancelAfter": 2000000000,
  "FinishFunction": "REGULAR_WASM_HEX_OR_EMPTY"
}
```

**Expected Result:**

- Transaction succeeds with `tesSUCCESS`
- Note the `Sequence` number (e.g., `123`)
- Extract escrow keylet from transaction metadata `AffectedNodes[].CreatedNode.LedgerIndex`

### Step 2: Create Second Escrow (Atomic Swap 2 with Data Field)

**EscrowCreate Transaction:**

```json
{
  "TransactionType": "EscrowCreate",
  "Account": "rBob...",
  "Destination": "rAlice...",
  "Amount": "2000000",
  "CancelAfter": 2000000000,
  "FinishFunction": "ATOMIC_SWAP2_WASM_HEX_HERE",
  "Data": "FIRST_ESCROW_KEYLET_32_BYTES_HEX"
}
```

**Key Details:**

- `Data`: Use the keylet from Step 1's transaction metadata (32 bytes hex)
- `CancelAfter`: Must be set - this becomes the swap deadline
- `FinishFunction`: Use the compiled atomic_swap2.wasm

**Expected Result:**

- Transaction succeeds with `tesSUCCESS`
- Note the `Sequence` number (e.g., `456`)

### Step 3: Execute First Phase - Initialize Timing

**EscrowFinish Transaction (Phase 1):**

```json
{
  "TransactionType": "EscrowFinish",
  "Account": "rBob...",
  "Owner": "rBob...",
  "OfferSequence": 456,
  "ComputationAllowance": 1000000
}
```

**Expected Result:**

- Transaction succeeds with `tesSUCCESS` but escrow does NOT complete yet
- Trace shows "Phase 1: Initialized timing data"
- Escrow data field is updated with CancelAfter timestamp
- Contract returns `0` to indicate "wait for phase 2"

### Step 4: Execute Second Phase - Complete Swap

**EscrowFinish Transaction (Phase 2):**

```json
{
  "TransactionType": "EscrowFinish",
  "Account": "rBob...",
  "Owner": "rBob...",
  "OfferSequence": 456,
  "ComputationAllowance": 1000000
}
```

**Expected Result (if within deadline):**

- Transaction succeeds with `tesSUCCESS`
- Escrow 2 is consumed and Alice receives Bob's funds
- Trace shows "Phase 2: Completed within deadline"

**Expected Result (if deadline exceeded):**

- Transaction succeeds with `tesSUCCESS` but escrow fails to complete
- Trace shows "Phase 2: Deadline exceeded"
- Contract returns `0` indicating timing failure

### Two-Phase Execution Example

**Data Field Evolution:**

_Initial State (at creation):_

```
Data: [First Escrow Keylet - 32 bytes]
Length: 32 bytes
```

_After Phase 1:_

```
Data: [First Escrow Keylet - 32 bytes][CancelAfter - 4 bytes]
Length: 36 bytes
```

_Phase 2 reads the last 4 bytes as deadline timestamp_

## Transaction Structure Reference

### EscrowCreate (Setup)

```json
{
  "TransactionType": "EscrowCreate",
  "Account": "[SENDER_ADDRESS]",
  "Destination": "[RECEIVER_ADDRESS]",
  "Amount": "[AMOUNT_IN_DROPS]",
  "CancelAfter": "[UNIX_TIMESTAMP]",
  "FinishFunction": "[ATOMIC_SWAP2_WASM_HEX]",
  "Data": "[FIRST_ESCROW_KEYLET_32_BYTES_HEX]"
}
```

### EscrowFinish (Execution)

```json
{
  "TransactionType": "EscrowFinish",
  "Account": "[FINISHER_ADDRESS]",
  "Owner": "[ESCROW_OWNER_ADDRESS]",
  "OfferSequence": "[ESCROW_SEQUENCE_NUMBER]",
  "ComputationAllowance": 1000000
}
```

## Building

### Prerequisites

- Rust with `wasm32v1-none` target
- XRPL WASM standard library

### Build Commands

```shell
cargo build --target wasm32v1-none
cargo build --target wasm32v1-none --release
```

The resulting WASM file will be located at:

```
./target/wasm32v1-none/release/atomic_swap2.wasm
```

## Testing

```shell
cd ../../../
CI=1 ./scripts/run-tests.sh examples/smart-escrows/atomic_swap2
```

## Important Notes

⚠️ **Two Executions Required**: You must call `EscrowFinish` twice - once for each phase.

⚠️ **Timing**: The deadline is set from the escrow's `CancelAfter` field during Phase 1.

⚠️ **Data Field**: Must contain exactly 32 bytes (the first escrow's keylet) when creating the escrow.

## What can go wrong?

| Scenario                    | Phase | Result       |
| --------------------------- | ----- | ------------ |
| Wrong data field length     | 1     | Escrow fails |
| Referenced escrow not found | 1     | Escrow fails |
| Missing CancelAfter field   | 1     | Escrow fails |
| Deadline exceeded           | 2     | Escrow fails |

## Complete Atomic Swap

This escrow is designed to work **with other escrows** to create complete atomic swaps. For example:

- **Escrow A** (Alice→Bob): Uses any contract (including atomic_swap1)
- **Escrow B** (Bob→Alice): Uses this data field-based contract, references Escrow A's keylet

The atomic_swap1 example shows a different validation approach using memos, but both examples demonstrate the same principle: **one escrow validates the other exists before completing**.
