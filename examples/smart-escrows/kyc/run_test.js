const xrpl = require("xrpl")

async function submit(client, tx, wallet, debug = false) {
  const result = await client.submitAndWait(tx, { autofill: true, wallet })
  console.log("SUBMITTED " + tx.TransactionType)
  if (debug) console.log(result.result ?? result)
  else console.log("Result code: " + result.result?.meta?.TransactionResult)
  return result
}

async function test(testContext) {
  try {
    const { client, sourceWallet, destWallet, offerSequence } = testContext
    const txFail = {
      TransactionType: "EscrowFinish",
      Account: sourceWallet.address,
      Owner: sourceWallet.address,
      OfferSequence: parseInt(offerSequence),
      ComputationAllowance: 1000000,
    }

    // Submitting EscrowFinish transaction...
    // This should fail since the credential hasn't been created yet
    const responseFail = await submit(client, txFail, sourceWallet)

    if (responseFail.result.meta.TransactionResult !== "tecWASM_REJECTED") {
      console.log("\nEscrow finished successfully?????")
      process.exit(1)
    }

    const credTx = {
      TransactionType: "CredentialCreate",
      Account: destWallet.address,
      Subject: destWallet.address,
      CredentialType: xrpl.convertStringToHex("termsandconditions"),
      URI: xrpl.convertStringToHex("https://example.com/terms"),
    }

    // Submitting CredentialCreate transaction...
    const credResponse = await submit(client, credTx, destWallet)

    if (credResponse.result.meta.TransactionResult !== "tesSUCCESS") {
      console.error(
        "\nFailed to create credential:",
        credResponse.result.meta.TransactionResult,
      )
    }

    const tx = {
      TransactionType: "EscrowFinish",
      Account: sourceWallet.address,
      Owner: sourceWallet.address,
      OfferSequence: parseInt(offerSequence),
      ComputationAllowance: 1000000,
    }

    // Submitting EscrowFinish transaction...
    const response = await submit(client, tx, sourceWallet)

    if (response.result.meta.TransactionResult !== "tesSUCCESS") {
      console.error(
        "\nFailed to finish escrow:",
        response.result.meta.TransactionResult,
      )
      process.exit(1)
    }
  } catch (error) {
    console.error("Error:", error.message)
    console.log(error)
    await client.disconnect()
    process.exit(1)
  }
}

module.exports = { test }
