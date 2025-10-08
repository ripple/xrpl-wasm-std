const xrpl = require("xrpl")

function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

const url = process.argv.length > 4 ? process.argv[4] : "ws://127.0.0.1:6006"
const client = new xrpl.Client(url)

async function submit(tx, wallet, debug = false) {
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
    console.log("Connecting...")
    await client.connect()

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

    if (responseFail.result.meta.TransactionResult !== "tecWASM_REJECTED") {
      console.log("\nEscrow finished successfully?????")
      process.exit(1)
    }

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
      process.exit(1)
    }

  } catch (error) {
    console.error("Error:", error.message)
    console.log(error)
    process.exit(1)
  } finally {
    await client.disconnect()
    console.log("Disconnected")
  }
}

module.exports = { test }
