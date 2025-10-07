const path = require('path')
const fs = require('fs')
var execSync = require('child_process').execSync

if (process.argv.length < 4) {
    console.error('Usage: node tools/copyRippledFixture.js <project_name> <fixture_name> <rippled_path>')
    process.exit(1)
}

function main() {
    const projectName = process.argv[2]
    const projectPath = path.resolve(__dirname, `../rippled-tests/${projectName}`)
    execSync(`(cd ${projectPath} && cargo build --target wasm32v1-none --release && wasm-opt target/wasm32v1-none/release/${projectName}.wasm -Oz -o target/wasm32v1-none/release/${projectName}.wasm)`,
        function (error, stdout, stderr) {
            if (stderr) {
                console.error(`stderr: ${stderr}`)
            }
            console.log(`stdout: ${stdout}`)
            if (error) {
                console.error(`exec error: ${error}`)
                process.exit(1)
            }
            console.log(`WASM file for ${projectName} has been built and optimized.`)
        }
    )

    const srcPath = path.resolve(__dirname, `../rippled-tests/${projectName}/target/wasm32v1-none/release/${projectName}.wasm`)
    const data = fs.readFileSync(srcPath)
    const wasm = data.toString('hex')

    const fixtureName = process.argv[3]
    console.log(`Updating fixture: ${fixtureName}`)
    const rippledPath = process.argv[4]
    const dstPath = path.resolve(rippledPath, 'src/test/app/wasm_fixtures/fixtures.cpp')
    const dstContent = fs.readFileSync(dstPath, 'utf8')
    const re = new RegExp(String.raw`extern std::string const ${fixtureName} =[ \n]+"[^;]*;`, "g")
    const updatedContent = dstContent.replace(re, `extern std::string const ${fixtureName} = "${wasm}";`)
    fs.writeFileSync(dstPath, updatedContent)
}

main()
