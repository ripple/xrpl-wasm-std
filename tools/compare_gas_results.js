#!/usr/bin/env node

const fs = require("fs")
const path = require("path")

const BENCHMARK_DIR = path.join(__dirname, "../.benchmark")
const RESULTS_FILE = path.join(BENCHMARK_DIR, "gas_benchmark_results.json")
const REPORT_FILE = path.join(BENCHMARK_DIR, "GAS_BENCHMARK_REPORT.md")

function calculatePercentChange(baseline, optimized) {
  if (baseline === 0) return 0
  return ((optimized - baseline) / baseline) * 100
}

function formatNumber(num) {
  return num.toFixed(2)
}

function generateReport(results) {
  const { optimized, baseline } = results

  let report = `# Gas Benchmark Report\n\n`
  report += `Generated: ${results.timestamp}\n\n`

  report += `## Summary\n\n`

  if (baseline) {
    report += `### Binary Size Comparison\n\n`
    report += `| Metric | Baseline | Optimized | Change | % Change |\n`
    report += `|--------|----------|-----------|--------|----------|\n`

    const sizeDiff = optimized.binarySize - baseline.binarySize
    const sizePercent = calculatePercentChange(
      baseline.binarySize,
      optimized.binarySize,
    )
    const sizeChange = sizeDiff > 0 ? `+${sizeDiff}` : `${sizeDiff}`
    const sizePercent_str =
      sizePercent > 0
        ? `+${formatNumber(sizePercent)}%`
        : `${formatNumber(sizePercent)}%`

    report += `| Binary Size | ${baseline.binarySize} bytes | ${optimized.binarySize} bytes | ${sizeChange} bytes | ${sizePercent_str} |\n\n`

    report += `### Gas Usage Comparison\n\n`
    report += `| Metric | Baseline | Optimized | Change | % Change |\n`
    report += `|--------|----------|-----------|--------|----------|\n`

    const gasDiff = optimized.avgGas - baseline.avgGas
    const gasPercent = calculatePercentChange(baseline.avgGas, optimized.avgGas)
    const gasChange =
      gasDiff > 0 ? `+${formatNumber(gasDiff)}` : `${formatNumber(gasDiff)}`
    const gasPercent_str =
      gasPercent > 0
        ? `+${formatNumber(gasPercent)}%`
        : `${formatNumber(gasPercent)}%`

    report += `| Average Gas | ${formatNumber(baseline.avgGas)} | ${formatNumber(optimized.avgGas)} | ${gasChange} | ${gasPercent_str} |\n`
    report += `| Std Dev | ${formatNumber(baseline.stdDev)} | ${formatNumber(optimized.stdDev)} | - | - |\n`
    report += `| Min Gas | ${baseline.minGas} | ${optimized.minGas} | - | - |\n`
    report += `| Max Gas | ${baseline.maxGas} | ${optimized.maxGas} | - | - |\n\n`

    // Interpretation
    report += `## Interpretation\n\n`
    if (gasPercent < -5) {
      report += `✅ **Significant Improvement**: Gas usage reduced by ${Math.abs(formatNumber(gasPercent))}%\n\n`
    } else if (gasPercent < 0) {
      report += `✅ **Minor Improvement**: Gas usage reduced by ${Math.abs(formatNumber(gasPercent))}%\n\n`
    } else if (gasPercent > 5) {
      report += `⚠️ **Regression**: Gas usage increased by ${formatNumber(gasPercent)}%\n\n`
    } else if (gasPercent > 0) {
      report += `⚠️ **Minor Regression**: Gas usage increased by ${formatNumber(gasPercent)}%\n\n`
    } else {
      report += `✅ **No Change**: Gas usage is equivalent\n\n`
    }

    if (sizePercent < -5) {
      report += `✅ **Binary Size Reduced**: ${Math.abs(formatNumber(sizePercent))}% smaller\n\n`
    } else if (sizePercent > 5) {
      report += `⚠️ **Binary Size Increased**: ${formatNumber(sizePercent)}% larger\n\n`
    } else {
      report += `✅ **Binary Size Stable**: No significant change\n\n`
    }
  } else {
    report += `### Optimized Branch Metrics\n\n`
    report += `| Metric | Value |\n`
    report += `|--------|-------|\n`
    report += `| Binary Size | ${optimized.binarySize} bytes |\n`
    report += `| Average Gas | ${formatNumber(optimized.avgGas)} |\n`
    report += `| Std Dev | ${formatNumber(optimized.stdDev)} |\n`
    report += `| Min Gas | ${optimized.minGas} |\n`
    report += `| Max Gas | ${optimized.maxGas} |\n\n`
  }

  report += `## Detailed Results\n\n`

  if (baseline) {
    report += `### Baseline Branch\n`
    report += `- Binary Size: ${baseline.binarySize} bytes\n`
    report += `- Average Gas: ${formatNumber(baseline.avgGas)}\n`
    report += `- Std Dev: ${formatNumber(baseline.stdDev)}\n`
    report += `- Gas Readings: ${baseline.gasReadings.join(", ")}\n\n`
  }

  report += `### Optimized Branch\n`
  report += `- Binary Size: ${optimized.binarySize} bytes\n`
  report += `- Average Gas: ${formatNumber(optimized.avgGas)}\n`
  report += `- Std Dev: ${formatNumber(optimized.stdDev)}\n`
  report += `- Gas Readings: ${optimized.gasReadings.join(", ")}\n\n`

  report += `## Notes\n\n`
  report += `- Gas measurements are taken from ${optimized.gasReadings.length} runs\n`
  report += `- Standard deviation indicates variance in gas usage across runs\n`
  report += `- Binary size is deterministic and should be identical across runs\n`
  report += `- Negative gas changes indicate improvements (less gas consumed)\n`

  return report
}

function main() {
  if (!fs.existsSync(RESULTS_FILE)) {
    console.error(`Results file not found: ${RESULTS_FILE}`)
    console.error("Run 'node tools/gas_benchmark.js' first to generate results")
    process.exit(1)
  }

  const results = JSON.parse(fs.readFileSync(RESULTS_FILE, "utf8"))
  const report = generateReport(results)

  fs.writeFileSync(REPORT_FILE, report)
  console.log(`Report generated: ${REPORT_FILE}`)
  console.log("\n" + report)
}

main()
