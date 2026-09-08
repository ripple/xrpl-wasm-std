async function test(testContext) {
  const { deploy, finish, sourceWallet, destWallet, finishEscrow } = testContext

  const escrowResult = await deploy(sourceWallet, destWallet, finish)

  await finishEscrow(testContext, sourceWallet, {
    owner: sourceWallet.address,
    offerSequence: escrowResult.sequence,
  })
}

module.exports = { test }
