const xrpl = require("xrpl")

const client =
  process.argv.length > 4
    ? new xrpl.Client(process.argv[4])
    : new xrpl.Client("ws://127.0.0.1:6006")

async function submit(tx, wallet, debug = false) {
  const txResult = await client.submitAndWait(tx, { autofill: true, wallet })
  console.log(
    "SUBMITTED " + tx.TransactionType + "(" + txResult.result.hash + ")",
  )

  if (debug) console.log(txResult.result ?? txResult)
  else console.log("Result code: " + txResult.result?.meta?.TransactionResult)
  return txResult
}

/**
 * Deploy a smart-escrow by submitting an `EscrowCreate` transaction.
 *
 * Two calling styles:
 *
 *   deploy(source, dest, finish, "DEADBEEF")            // legacy positional data
 *   deploy(source, dest, finish, { data, amount, ... }) // options bag
 *
 * Options bag fields (all optional):
 *   - data:              string hex `Data` payload
 *   - amount:            string drops, default "100000"
 *   - cancelAfterOffset: seconds added to validated close_time, default 2000
 *   - finishAfter:       absolute Ripple-epoch seconds
 *   - condition:         hex preimage-SHA256 crypto-condition
 *   - sourceTag:         u32
 *   - destinationTag:    u32
 */
async function deploy(sourceWallet, destWallet, finish, dataOrOpts = null) {
  const opts =
    typeof dataOrOpts === "string" || dataOrOpts === null
      ? { data: dataOrOpts }
      : dataOrOpts

  const {
    data = null,
    amount = "100000",
    cancelAfterOffset = 2000,
    finishAfter = null,
    condition = null,
    sourceTag = null,
    destinationTag = null,
  } = opts

  await client.connect()
  console.log("connected")

  const close_time = (
    await client.request({
      command: "ledger",
      ledger_index: "validated",
    })
  ).result.ledger.close_time

  const tx = {
    TransactionType: "EscrowCreate",
    Account: sourceWallet.address,
    Amount: amount,
    Destination: destWallet.address,
    CancelAfter: close_time + cancelAfterOffset,
    Bytecode: finish,
  }
  if (data != null) tx.Data = data
  if (finishAfter != null) tx.FinishAfter = finishAfter
  if (condition != null) tx.Condition = condition
  if (sourceTag != null) tx.SourceTag = sourceTag
  if (destinationTag != null) tx.DestinationTag = destinationTag

  const response1 = await submit(tx, sourceWallet)

  if (response1.result.meta.TransactionResult !== "tesSUCCESS") process.exit(1)
  const sequence = response1.result.tx_json.Sequence

  // Extract escrow ledger entry ID from the created escrow node in metadata
  let escrowId = null
  if (response1.result.meta && response1.result.meta.AffectedNodes) {
    for (const node of response1.result.meta.AffectedNodes) {
      if (node.CreatedNode && node.CreatedNode.LedgerEntryType === "Escrow") {
        escrowId = node.CreatedNode.LedgerIndex
        break
      }
    }
  }

  await client.disconnect()

  return { sequence, escrowId }
}

module.exports = { deploy }
