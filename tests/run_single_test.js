const xrpl = require("xrpl")
const fs = require("fs")
const path = require("path")

const client =
  process.argv.length > 4
    ? new xrpl.Client(process.argv[4])
    : new xrpl.Client("ws://127.0.0.1:6006")

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

  const { deploy } = require("./deploy_wasm_code.js")

  const offerSequence = await deploy(wallets[0], wallets[1], wasmSource)

  console.log(`Running test in directory: ${targetDir}`)
  const runTestPath = path.resolve(targetDir, "run_test.js")
  const { test } = require(runTestPath)

  // Dynamically import the test function from the target directory

  const testContext = {
    sourceWallet: wallets[0],
    offerSequence,
    destWallet: wallets[1],
    allWallets: wallets,
  }

  await test(client, testContext)

  if (interval) clearInterval(interval)
  await client.disconnect()
}

main().catch(console.error)
