if (process.argv.length != 2) {
  console.error("Usage: " + process.argv[0] + " " + process.argv[1])
  process.exit(1)
}

////////////////////////////////////////////////////////////////////////
//  Get all necessary files from rippled
////////////////////////////////////////////////////////////////////////
const path = require("path")
const fs = require("fs/promises")

async function readFile(folder, filename) {
  const filePath = path.join(folder, filename)
  try {
    return await fs.readFile(filePath, "utf-8")
  } catch (e) {
    throw new Error(`File not found: ${filePath}, ${e.message}`)
  }
}

async function main() {
  const rustHostFunctionFile = await readFile(
    __dirname,
    "../xrpl-wasm-stdlib/src/host/host_bindings.rs",
  )

  let rustHits = [
    ...rustHostFunctionFile.matchAll(
      /^ *pub fn ([A-Za-z0-9_]+)\([ \n]*([A-Za-z0-9_:*, \n]*)\) -> ([A-Za-z0-9]+);$/gm,
    ),
  ]
  const rustFuncs = rustHits.map((hit) => [
    hit[1],
    hit[2]
      .trim()
      .split(",")
      .map((s) => s.trim())
      .filter((s) => s.length > 0)
      .map((s) => "_" + s)
      .join(", "),
    hit[3],
  ])

  const rustHostFunctions =
    `// This file exists as a host_binding stand-in for non-WASM targets. For example, this file will
// be used during unit tests.\n\n` +
    rustFuncs
      .map((hit) => {
        return `#[allow(unused)]
    ${(hit[1].match(/:/g) || []).length > 7 ? "#[allow(clippy::too_many_arguments)]\n" : ""}#[allow(clippy::missing_safety_doc)]
    pub unsafe fn ${hit[0]}(${hit[1]}) -> ${hit[2]} {
        ${hit[0].includes("keylet") ? "32" : "-1"}
    }`
      })
      .join("\n\n")

  const rustHostFunctionTestFile = path.join(
    __dirname,
    "../xrpl-wasm-stdlib/src/host/host_bindings_for_testing.rs",
  )
  await fs.writeFile(rustHostFunctionTestFile, rustHostFunctions, "utf8")
  console.log(`Wrote host bindings for testing to ${rustHostFunctionTestFile}`)
}

main()
