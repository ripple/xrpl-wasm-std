const xrpl = require("xrpl")

function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

const url = "ws://127.0.0.1:6006"
const client = new xrpl.Client(url)

async function submit(tx, wallet, debug = true) {
  const result = await client.submitAndWait(tx, {autofill: true, wallet})
  console.log("SUBMITTED " + tx.TransactionType)
  if (debug)
    console.log(result.result ?? result)
  else
    console.log("Result code: " + result.result?.meta?.TransactionResult)
  return result
}

async function test(sourceWallet, destWallet, offerSequence) {
  try {
    console.log("Connecting to the WASM Devnet...")
    await client.connect()

    await client.request({command: 'ledger_accept'})

    console.log("\nTransaction Details:")
    console.log(`Account (Finishing Escrow): ${sourceWallet.address}`)
    console.log(`Owner (Created Escrow): ${sourceWallet.address}`)
    console.log(`Offer Sequence: ${offerSequence}\n`)

    const txFail = {
      TransactionType: 'EscrowFinish',
      Account: sourceWallet.address,
      Owner: sourceWallet.address,
      OfferSequence: parseInt(offerSequence),
      ComputationAllowance: 1000000,
    }

    console.log("Submitting EscrowFinish transaction... (this should fail)")
    const responseFail = await submit(txFail, sourceWallet)

    if (responseFail.result.meta.TransactionResult === "tesSUCCESS") {
      console.log("\nEscrow finished successfully!")
    } else {
      console.error("\nFailed to finish escrow:", responseFail.result.meta.TransactionResult)
    }

    await sleep(5000)

    const credTx = {
      TransactionType: 'CredentialCreate',
      Account: destWallet.address,
      Subject: destWallet.address,
      CredentialType: xrpl.convertStringToHex('termsandconditions'),
      URI: xrpl.convertStringToHex("https://example.com/terms"),
    }

    console.log("Submitting CredentialCreate transaction...")
    const credResponse = await submit(credTx, destWallet)

    if (credResponse.result.meta.TransactionResult === "tesSUCCESS") {
      console.log("Credential created successfully!")
    } else {
      console.error("\nFailed to create credential:", credResponse.result.meta.TransactionResult)
    }

    await sleep(5000)

    const tx = {
      TransactionType: 'EscrowFinish',
      Account: sourceWallet.address,
      Owner: sourceWallet.address,
      OfferSequence: parseInt(offerSequence),
      ComputationAllowance: 1000000,
    }

    console.log("Submitting EscrowFinish transaction...")
    const response = await submit(tx, sourceWallet)

    if (response.result.meta.TransactionResult === "tesSUCCESS") {
      console.log("\nEscrow finished successfully!")
    } else {
      console.error("\nFailed to finish escrow:", response.result.meta.TransactionResult)
    }

  } catch (error) {
    console.error("Error:", error.message)
    console.log(error)
    process.exit(1)
  } finally {
    await client.disconnect()
    console.log("Disconnected")
    process.exit(1)
  }
}

module.exports = { test }
