#!/usr/bin/env node

const xrpl = require("xrpl")
const fs = require("fs")
const path = require("path")
const { execSync } = require("child_process")

const WASM_PATH = path.join(
  __dirname,
  "../e2e-tests/target/wasm32v1-none/release/gas_benchmark.wasm",
)
const BENCHMARK_DIR = path.join(__dirname, "../.benchmark")
const RESULTS_FILE = path.join(BENCHMARK_DIR, "gas_benchmark_results.json")
const NETWORK_URL = "ws://127.0.0.1:6006"
const COMPUTATION_ALLOWANCE = 1000000
const NUM_RUNS = 5

const client = new xrpl.Client(NETWORK_URL)

async function submit(tx, wallet) {
  const result = await client.submitAndWait(tx, { autofill: true, wallet })
  return result
}

async function fundWallet(wallet = undefined) {
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

function getWasmHex(filePath) {
  if (!fs.existsSync(filePath)) {
    throw new Error(`WASM file not found: ${filePath}`)
  }
  const data = fs.readFileSync(filePath)
  return data.toString("hex")
}

function getBinarySize(filePath) {
  if (!fs.existsSync(filePath)) {
    throw new Error(`WASM file not found: ${filePath}`)
  }
  return fs.statSync(filePath).size
}

async function deployEscrow(sourceWallet, destWallet, wasmHex) {
  // Get current ledger close time for CancelAfter
  const ledgerInfo = await client.request({
    command: "ledger",
    ledger_index: "validated",
  })
  const closeTime = ledgerInfo.result.ledger.close_time

  const tx = {
    TransactionType: "EscrowCreate",
    Account: sourceWallet.address,
    Amount: "100000",
    Destination: destWallet.address,
    CancelAfter: closeTime + 2000,
    FinishFunction: wasmHex,
  }

  const result = await submit(tx, sourceWallet)
  if (result.result?.meta?.TransactionResult !== "tesSUCCESS") {
    throw new Error(
      `Failed to create escrow: ${result.result?.meta?.TransactionResult}`,
    )
  }

  // Return the sequence number of the EscrowCreate transaction
  return result.result.tx_json.Sequence
}

async function executeEscrow(sourceWallet, destWallet, offerSequence) {
  const tx = {
    TransactionType: "EscrowFinish",
    Account: sourceWallet.address,
    Owner: sourceWallet.address,
    OfferSequence: parseInt(offerSequence),
    ComputationAllowance: COMPUTATION_ALLOWANCE,
  }

  const result = await submit(tx, sourceWallet)
  if (result.result?.meta?.TransactionResult !== "tesSUCCESS") {
    throw new Error(
      `Failed to finish escrow: ${result.result?.meta?.TransactionResult}`,
    )
  }

  const gasUsed = result.result?.meta?.GasUsed || 0
  return gasUsed
}

async function measureGas() {
  console.log(`\n=== Measuring gas ===`)

  // Build the contract
  console.log("Building contract...")
  try {
    execSync(
      "cd e2e-tests && cargo build -p gas_benchmark --target wasm32v1-none --release",
      { stdio: "inherit" },
    )
  } catch (error) {
    throw new Error(`Failed to build contract: ${error.message}`)
  }

  // Get binary size
  const binarySize = getBinarySize(WASM_PATH)
  console.log(`Binary size: ${binarySize} bytes`)

  // Get WASM hex
  const wasmHex = getWasmHex(WASM_PATH)

  // Connect to network
  await client.connect()
  console.log("Connected to network")

  // Setup ledger acceptance interval for local testing
  let interval
  if (client.url.includes("localhost") || client.url.includes("127.0.0.1")) {
    interval = setInterval(() => {
      if (client.isConnected()) {
        client.request({ command: "ledger_accept" })
      }
    }, 1000)
  }

  try {
    // Fund wallets
    const sourceWallet = await fundWallet()
    const destWallet = await fundWallet()
    console.log(`Source wallet: ${sourceWallet.address}`)
    console.log(`Dest wallet: ${destWallet.address}`)

    // Execute escrow multiple times and measure gas
    const gasReadings = []
    for (let i = 0; i < NUM_RUNS; i++) {
      // Deploy escrow with contract
      console.log(`Run ${i + 1}/${NUM_RUNS}...`)
      console.log("  Deploying escrow with contract...")
      let offerSequence = await deployEscrow(sourceWallet, destWallet, wasmHex)
      console.log(`  Escrow created with sequence: ${offerSequence}`)

      // Execute escrow and measure gas
      const gas = await executeEscrow(sourceWallet, destWallet, offerSequence)
      gasReadings.push(gas)
      console.log(`  Gas used: ${gas}`)
    }

    // Calculate statistics
    const avgGas = gasReadings.reduce((a, b) => a + b, 0) / gasReadings.length
    const stdDev = Math.sqrt(
      gasReadings.reduce((sum, val) => sum + Math.pow(val - avgGas, 2), 0) /
        gasReadings.length,
    )

    return {
      branch,
      binarySize,
      gasReadings,
      avgGas,
      stdDev,
      minGas: Math.min(...gasReadings),
      maxGas: Math.max(...gasReadings),
    }
  } finally {
    if (interval) clearInterval(interval)
    await client.disconnect()
  }
}

async function main() {
  console.log("Gas Benchmark Tool")
  console.log("==================")

  try {
    // Measure gas for current branch
    const results = await measureGas()

    // Ensure benchmark directory exists
    if (!fs.existsSync(BENCHMARK_DIR)) {
      fs.mkdirSync(BENCHMARK_DIR, { recursive: true })
    }

    // Load existing results if they exist
    let allResults = {
      timestamp: new Date().toISOString(),
    }

    if (fs.existsSync(RESULTS_FILE)) {
      const existing = JSON.parse(fs.readFileSync(RESULTS_FILE, "utf8"))
      allResults = existing
      allResults.timestamp = new Date().toISOString()
    }

    // Save results - if we already have optimized results, this becomes baseline
    if (allResults.optimized && !allResults.baseline) {
      allResults.baseline = allResults.optimized
      allResults.optimized = results
    } else {
      allResults.optimized = results
    }

    fs.writeFileSync(RESULTS_FILE, JSON.stringify(allResults, null, 2))
    console.log(`\nResults saved to ${RESULTS_FILE}`)

    // Print summary
    console.log("\n=== Summary ===")
    console.log(`Binary size: ${results.binarySize} bytes`)
    console.log(`Average gas: ${results.avgGas.toFixed(2)}`)
    console.log(`Std dev: ${results.stdDev.toFixed(2)}`)
    console.log(`Min gas: ${results.minGas}`)
    console.log(`Max gas: ${results.maxGas}`)
  } catch (error) {
    console.error("Error:", error.message)
    process.exit(1)
  }
}

main().catch(console.error)
