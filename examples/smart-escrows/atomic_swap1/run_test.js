const xrpl = require("xrpl")

async function test(testContext) {
  const { deploy, finish, submit, sourceWallet, destWallet } = testContext

  // This test suite validates:
  // ✅ Rejection without memo
  // ✅ Rejection with invalid keylet
  // ✅ Rejection with self-reference
  // ✅ Success with valid counterpart
  // ✅ Atomic consumption behavior

  // Deploy first escrow (source -> dest)
  const firstEscrowResult = await deploy(sourceWallet, destWallet, finish)

  // Deploy second escrow (dest -> source) - this will be the counterpart
  await deploy(destWallet, sourceWallet, finish)

  // Test 1: Try to finish first escrow without memo (should fail)
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
  // Test 2: Try to finish first escrow with wrong keylet in memo (should fail)
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
  // Test 3: Try to finish first escrow with first escrow's own keylet (should fail - same escrow)
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
  // Test 4: Create fresh escrows for the atomic swap completion test
  // Deploy fresh escrows for the successful atomic swap test
  const freshFirstEscrowResult = await deploy(sourceWallet, destWallet, finish)

  const freshSecondEscrowResult = await deploy(destWallet, sourceWallet, finish)

  // Test 4a: Finish first fresh escrow with correct counterpart keylet (should succeed)

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
  // Test 4b: Try to finish second fresh escrow with first escrow's keylet (should fail since first escrow was consumed)
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
}

module.exports = { test }
