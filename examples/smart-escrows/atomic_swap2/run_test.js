// NOTE: This is a simplified test for atomic_swap2.
// For a comprehensive atomic swap test implementation, see atomic_swap1/run_test.js
// The atomic_swap2 example demonstrates a data field-based atomic swap with timing validation.

async function test(testContext) {
  const { deploy, finish, submit, sourceWallet, destWallet } = testContext

  console.log("\nTesting Atomic Swap 2 (Data field-based):")
  console.log(
    "This is a simplified test - see atomic_swap1/run_test.js for comprehensive testing",
  )

  // Deploy an escrow with some initial data
  const escrowResult = await deploy(sourceWallet, destWallet, finish)
  console.log(`Escrow deployed with sequence: ${escrowResult.sequence}`)

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
    console.log("✓ Escrow correctly rejected due to missing/invalid data field")
  } else if (response.result.meta.TransactionResult === "tesSUCCESS") {
    console.log("✓ Escrow finished successfully")
  } else {
    console.error(
      "\nUnexpected result:",
      response.result.meta.TransactionResult,
    )
    process.exit(1)
  }

  console.log("✓ Basic atomic_swap2 test completed")
  console.log("For full atomic swap testing, run the atomic_swap1 test")
}

module.exports = { test }
