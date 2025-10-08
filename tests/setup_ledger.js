const xrpl = require("xrpl")
const fs = require('fs')
const path = require('path')

const client = process.argv.length > 2 ? new xrpl.Client(process.argv[2]) : new xrpl.Client("ws://127.0.0.1:6006")

async function submit(tx, wallet, debug = true) {
  const txResult = await client.submitAndWait(tx, {autofill: true, wallet})
  console.log("SUBMITTED " + tx.TransactionType)

  if (debug)
    console.log(txResult.result ?? txResult)
  else
    console.log("Result code: " + txResult.result?.meta?.TransactionResult)
  return txResult
}

async function fundWallet(wallet = undefined) {
  if (!(client.url.includes("localhost") || client.url.includes("127.0.0.1"))) {
    const walletToFund = wallet || xrpl.Wallet.generate()
    const result = await client.fundWallet(walletToFund)
    return result.wallet
  }
  const master = xrpl.Wallet.fromSeed("snoPBrXtMeMyMHUVTgbuqAfg1SUTb", { algorithm: xrpl.ECDSA.secp256k1 })

  const walletToFund = wallet || xrpl.Wallet.generate()
  await submit({
    TransactionType: 'Payment',
    Account: "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh",
    Amount: xrpl.xrpToDrops(10000),
    Destination: walletToFund.address,
  }, master)
  return walletToFund
}

async function main() {
  console.log("🧪 Setting up accounts...")
  await client.connect()
  console.log("connected")

  let interval
  if (client.url.includes("localhost") || client.url.includes("127.0.0.1")) {
    interval = setInterval(() => {if (client.isConnected()) client.request({command: 'ledger_accept'})},1000)
  }

  const wallets = []
  for (let i = 0; i < 5; i++) {
    const wallet = await fundWallet()
    wallets.push({
      address: wallet.address,
      seed: wallet.seed,
      publicKey: wallet.publicKey
    })
  }

  const filePath = path.join(__dirname, 'wallets.json')
  fs.writeFileSync(filePath, JSON.stringify(wallets, null, 2))
  console.log(`Saved ${wallets.length} wallets to wallets.json`)

  if (interval)
    clearInterval(interval)

  await client.disconnect()
}

main().catch(console.error)
