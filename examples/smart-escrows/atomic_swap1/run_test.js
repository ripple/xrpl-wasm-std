const xrpl = require("xrpl")

async function test(testContext) {
  const { deploy, finish, client, submit, sourceWallet, destWallet } =
    testContext

  console.log("\nTesting Atomic Swap 1 (Memo-based):")
  console.log(`Source wallet: ${sourceWallet.address}`)
  console.log(`Dest wallet: ${destWallet.address}`)

  // Deploy first escrow (source -> dest)
  const firstEscrowSequence = await deploy(sourceWallet, destWallet, finish)
  console.log(`First escrow deployed with sequence: ${firstEscrowSequence}`)

  // Generate keylet for the first escrow
  const firstEscrowKeylet = xrpl.getEscrowKeylet(
    sourceWallet.address,
    parseInt(firstEscrowSequence),
  )
  console.log(`First escrow keylet: ${firstEscrowKeylet}`)

  // Deploy second escrow (dest -> source) - this will be the counterpart
  const secondEscrowSequence = await deploy(destWallet, sourceWallet, finish)
  console.log(`Second escrow deployed with sequence: ${secondEscrowSequence}`)

  // Test 1: Try to finish first escrow without memo (should fail)
  console.log("\nTest 1: Finishing first escrow without memo (should fail)")
  const txNoMemo = {
    TransactionType: "EscrowFinish",
    Account: sourceWallet.address,
    Owner: sourceWallet.address,
    OfferSequence: parseInt(firstEscrowSequence),
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
    OfferSequence: parseInt(firstEscrowSequence),
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
    OfferSequence: parseInt(firstEscrowSequence),
    ComputationAllowance: 1000000,
    Memos: [
      {
        Memo: {
          MemoType: xrpl.convertStringToHex("counterpart_escrow"),
          MemoData: firstEscrowKeylet,
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

  // Test 4: Finish first escrow with correct counterpart keylet (should succeed)
  console.log(
    "\nTest 4: Finishing first escrow with correct counterpart keylet (should succeed)",
  )
  const secondEscrowKeylet = xrpl.getEscrowKeylet(
    destWallet.address,
    parseInt(secondEscrowSequence),
  )
  console.log(`Second escrow keylet: ${secondEscrowKeylet}`)

  const txCorrectMemo = {
    TransactionType: "EscrowFinish",
    Account: sourceWallet.address,
    Owner: sourceWallet.address,
    OfferSequence: parseInt(firstEscrowSequence),
    ComputationAllowance: 1000000,
    Memos: [
      {
        Memo: {
          MemoType: xrpl.convertStringToHex("counterpart_escrow"),
          MemoData: secondEscrowKeylet,
        },
      },
    ],
  }

  const responseCorrectMemo = await submit(txCorrectMemo, sourceWallet)
  if (responseCorrectMemo.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to finish first escrow with correct counterpart:",
      responseCorrectMemo.result.meta.TransactionResult,
    )
    process.exit(1)
  }
  console.log(
    "✓ Successfully finished first escrow with correct counterpart keylet",
  )

  // Test 5: Finish second escrow with first escrow's keylet (should succeed)
  console.log(
    "\nTest 5: Finishing second escrow with first escrow keylet (should succeed)",
  )
  const txSecondEscrow = {
    TransactionType: "EscrowFinish",
    Account: destWallet.address,
    Owner: destWallet.address,
    OfferSequence: parseInt(secondEscrowSequence),
    ComputationAllowance: 1000000,
    Memos: [
      {
        Memo: {
          MemoType: xrpl.convertStringToHex("counterpart_escrow"),
          MemoData: firstEscrowKeylet,
        },
      },
    ],
  }

  const responseSecondEscrow = await submit(txSecondEscrow, destWallet)
  if (responseSecondEscrow.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to finish second escrow:",
      responseSecondEscrow.result.meta.TransactionResult,
    )
    process.exit(1)
  }
  console.log("✓ Successfully finished second escrow")

  console.log("\n🎉 All atomic swap tests passed!")
  console.log("✓ Atomic swap completed successfully - both escrows finished")
}

module.exports = { test }
