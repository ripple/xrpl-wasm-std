if (process.argv.length != 3 && process.argv.length != 4) {
  console.error(
    "Usage: " +
      process.argv[0] +
      " " +
      process.argv[1] +
      " path/to/rippled [path/to/pipe/to]",
  )
  console.error(
    "path/to/rippled may also be a GitHub URL, e.g. https://github.com/XRPLF/rippled or https://github.com/XRPLF/rippled/tree/HEAD",
  )
  process.exit(1)
}

////////////////////////////////////////////////////////////////////////
//  Get all necessary files from rippled
////////////////////////////////////////////////////////////////////////
const path = require("path")
const fs = require("fs/promises")

async function readFileFromGitHub(repo, filename) {
  if (!repo.includes("tree")) {
    repo += "/tree/HEAD"
  }
  let url = repo.replace("github.com", "raw.githubusercontent.com")
  url = url.replace("tree/", "")
  url += "/" + filename

  if (!url.startsWith("http")) {
    url = "https://" + url
  }

  try {
    const response = await fetch(url)
    if (!response.ok) {
      throw new Error(`${response.status} ${response.statusText}`)
    }
    return await response.text()
  } catch (e) {
    console.error(`Error reading ${url}: ${e.message}`)
    process.exit(1)
  }
}

async function readFile(folder, filename) {
  const filePath = path.join(folder, filename)
  try {
    return await fs.readFile(filePath, "utf-8")
  } catch (e) {
    throw new Error(`File not found: ${filePath}, ${e.message}`)
  }
}

const read = (() => {
  try {
    const url = new URL(process.argv[2])
    return url.hostname === "github.com" ? readFileFromGitHub : readFile
  } catch {
    return readFile // Default to readFile if process.argv[2] is not a valid URL
  }
})()

async function main() {
  const sfieldHeaderFile = await read(
    process.argv[2],
    "include/xrpl/protocol/SField.h",
  )
  const sfieldMacroFile = await read(
    process.argv[2],
    "include/xrpl/protocol/detail/sfields.macro",
  )

  let output = ""
  function addLine(line) {
    output += line + "\n"
  }

  addLine("#![allow(non_upper_case_globals)]\n")

  // process STypes
  let stypeHits = [
    ...sfieldHeaderFile.matchAll(
      /^ *STYPE\(STI_([^ ]*?)[ \n]*,[ \n]*([0-9-]+)[ \n]*\)[ \n]*\\?$/gm,
    ),
  ]
  if (stypeHits.length === 0)
    stypeHits = [
      ...sfieldHeaderFile.matchAll(
        /^ *STI_([^ ]*?)[ \n]*=[ \n]*([0-9-]+)[ \n]*,?$/gm,
      ),
    ]
  const stypeMap = {}
  stypeHits.forEach(([_, key, value]) => {
    stypeMap[key] = value
  })

  ////////////////////////////////////////////////////////////////////////
  //  SField processing
  ////////////////////////////////////////////////////////////////////////

  addLine("pub const Invalid: i32 = -1;")
  addLine("pub const Generic: i32 = 0;")
  addLine("pub const hash: i32 = -1;")
  addLine("pub const index: i32 = 0;")

  // Parse SField.cpp for all the SFields and their serialization info
  let sfieldHits = [
    ...sfieldMacroFile.matchAll(
      /^ *[A-Z]*TYPED_SFIELD *\( *sf([^,\n]*),[ \n]*([^, \n]+)[ \n]*,[ \n]*([0-9]+)(,.*?(notSigning))?/gm,
    ),
  ]
  sfieldHits.sort((a, b) => {
    const aValue = parseInt(stypeMap[a[2]]) * 2 ** 16 + parseInt(a[3])
    const bValue = parseInt(stypeMap[b[2]]) * 2 ** 16 + parseInt(b[3])
    return aValue - bValue // Ascending order
  })
  for (let x = 0; x < sfieldHits.length; ++x) {
    addLine(
      `pub const ${sfieldHits[x][1]}: i32 = ${parseInt(stypeMap[sfieldHits[x][2]]) * 2 ** 16 + parseInt(sfieldHits[x][3])};`,
    )
  }

  ////////////////////////////////////////////////////////////////////////
  //  Serialized type processing
  ////////////////////////////////////////////////////////////////////////
  // addLine('  "TYPES": {')

  // stypeHits.push(['', 'DONE', -1])
  // stypeHits.sort((a, b) => sorter(translate(a[1]), translate(b[1])))
  // for (let x = 0; x < stypeHits.length; ++x) {
  //   addLine(
  //     '    "' +
  //       translate(stypeHits[x][1]) +
  //       '": ' +
  //       stypeHits[x][2] +
  //       (x < stypeHits.length - 1 ? ',' : ''),
  //   )
  // }

  const outputFile =
    process.argv.length == 4
      ? process.argv[3]
      : path.join(__dirname, "../xrpl-wasm-stdlib/src/sfield.rs")
  try {
    await fs.writeFile(outputFile, output, "utf8")
    console.log("File written successfully to", outputFile)
  } catch (err) {
    console.error("Error writing to file:", err)
  }
}

main()
