// Shared JS test harness for smart-escrow runTest.js files.
//
// This is a mechanical dedup layer over runSingleTest.js + deployWasmCode.js.
// Each helper is small (~10-15 lines) and shaped so it will translate 1:1 to
// a future Rust-native harness — no client-agnostic abstractions, no
// framework-y magic.
//
// Injected into every runTest.js via `testContext` in `runSingleTest.js`, so
// tests write:
//
//     const { finishEscrow, expectResult, expectEscrowConsumed } = testContext
//
// rather than repeating the same 8-line EscrowFinish + assert pattern.

const fs = require("fs")
const path = require("path")

/**
 * Submit an `EscrowFinish` and assert on its result code.
 *
 * @param {object}      ctx            Test context (needs `submit`).
 * @param {xrpl.Wallet} wallet         Signer.
 * @param {object}      opts
 * @param {string}      opts.owner     `EscrowCreate` account (the escrow owner).
 * @param {number}      opts.offerSequence  `Sequence` of the create tx.
 * @param {number}     [opts.gas=1_000_000] WASM gas budget for the finish.
 * @param {string|string[]} [opts.expect="tesSUCCESS"] Expected result code(s).
 * @param {any[]}      [opts.memos]    Optional Memos array.
 * @returns {Promise<object>}          The full submit response.
 */
async function finishEscrow(ctx, wallet, opts) {
  const {
    owner,
    offerSequence,
    gas = 1_000_000,
    expect = "tesSUCCESS",
    memos = null,
  } = opts
  const tx = {
    TransactionType: "EscrowFinish",
    Account: wallet.address,
    Owner: owner,
    OfferSequence: parseInt(offerSequence),
    Gas: gas,
  }
  if (memos) tx.Memos = memos
  const response = await ctx.submit(tx, wallet)
  expectResult(response, expect, "EscrowFinish")
  return response
}

/**
 * Assert `response.result.meta.TransactionResult` matches `expected`.
 * `expected` may be a single code or an array of acceptable codes.
 * Prints a clear label on mismatch and calls `process.exit(1)`.
 */
function expectResult(response, expected, label = "tx") {
  const got = response?.result?.meta?.TransactionResult
  const expectedList = Array.isArray(expected) ? expected : [expected]
  if (!expectedList.includes(got)) {
    console.error(
      `\n${label}: expected ${expectedList.join(" | ")} but got ${got}`,
    )
    process.exit(1)
  }
}

/**
 * Assert the response's `AffectedNodes` contains a `DeletedNode` of type
 * `Escrow` — i.e. the escrow was consumed by the transaction.
 */
function expectEscrowConsumed(response, label = "escrow") {
  const consumed = _hasEscrowDeletion(response)
  if (!consumed) {
    console.error(
      `\n${label}: expected escrow to be consumed (DeletedNode) but it survived`,
    )
    process.exit(1)
  }
}

/**
 * Assert the response's `AffectedNodes` does NOT contain a `DeletedNode` of
 * type `Escrow` — i.e. the escrow survived the transaction (e.g. after a
 * `tecBYTECODE_REJECTED` mid-phase result).
 */
function expectEscrowSurvived(response, label = "escrow") {
  const consumed = _hasEscrowDeletion(response)
  if (consumed) {
    console.error(
      `\n${label}: expected escrow to survive but a DeletedNode of type Escrow appeared`,
    )
    process.exit(1)
  }
}

function _hasEscrowDeletion(response) {
  const nodes = response?.result?.meta?.AffectedNodes ?? []
  return nodes.some(
    (node) => node.DeletedNode && node.DeletedNode.LedgerEntryType === "Escrow",
  )
}

/**
 * Read the `close_time` of the most-recent validated ledger.
 * @returns {Promise<number>} Ripple-epoch seconds.
 */
async function getLedgerCloseTime(client) {
  const res = await client.request({
    command: "ledger",
    ledger_index: "validated",
  })
  return res.result.ledger.close_time
}

/**
 * Read the ISO-8601 `close_time_iso` of the most-recent validated ledger.
 * Used by the oracle test (which asserts against ISO timestamps).
 * @returns {Promise<string>}
 */
async function getLedgerCloseTimeIso(client) {
  const res = await client.request({
    command: "ledger",
    ledger_index: "validated",
  })
  return res.result.ledger.close_time_iso
}

/**
 * Load a sibling example's release WASM as a hex string.
 *
 * @param {string} callerDir  The calling test's `__dirname`.
 * @param {string} relative   Path relative to `callerDir`, e.g.
 *                            "../../../target/wasm32v1-none/release/atomic_swap2.wasm"
 */
function loadWasmHex(callerDir, relative) {
  const abs = path.resolve(callerDir, relative)
  try {
    return fs.readFileSync(abs).toString("hex")
  } catch (err) {
    console.error(`Failed to load WASM at ${abs}: ${err.message}`)
    console.error(`Make sure the sibling contract is built first.`)
    process.exit(1)
  }
}

module.exports = {
  finishEscrow,
  expectResult,
  expectEscrowConsumed,
  expectEscrowSurvived,
  getLedgerCloseTime,
  getLedgerCloseTimeIso,
  loadWasmHex,
}
