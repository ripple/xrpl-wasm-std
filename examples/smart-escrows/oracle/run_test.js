const xrpl = require("xrpl")

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms))
}

const url = process.argv.length > 4 ? process.argv[4] : "ws://127.0.0.1:6006"
const client = new xrpl.Client(url)

const oracleWallet = xrpl.Wallet.fromSeed("snoPBrXtMeMyMHUVTgbuqAfg1SUTb", {
  algorithm: xrpl.ECDSA.secp256k1,
})

async function submit(tx, wallet, debug = false) {
  const result = await client.submitAndWait(tx, { autofill: true, wallet })
  console.log("SUBMITTED " + tx.TransactionType)
  if (debug) console.log(result.result ?? result)
  else console.log("Result code: " + result.result?.meta?.TransactionResult)
  return result
}

async function test(sourceWallet, destWallet, offerSequence) {
  try {
    await client.connect()

    const closeTime = (
      await client.request({
        command: "ledger",
        ledger_index: "validated",
      })
    ).result.ledger.close_time_iso

    const oracleCreate = {
      TransactionType: "OracleSet",
      Account: oracleWallet.address,
      OracleDocumentID: 1,
      Provider: xrpl.convertStringToHex("sample"),
      AssetClass: xrpl.convertStringToHex("currency"),
      LastUpdateTime: Math.floor(new Date(closeTime).getTime() / 1000) + 20,
      PriceDataSeries: [
        {
          PriceData: {
            BaseAsset: "XRP",
            QuoteAsset: "USD",
            AssetPrice: 1,
            Scale: 1,
          },
        },
      ],
    }
    const oracleCreateResponse = await submit(oracleCreate, oracleWallet)
    if (oracleCreateResponse.result.meta.TransactionResult !== "tesSUCCESS") {
      console.error(
        "\nFailed to create oracle:",
        oracleCreateResponse.result.meta.TransactionResult,
      )
      process.exit(1)
    }

    const txFail = {
      TransactionType: "EscrowFinish",
      Account: sourceWallet.address,
      Owner: sourceWallet.address,
      OfferSequence: parseInt(offerSequence),
      ComputationAllowance: 1000000,
    }

    // This EscrowCreate should fail since the oracle must show the price as <= 1 USD/XRP
    const responseFail = await submit(txFail, sourceWallet)

    if (responseFail.result.meta.TransactionResult !== "tecWASM_REJECTED") {
      console.error("\nEscrow finished successfully????")
      process.exit(1)
    }

    const closeTime2 = (
      await client.request({
        command: "ledger",
        ledger_index: "validated",
      })
    ).result.ledger.close_time_iso

    const oracleUpdate = {
      TransactionType: "OracleSet",
      Account: oracleWallet.address,
      OracleDocumentID: 1,
      LastUpdateTime: Math.floor(new Date(closeTime2).getTime() / 1000) + 20,
      PriceDataSeries: [
        {
          PriceData: {
            BaseAsset: "XRP",
            QuoteAsset: "USD",
            AssetPrice: 2,
            Scale: 1,
          },
        },
      ],
    }
    const oracleUpdateResponse = await submit(oracleUpdate, oracleWallet)
    if (oracleUpdateResponse.result.meta.TransactionResult !== "tesSUCCESS") {
      console.error(
        "\nFailed to create oracle:",
        oracleUpdateResponse.result.meta.TransactionResult,
      )
      process.exit(1)
    }

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
  } catch (error) {
    console.error("Error:", error.message)
    console.log(error)
    process.exit(1)
  } finally {
    await client.disconnect()
  }
}

module.exports = { test }
