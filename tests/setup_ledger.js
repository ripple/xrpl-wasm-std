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
  const master = xrpl.Wallet.fromSeed("snoPBrXtMeMyMHUVTgbuqAfg1SUTb", { algorithm: xrpl.ECDSA.secp256k1 })

  const walletToFund = wallet || xrpl.Wallet.generate()
  await submit({
    TransactionType: 'Payment',
    Account: "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh",
    Amount: xrpl.xrpToDrops(10000),
    Destination: walletToFund.address,
  }, master)
  return { walletToFund }
}

async function main() {
  console.log("🧪 Setting up accounts...")
  await client.connect()
  console.log("connected")

  let interval
  if (client.url.includes("localhost")) {
    interval = setInterval(() => {if (client.isConnected()) client.request({command: 'ledger_accept'})},1000)
  }

  const wallets = []
  for (let i = 0; i < 5; i++) {
    const { walletToFund } = await fundWallet()
    wallets.push({
      address: walletToFund.address,
      seed: walletToFund.seed,
      publicKey: walletToFund.publicKey
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
