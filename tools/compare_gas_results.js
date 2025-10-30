#!/usr/bin/env node

const fs = require("fs")
const path = require("path")

const BENCHMARK_DIR = path.join(__dirname, "../.benchmark")

// Get contract names from command line arguments, or find all result files
function getResultFiles() {
  const args = process.argv.slice(2)

  if (args.length > 0) {
    // Use specified contracts
    return args.map((contract) =>
      path.join(BENCHMARK_DIR, `${contract}_results.json`),
    )
  }

  // Find all *_results.json files in benchmark directory
  if (!fs.existsSync(BENCHMARK_DIR)) {
    return []
  }

  const files = fs.readdirSync(BENCHMARK_DIR)
  return files
    .filter((file) => file.endsWith("_results.json"))
    .map((file) => path.join(BENCHMARK_DIR, file))
}

function formatNumber(num) {
  return num.toFixed(2).replace(/\B(?=(\d{3})+(?!\d))/g, ",")
}

function generateContractDetails(contractName, results) {
  const { current, previous } = results

  let report = `### ${contractName}\n\n`

  if (previous) {
    report += `#### Detailed Results\n\n`
    report += `**Previous Measurement:**\n`
    report += `- Binary Size: ${previous.binarySize.toLocaleString()} bytes\n`
    report += `- Average Gas: ${formatNumber(previous.avgGas)}\n`
    report += `- Std Dev: ${formatNumber(previous.stdDev)}\n`
    report += `- Gas Readings: ${previous.gasReadings.join(", ")}\n\n`
  }

  report += `**Current Measurement:**\n`
  report += `- Binary Size: ${current.binarySize.toLocaleString()} bytes\n`
  report += `- Average Gas: ${formatNumber(current.avgGas)}\n`
  report += `- Std Dev: ${formatNumber(current.stdDev)}\n`
  report += `- Gas Readings: ${current.gasReadings.join(", ")}\n\n`

  return report
}

function generateSummaryRow(contractName, results) {
  const { current } = results

  const binarySize = current.binarySize.toLocaleString()
  return `| ${contractName} | ${binarySize} | ${formatNumber(current.avgGas)} | ${formatNumber(current.stdDev)} |\n`
}

function main() {
  const resultFiles = getResultFiles()

  if (resultFiles.length === 0) {
    console.error(`No results files found in ${BENCHMARK_DIR}`)
    console.error("Run 'node tools/gas_benchmark.js' first to generate results")
    process.exit(1)
  }

  // Generate unified report for all contracts
  let unifiedReport = `# Gas Benchmark Report\n\n`

  // Get timestamp and branch from first result file
  const firstResults = JSON.parse(fs.readFileSync(resultFiles[0], "utf8"))
  const timestamp = firstResults.timestamp
  const branch = firstResults.branch || "unknown"

  unifiedReport += `Generated: ${timestamp}\n`
  unifiedReport += `Branch: ${branch}\n\n`

  unifiedReport += `## Summary\n\n`
  unifiedReport += `| Contract | Binary Size | Avg Gas | Std Dev |\n`
  unifiedReport += `|----------|-------------|---------|----------|\n`

  // Collect all results for summary table
  const allResults = {}
  for (const resultsFile of resultFiles) {
    if (!fs.existsSync(resultsFile)) {
      console.error(`Results file not found: ${resultsFile}`)
      continue
    }

    const results = JSON.parse(fs.readFileSync(resultsFile, "utf8"))
    const contractName = path.basename(resultsFile, "_results.json")
    allResults[contractName] = results

    unifiedReport += generateSummaryRow(contractName, results)
  }

  unifiedReport += `\n## Details\n\n`

  // Generate detailed section for each contract
  for (const contractName in allResults) {
    unifiedReport += generateContractDetails(
      contractName,
      allResults[contractName],
    )
  }

  // Add notes section
  unifiedReport += `## Notes\n\n`
  unifiedReport += `- Gas measurements are taken from multiple runs per contract\n`
  unifiedReport += `- Standard deviation indicates variance in gas usage across runs\n`
  unifiedReport += `- Binary size is deterministic and should be identical across runs\n`
  unifiedReport += `- Negative gas changes indicate improvements (less gas consumed)\n`

  // Write unified report
  const reportFile = path.join(BENCHMARK_DIR, "GAS_BENCHMARK_REPORT.md")
  fs.writeFileSync(reportFile, unifiedReport)
  console.log(`Report generated: ${reportFile}`)
  console.log("\n" + unifiedReport)
}

main()
