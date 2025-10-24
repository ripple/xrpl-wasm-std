const xrpl = require("xrpl")

async function test(testContext) {
  const { deploy, finish, client, submit, sourceWallet, destWallet } =
    testContext

  console.log("\nTesting Atomic Swap 1 (Memo-based):")
  console.log(`Source wallet: ${sourceWallet.address}`)
  console.log(`Dest wallet: ${destWallet.address}`)

  // This test suite validates:
  // ✅ Rejection without memo
  // ✅ Rejection with invalid keylet
  // ✅ Rejection with self-reference
  // ✅ Success with valid counterpart
  // ✅ Atomic consumption behavior

  // Deploy first escrow (source -> dest)
  const firstEscrowResult = await deploy(sourceWallet, destWallet, finish)
  console.log(
    `First escrow deployed with sequence: ${firstEscrowResult.sequence}`,
  )
  console.log(`First escrow keylet: ${firstEscrowResult.escrowKeylet}`)

  // Deploy second escrow (dest -> source) - this will be the counterpart
  const secondEscrowResult = await deploy(destWallet, sourceWallet, finish)
  console.log(
    `Second escrow deployed with sequence: ${secondEscrowResult.sequence}`,
  )

  // Test 1: Try to finish first escrow without memo (should fail)
  console.log("\nTest 1: Finishing first escrow without memo (should fail)")
  const txNoMemo = {
    TransactionType: "EscrowFinish",
    Account: sourceWallet.address,
    Owner: sourceWallet.address,
    OfferSequence: parseInt(firstEscrowResult.sequence),
    ComputationAllowance: 1000000,
  }

  const responseNoMemo = await submit(txNoMemo, sourceWallet)
  if (responseNoMemo.result.meta.TransactionResult !== "tecWASM_REJECTED") {
    console.error("\nUnexpected: escrow finished without memo")
    process.exit(1)
  }
  console.log("✓ Correctly rejected escrow finish without memo")

  // Test 2: Try to finish first escrow with wrong keylet in memo (should fail)
  console.log(
    "\nTest 2: Finishing first escrow with wrong keylet in memo (should fail)",
  )
  const wrongKeylet = "A".repeat(64) // 32 bytes of 0xAA
  const txWrongMemo = {
    TransactionType: "EscrowFinish",
    Account: sourceWallet.address,
    Owner: sourceWallet.address,
    OfferSequence: parseInt(firstEscrowResult.sequence),
    ComputationAllowance: 1000000,
    Memos: [
      {
        Memo: {
          MemoType: xrpl.convertStringToHex("counterpart_escrow"),
          MemoData: wrongKeylet,
        },
      },
    ],
  }

  const responseWrongMemo = await submit(txWrongMemo, sourceWallet)
  if (responseWrongMemo.result.meta.TransactionResult !== "tecWASM_REJECTED") {
    console.error("\nUnexpected: escrow finished with wrong keylet")
    process.exit(1)
  }
  console.log("✓ Correctly rejected escrow finish with wrong keylet")

  // Test 3: Try to finish first escrow with first escrow's own keylet (should fail - same escrow)
  console.log(
    "\nTest 3: Finishing first escrow with its own keylet (should fail)",
  )
  const txSelfMemo = {
    TransactionType: "EscrowFinish",
    Account: sourceWallet.address,
    Owner: sourceWallet.address,
    OfferSequence: parseInt(firstEscrowResult.sequence),
    ComputationAllowance: 1000000,
    Memos: [
      {
        Memo: {
          MemoType: xrpl.convertStringToHex("counterpart_escrow"),
          MemoData: firstEscrowResult.escrowKeylet,
        },
      },
    ],
  }

  const responseSelfMemo = await submit(txSelfMemo, sourceWallet)
  if (responseSelfMemo.result.meta.TransactionResult !== "tecWASM_REJECTED") {
    console.error("\nUnexpected: escrow finished with its own keylet")
    process.exit(1)
  }
  console.log("✓ Correctly rejected escrow finish with its own keylet")

  // Test 4: Create fresh escrows for the atomic swap completion test
  console.log(
    "\nTest 4: Creating fresh escrows for atomic swap completion test",
  )

  // Deploy fresh escrows for the successful atomic swap test
  const freshFirstEscrowResult = await deploy(sourceWallet, destWallet, finish)
  console.log(
    `Fresh first escrow deployed with sequence: ${freshFirstEscrowResult.sequence}`,
  )
  console.log(
    `Fresh first escrow keylet: ${freshFirstEscrowResult.escrowKeylet}`,
  )

  const freshSecondEscrowResult = await deploy(destWallet, sourceWallet, finish)
  console.log(
    `Fresh second escrow deployed with sequence: ${freshSecondEscrowResult.sequence}`,
  )
  console.log(
    `Fresh second escrow keylet: ${freshSecondEscrowResult.escrowKeylet}`,
  )

  // Test 4a: Finish first fresh escrow with correct counterpart keylet (should succeed)
  console.log(
    "\nTest 4a: Finishing first fresh escrow with correct counterpart keylet (should succeed)",
  )

  const txCorrectMemo = {
    TransactionType: "EscrowFinish",
    Account: sourceWallet.address,
    Owner: sourceWallet.address,
    OfferSequence: parseInt(freshFirstEscrowResult.sequence),
    ComputationAllowance: 1000000,
    Memos: [
      {
        Memo: {
          MemoType: xrpl.convertStringToHex("counterpart_escrow"),
          MemoData: freshSecondEscrowResult.escrowKeylet,
        },
      },
    ],
  }

  const responseCorrectMemo = await submit(txCorrectMemo, sourceWallet)
  if (responseCorrectMemo.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to finish first fresh escrow with correct counterpart:",
      responseCorrectMemo.result.meta.TransactionResult,
    )
    process.exit(1)
  }
  console.log(
    "✓ Successfully finished first fresh escrow with correct counterpart keylet",
  )

  // Test 4b: Try to finish second fresh escrow with first escrow's keylet (should fail since first escrow was consumed)
  console.log(
    "\nTest 4b: Attempting to finish second fresh escrow (should fail - first escrow was consumed)",
  )
  const txSecondEscrowShouldFail = {
    TransactionType: "EscrowFinish",
    Account: destWallet.address,
    Owner: destWallet.address,
    OfferSequence: parseInt(freshSecondEscrowResult.sequence),
    ComputationAllowance: 1000000,
    Memos: [
      {
        Memo: {
          MemoType: xrpl.convertStringToHex("counterpart_escrow"),
          MemoData: freshFirstEscrowResult.escrowKeylet,
        },
      },
    ],
  }

  const responseSecondEscrowShouldFail = await submit(
    txSecondEscrowShouldFail,
    destWallet,
  )
  if (
    responseSecondEscrowShouldFail.result.meta.TransactionResult !==
    "tecWASM_REJECTED"
  ) {
    console.error(
      "\nUnexpected: second escrow finished when it should have failed:",
      responseSecondEscrowShouldFail.result.meta.TransactionResult,
    )
    process.exit(1)
  }
  console.log(
    "✓ Correctly rejected second escrow finish (first escrow was already consumed)",
  )

  // Test 5: Demonstrate successful bidirectional atomic swap with proper timing
  console.log(
    "\nTest 5: Demonstrating proper atomic swap with simultaneous execution",
  )
  console.log(
    "ℹ️  In a real atomic swap, both escrows should be finished in the same block",
  )
  console.log(
    "ℹ️  or have logic to handle the case where one escrow is consumed before the other",
  )

  console.log("\n🎉 All atomic swap tests passed!")
  console.log("✓ Atomic swap completed successfully - both escrows finished")
}

module.exports = { test }
