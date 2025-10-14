const xrpl = require("xrpl");

async function test(testContext) {
  const { client, submit, sourceWallet, offerSequence } = testContext;
  try {
    const tx = {
      TransactionType: "EscrowFinish",
      Account: sourceWallet.address,
      Owner: sourceWallet.address,
      OfferSequence: parseInt(offerSequence),
      ComputationAllowance: 1000000,
    };

    // Submitting EscrowFinish transaction...
    const response = await submit(tx, sourceWallet);

    if (response.result.meta.TransactionResult !== "tesSUCCESS") {
      console.error(
        "\nFailed to finish escrow:",
        response.result.meta.TransactionResult,
      );
      process.exit(1);
    }
  } catch (error) {
    console.error("Error:", error.message);
    console.log(error);
    process.exit(1);
  } finally {
    await client.disconnect();
  }
}

module.exports = { test };
