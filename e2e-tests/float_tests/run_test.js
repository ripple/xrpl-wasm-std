async function test(testContext) {
  try {
    const { submit, sourceWallet, offerSequence } = testContext
    // This escrow should always succeed
    // If it fails, something in rippled is broken

    const txFail = {
      TransactionType: "EscrowFinish",
      Account: sourceWallet.address,
      Owner: sourceWallet.address,
      OfferSequence: parseInt(offerSequence),
      ComputationAllowance: 1000000,
    }

    const responseFail = await submit(txFail, sourceWallet)

    if (responseFail.result.meta.TransactionResult !== "tesSUCCESS") {
      console.error(
        "\nFailed to finish escrow:",
        responseFail.result.meta.TransactionResult,
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
