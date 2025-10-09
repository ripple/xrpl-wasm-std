const xrpl = require("xrpl")
const fs = require("fs")
const path = require("path")

const client =
  process.argv.length > 4
    ? new xrpl.Client(process.argv[4])
    : new xrpl.Client("ws://127.0.0.1:6006")

function getFinishFunctionFromFile(filePath) {
  if (!filePath) {
    console.error("Please provide a file path as a CLI argument.")
    process.exit(1)
  }

  let absolutePath = ""
  if (filePath.endsWith(".wasm")) {
    absolutePath = path.resolve(filePath)
  } else {
    absolutePath = path.resolve(
      __dirname,
      `../../projects/target/wasm32v1-none/release/${filePath}.wasm`,
    )
  }
  try {
    const data = fs.readFileSync(absolutePath)
    return data.toString("hex")
  } catch (err) {
    console.error(`Error reading file at ${absolutePath}:`, err.message)
    process.exit(1)
  }
}

async function submit(tx, wallet, debug = true) {
  const txResult = await client.submitAndWait(tx, { autofill: true, wallet })
  console.log("SUBMITTED " + tx.TransactionType)

  if (debug) console.log(txResult.result ?? txResult)
  else console.log("Result code: " + txResult.result?.meta?.TransactionResult)
  return txResult
}

async function deploy(sourceWallet, destWallet, wasmSource) {
  await client.connect()
  console.log("connected")

  const finish = getFinishFunctionFromFile(wasmSource)

  const close_time = (
    await client.request({
      command: "ledger",
      ledger_index: "validated",
    })
  ).result.ledger.close_time

  const response1 = await submit(
    {
      TransactionType: "EscrowCreate",
      Account: sourceWallet.address,
      Amount: "100000",
      Destination: destWallet.address,
      CancelAfter: close_time + 2000,
      FinishFunction: finish,
      Data: xrpl.xrpToDrops(70),
    },
    sourceWallet,
  )

  if (response1.result.meta.TransactionResult !== "tesSUCCESS") process.exit(1)
  const sequence = response1.result.tx_json.Sequence

  await client.disconnect()

  return sequence
}

module.exports = { deploy }
