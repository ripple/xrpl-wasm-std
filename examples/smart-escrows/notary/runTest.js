const xrpl = require("xrpl")

const notary = xrpl.Wallet.fromSeed("snoPBrXtMeMyMHUVTgbuqAfg1SUTb", {
  algorithm: xrpl.ECDSA.secp256k1,
})

async function test(testContext) {
  const { sourceWallet, deploy, finish, destWallet, finishEscrow } = testContext

  const escrowResult = await deploy(sourceWallet, destWallet, finish)

  // Non-notary submitter is rejected.
  await finishEscrow(testContext, sourceWallet, {
    owner: sourceWallet.address,
    offerSequence: escrowResult.sequence,
    expect: "tecBYTECODE_REJECTED",
  })

  // Notary succeeds.
  await finishEscrow(testContext, notary, {
    owner: sourceWallet.address,
    offerSequence: escrowResult.sequence,
  })
}

module.exports = { test }
