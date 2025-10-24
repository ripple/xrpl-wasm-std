// NOTE: This is a simplified test for atomic_swap2.
// For a comprehensive atomic swap test implementation, see atomic_swap1/run_test.js
// The atomic_swap2 example demonstrates a data field-based atomic swap with timing validation.

async function test(testContext) {
  const { deploy, finish, submit, sourceWallet, destWallet } = testContext

  // Deploy an escrow with some initial data
  const escrowResult = await deploy(sourceWallet, destWallet, finish)

  // This test just demonstrates basic functionality
  // A full test would involve:
  // 1. Creating escrow with first escrow's keylet in data field
  // 2. First finish attempt (should append CancelAfter and fail)
  // 3. Second finish attempt (should validate timing and succeed if before CancelAfter)

  const tx = {
    TransactionType: "EscrowFinish",
    Account: sourceWallet.address,
    Owner: sourceWallet.address,
    OfferSequence: parseInt(escrowResult.sequence),
    ComputationAllowance: 1000000,
  }

  const response = await submit(tx, sourceWallet)

  // This will likely fail because the data field doesn't contain a valid escrow keylet
  if (response.result.meta.TransactionResult === "tecWASM_REJECTED") {
    // Expected - escrow correctly rejected due to missing/invalid data field
  } else {
    console.error(
      "\nUnexpected result:",
      response.result.meta.TransactionResult,
    )
    process.exit(1)
  }
}

module.exports = { test }
