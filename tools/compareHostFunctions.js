if (process.argv.length !== 3) {
    console.error(
        'Usage: ' +
        process.argv[0] +
        ' ' +
        process.argv[1] +
        ' path/to/rippled',
    )
    process.exit(1)
}

////////////////////////////////////////////////////////////////////////
//  Get all necessary files from rippled
////////////////////////////////////////////////////////////////////////
const path = require('path')
const fs = require('fs/promises')

async function readFileFromGitHub(repo, filename) {
    if (!repo.includes('tree')) {
        repo += '/tree/HEAD'
    }
    let url = repo.replace('github.com', 'raw.githubusercontent.com')
    url = url.replace('tree/', '')
    url += '/' + filename

    if (!url.startsWith('http')) {
        url = 'https://' + url
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

function areListsEqual(a, b) {
    if (a.length !== b.length) return false
    for (let i = 0; i < a.length; i++) {
        if (a[i] !== b[i]) {
            return false
        }
    }
    return true
}

async function readFile(folder, filename) {
    const filePath = path.join(folder, filename)
    try {
        return await fs.readFile(filePath, 'utf-8')
    } catch (e) {
        throw new Error(`File not found: ${filePath}, ${e.message}`)
    }
}

const read = process.argv[2].includes('github.com')
    ? readFileFromGitHub
    : readFile

async function main() {
    const wasmImportFile = await read(
        process.argv[2], 'src/xrpld/app/wasm/detail/WasmVM.cpp',
    )
    const hostWrapperFile = await read(
        process.argv[2], 'src/xrpld/app/wasm/HostFuncWrapper.h',
    )
    const rustHostFunctionFile = await readFile(__dirname, '../xrpl-wasm-std/src/host/host_bindings.rs')
    const rustHostFunctionTestFile = await readFile(__dirname, '../xrpl-wasm-std/src/host/host_bindings_for_testing.rs')

    let importHits = [
        ...wasmImportFile.matchAll(
            // parse the WASM host function imports in `WasmVM.cpp`
            /^ *WASM_IMPORT_FUNC2? *\(i, *([A-Za-z0-9]+), *("([A-Za-z0-9_]+)",)? *hfs, *[0-9']+\);$/gm,
        ),
    ]
    const imports = importHits.map((hit) => [hit[1], hit[3] != null ? hit[3] : hit[1]]).sort((a, b) => a[0].localeCompare(b[0]))

    let wrapperHits = [
        ...hostWrapperFile.matchAll(
            // parse the `proto` functions in `HostFuncWrapper.h.h`
            /^ *using ([A-Za-z0-9]+)_proto =[ \n]*([A-Za-z0-9_]+)\(([A-Za-z0-9_\* \n,]*)\);$/gm,
        ),
    ]
    const wrappers = wrapperHits.map((hit) => [hit[1], hit[2], hit[3].length == 0 ? [] : hit[3].split(',').map((s) => s.trim())]).sort((a, b) => a[0].localeCompare(b[0]))
    if (!areListsEqual(imports.map(f => f[0]), wrappers.map(f => f[0]))) {
        console.error('Imports and C++ Host Functions do not match!')
        const importsMissing = wrappers.filter(f => !imports.some(func => func[0] === f[0]))
        const hfMissing = imports.filter(f => !wrappers.some(func => func[0] === f[0]))
        if (importsMissing.length > 0)
            console.error('Missing Imports:', '\x1b[31m' + importsMissing.map(func => func[0]).join(', ') + '\x1b[0m')
        if (hfMissing.length > 0)
            console.error('Missing C++ Host Functions:', '\x1b[31m' + hfMissing.map(func => func[0]).join(', ') + '\x1b[0m')
        process.exit(1)
    }

    for (let i = 0; i < imports.length; i++) {
        if (imports[i][0] !== wrappers[i][0]) {
            console.error(
                'Imports and Host Functions do not match at index ' +
                i +
                ': ' +
                imports[i][0] +
                ' !== ' +
                wrappers[i][0],
            )
            process.exit(1)
        }
    }

    const cppHostFunctions = imports.map((hit, i) => {
        return {
            name: hit[1],
            return: wrappers[i][1],
            params: wrappers[i][2],
        }
    }).sort((a, b) => a.name.localeCompare(b.name))

    const paramTranslation = {
        'i32': 'int32_t',
        'u32': 'uint32_t',
        'usize': 'int32_t',
        'i64': 'int64_t',
        '*const u8': 'uint8_t const*',
        '*mut u8': 'uint8_t*',
    }

    function translateParamType(param) {
        if (param in paramTranslation) {
            return paramTranslation[param]
        }
        console.error(`Unknown parameter type: ${param}`)
        process.exit(1)
    }

    function checkHits(fileTitle, rustHostFunctions) {
        if (!areListsEqual(rustHostFunctions.map(f => f.name), cppHostFunctions.map(f => f.name))) {
            console.error('Rust Host Functions and C++ Host Functions do not match!')
            const rustMissing = cppHostFunctions.filter(f => !rustHostFunctions.some(rf => rf.name === f.name))
            const cppMissing = rustHostFunctions.filter(f => !cppHostFunctions.some(rf => rf.name === f.name))
            if (rustMissing.length > 0)
                console.error('Missing Rust Host Functions:', '\x1b[31m' + rustMissing.map(f => f.name).join(', ') + '\x1b[0m')
            if (cppMissing.length > 0)
                console.error('Missing C++ Host Functions:', '\x1b[31m' + cppMissing.map(f => f.name).join(', ') + '\x1b[0m')
            process.exit(1)
        }

        let hasError = false
        rustHostFunctions.forEach((hit, index) => {
            const cppHit = cppHostFunctions[index]
            if (hit.name !== cppHit.name) {
                console.error(
                    `Rust Host Function name mismatch in ${fileTitle}: ${hit.name} !== ${cppHit.name}`,
                )
                hasError = true
            } else if (hit.return !== cppHit.return) {
                console.error(
                    `Rust Host Function return type mismatch in ${fileTitle} for ${hit.name}: ${hit.return} !== ${cppHit.return}`,
                )
                hasError = true
            } else if (hit.params.length !== cppHit.params.length) {
                console.error(
                    `Rust Host Function parameter count mismatch in ${fileTitle} for ${hit.name}: ${hit.params.length} !== ${cppHit.params.length} (${hit.params.join(', ')}) !== (${cppHit.params.join(', ')})`,
                )
                hasError = true
            } else {
                hit.params.forEach((param, paramIndex) => {
                    if (param !== cppHit.params[paramIndex]) {
                        console.error(
                            `Rust Host Function parameter type mismatch in ${fileTitle} for ${hit.name}, parameter ${paramIndex}: ${param} !== ${cppHit.params[paramIndex]}`,
                        )
                        hasError = true
                    }
                })
            }
        })
        if (hasError) {
            process.exit(1)
        }
    }

    let rustHits = [
        ...rustHostFunctionFile.matchAll(
            // parse the Rust host functions in `host_bindings.rs`
            /^ *pub fn ([A-Za-z0-9_]+)\([ \n]*([A-Za-z0-9_:*, \n]*)\) -> ([A-Za-z0-9]+);$/gm,
        ),
    ]
    const rustFuncs = rustHits.map((hit) => [hit[1], hit[3], hit[2].trim().split(',').map((s) => s.trim()).filter((s) => s.length > 0).map((s) => s.split(':')[1].trim())])
    const rustHostFunctions = rustFuncs.map((hit) => {
        return {
            name: hit[0],
            return: translateParamType(hit[1]),
            params: hit[2].map(translateParamType),
        }
    }).sort((a, b) => a.name.localeCompare(b.name))
    checkHits("host_bindings.rs", rustHostFunctions)

    let rustTestHits = [
        ...rustHostFunctionTestFile.matchAll(
            // parse the Rust host functions in `host_bindings_for_testing.rs`
            /^ *pub (unsafe )?fn ([A-Za-z0-9_]+)\([ \n]*([A-Za-z0-9_:*, \n]*)\) -> ([A-Za-z0-9]+)/gm,
        ),
    ]
    const rustTestFuncs = rustTestHits.map((hit) => [hit[2], hit[4], hit[3].trim().split(',').map((s) => s.trim()).filter((s) => s.length > 0).map((s) => s.split(':')[1].trim())])
    const rustTestHostFunctions = rustTestFuncs.map((hit) => {
        return {
            name: hit[0],
            return: translateParamType(hit[1]),
            params: hit[2].map(translateParamType),
        }
    }).sort((a, b) => a.name.localeCompare(b.name))
    checkHits("host_bindings_for_testing.rs", rustTestHostFunctions)


    console.log('All host functions match between Rust and C++ implementations.')
}

main()
