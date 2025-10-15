async function test(testContext) {
  const {deploy, finish, submit, sourceWallet, destWallet} = testContext

  const offerSequence = await deploy(sourceWallet, destWallet, finish)

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
}

module.exports = {test}
