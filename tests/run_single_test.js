const xrpl = require("xrpl")
const fs = require("fs")
const path = require("path")

const client =
  process.argv.length > 4
    ? new xrpl.Client(process.argv[4])
    : new xrpl.Client("ws://127.0.0.1:6006")

async function submit(tx, wallet, debug = false) {
  const result = await client.submitAndWait(tx, { autofill: true, wallet })
  console.log(
    "SUBMITTED " + tx.TransactionType + "(" + result.result.hash + ")",
  )
  if (debug) console.log(result.result ?? result)
  else console.log("Result code: " + result.result?.meta?.TransactionResult)
  return result
}

async function fundWallet(wallet = undefined) {
  if (!(client.url.includes("localhost") || client.url.includes("127.0.0.1"))) {
    const walletToFund = wallet || xrpl.Wallet.generate()
    const result = await client.fundWallet(walletToFund)
    return result.wallet
  }
  const master = xrpl.Wallet.fromSeed("snoPBrXtMeMyMHUVTgbuqAfg1SUTb", {
    algorithm: xrpl.ECDSA.secp256k1,
  })

  const walletToFund = wallet || xrpl.Wallet.generate()
  await submit(
    {
      TransactionType: "Payment",
      Account: "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh",
      Amount: xrpl.xrpToDrops(10000),
      Destination: walletToFund.address,
    },
    master,
  )
  return walletToFund
}

function getFinishFunctionFromFile(filePath) {
  if (!filePath) {
    console.error("Please provide a file path as a CLI argument.")
    process.exit(1)
  }

  const absolutePath = path.resolve(filePath)
  try {
    const data = fs.readFileSync(absolutePath)
    return data.toString("hex")
  } catch (err) {
    console.error(`Error reading file at ${absolutePath}:`, err.message)
    process.exit(1)
  }
}

async function main() {
  await client.connect()
  console.log("connected")

  let interval
  if (client.url.includes("localhost") || client.url.includes("127.0.0.1")) {
    interval = setInterval(() => {
      if (client.isConnected()) client.request({ command: "ledger_accept" })
    }, 1000)
  }

  const walletsPath = path.join(__dirname, "wallets.json")
  const walletsData = JSON.parse(fs.readFileSync(walletsPath, "utf8"))

  const wallets = walletsData.map(({ _address, seed, _pubkey }) => {
    return xrpl.Wallet.fromSeed(seed)
  })

  console.log(`Loaded ${wallets.length} wallets`)

  const args = process.argv.slice(2)
  if (args.length === 0) {
    throw new Error(
      "Please provide a directory path as a command line argument.",
    )
  }
  const targetDir = args[0]
  const wasmSource = args[1]
  const finish = getFinishFunctionFromFile(wasmSource)

  const { deploy } = require("./deploy_wasm_code.js")

  console.log(`Running test in directory: ${targetDir}`)
  const runTestPath = path.resolve(targetDir, "run_test.js")
  const { test } = require(runTestPath)

  // Dynamically import the test function from the target directory

  const testContext = {
    client,
    submit,
    sourceWallet: wallets[0],
    destWallet: wallets[1],
    allWallets: wallets,
    fundWallet,
    deploy,
    finish,
  }
  try {
    await test(testContext)
  } catch (error) {
    console.error("Error:", error.message)
    console.log(error)
  } finally {
    if (interval) clearInterval(interval)
    await client.disconnect()
  }
}

main().catch(console.error)
