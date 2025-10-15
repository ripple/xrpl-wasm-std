async function test(testContext) {
  const { deploy, finish, submit, sourceWallet, destWallet } = testContext

  const offerSequence = await deploy(sourceWallet, destWallet, finish)

  const tx = {
    TransactionType: "EscrowFinish",
    Account: sourceWallet.address,
    Owner: sourceWallet.address,
    OfferSequence: parseInt(offerSequence),
    ComputationAllowance: 1000000,
  }

  const response = await submit(tx, sourceWallet)

  if (response.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to finish escrow:",
      response.result.meta.TransactionResult,
    )
    process.exit(1)
  }
}

module.exports = { test }
