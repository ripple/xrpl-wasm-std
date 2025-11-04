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
  addLine("use core::marker::PhantomData;")
  addLine("use crate::core::ledger_objects::FieldGetter;")
  addLine("use crate::core::types::uint::{Hash128, Hash160, Hash192, Hash256};")
  addLine("use crate::core::types::account_id::AccountID;")
  addLine("use crate::core::types::amount::Amount;")
  addLine("use crate::core::types::blob::Blob;")
  addLine("use crate::core::types::currency::Currency;")
  addLine("use crate::core::types::issue::Issue;\n")
  addLine("/// A type-safe wrapper for XRPL serialized field codes.")
  addLine("///")
  addLine(
    "/// This struct encodes both the field code and the expected type as const generics,",
  )
  addLine(
    "/// allowing the compiler to automatically infer the correct type when calling `get_field`.",
  )
  addLine("///")
  addLine("/// # Example")
  addLine("///")
  addLine("/// ```rust,no_run")
  addLine("/// use xrpl_wasm_stdlib::core::ledger_objects::ledger_object;")
  addLine("/// use xrpl_wasm_stdlib::sfield;")
  addLine("///")
  addLine("/// // Type is automatically inferred from the SField constant")
  addLine(
    "/// let flags = ledger_object::get_field(0, sfield::Flags).unwrap();  // u32",
  )
  addLine(
    "/// let balance = ledger_object::get_field(0, sfield::Balance).unwrap();  // u64",
  )
  addLine("/// ```")
  addLine("pub struct SField<T: FieldGetter, const CODE: i32> {")
  addLine("    _phantom: PhantomData<T>,")
  addLine("}")
  addLine("")
  addLine("impl<T: FieldGetter, const CODE: i32> SField<T, CODE> {")
  addLine(
    "    /// Creates a new SField constant. This is a const fn that can be used in const contexts.",
  )
  addLine("    pub const fn new() -> Self {")
  addLine("        SField {")
  addLine("            _phantom: PhantomData,")
  addLine("        }")
  addLine("    }")
  addLine("}")
  addLine("")
  addLine(
    "impl<T: FieldGetter, const CODE: i32> From<SField<T, CODE>> for i32 {",
  )
  addLine("    fn from(_: SField<T, CODE>) -> Self {")
  addLine("        CODE")
  addLine("    }")
  addLine("}\n")

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

  // Map XRPL types to Rust types
  // All types now have FieldGetter implementations
  const typeMap = {
    UINT8: "u8",
    UINT16: "u16",
    UINT32: "u32",
    UINT64: "u64",
    UINT128: "Hash128",
    UINT160: "Hash160",
    UINT192: "Hash192",
    UINT256: "Hash256",
    AMOUNT: "Amount",
    ACCOUNT: "AccountID",
    VL: "Blob",
    CURRENCY: "Currency",
    ISSUE: "Issue",
  }

  ////////////////////////////////////////////////////////////////////////
  //  SField processing
  ////////////////////////////////////////////////////////////////////////

  addLine("pub const Invalid: i32 = -1;")
  addLine("pub const Generic: i32 = 0;")
  addLine("pub const hash: i32 = -1;")
  addLine("pub const index: i32 = 0;")
  addLine("")
  addLine(
    "// Legacy i32 constants for backward compatibility with current_tx functions",
  )
  addLine(
    "// These are kept for use with get_field(field_code: i32) in current_tx module",
  )

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
  // Generate all field constants
  for (let x = 0; x < sfieldHits.length; ++x) {
    const fieldName = sfieldHits[x][1]
    const xrplType = sfieldHits[x][2]
    const fieldCode =
      parseInt(stypeMap[xrplType]) * 2 ** 16 + parseInt(sfieldHits[x][3])
    const rustType = typeMap[xrplType]

    // Generate SField constant for types with FieldGetter implementations
    if (rustType) {
      addLine(
        `pub const ${fieldName}: SField<${rustType}, ${fieldCode}> = SField::new();`,
      )
    } else {
      // For types without FieldGetter, keep the old i32 constant for backward compatibility
      addLine(`pub const ${fieldName}: i32 = ${fieldCode};`)
    }
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
