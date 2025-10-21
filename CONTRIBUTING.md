# Contributing to XRPL WebAssembly Standard Library

## Quick Start

### Prerequisites

- **Rust toolchain** - [Install here](https://rust-lang.org/tools/install/)
- **Node.js** (for testing tools)
- **Basic Git/GitHub knowledge** - [Git Handbook](https://guides.github.com/introduction/git-handbook/)

### Setup

```shell
# Clone your fork and set up development environment
./scripts/setup.sh

# Verify installation
./scripts/run-tests.sh examples/smart-escrows/hello_world
```

## Development Workflow

### Running Formatting Checks

```shell
./scripts/fmt.sh && ./scripts/clippy.sh
```

### Test your changes

```
./scripts/run-all.sh
```

### Pull Request Requirements

**All PRs must:**

- Pass all existing tests (`./scripts/run-all.sh` and in CI)
- Follow general code style guidelines (enforced by CI)
- Include tests for new functionality
- Update documentation as needed

**For new examples:**

- Include comprehensive README with functionality description, build/test instructions, and code walkthrough
- Add integration test (`run_test.js`)
- Test on WASM devnet
- Add to main README examples list

**For library changes:**

- Consider backward compatibility
- Update API documentation and comprehensive guide
- Add unit tests where applicable
- Include performance considerations

## Testing

### Test Networks

| Network         | Endpoint                                 | Purpose             |
| --------------- | ---------------------------------------- | ------------------- |
| **WASM Devnet** | `wss://wasm.devnet.rippletest.net:51233` | Integration testing |
| **Local Node**  | `ws://localhost:6006`                    | Development         |

### Debugging and Development

**Web UI for manual testing:**

```shell
# Build your WASM contract
cargo build --target wasm32v1-none --release

# Upload to deployed testing interface
# Open: https://ripple.github.io/xrpl-wasm-std/ui/
# Click "Choose File" and select your .wasm file
```

**Using trace statements for debugging:**

These debugging statements will show up in the `debug.log` for rippled.

```rust
use xrpl_wasm_std::host::trace::{trace, trace_data, DataRepr};

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    trace("Contract starting").ok();

    let account = match EscrowFinish.get_account() {
        Ok(acc) => {
            trace_data("Account", &acc.as_bytes(), DataRepr::AsHex).ok();
            acc
        },
        Err(e) => {
            trace(&format!("Error: {:?}", e)).ok();
            return 0;
        }
    };

    // Rest of logic...
}
```

**Integration test template (`run_test.js`):**

```javascript
const CONFIG = {
  wasmPath: "./target/wasm32v1-none/release/my_example.wasm",
  rippledHost: process.env.RIPPLED_HOST || "wasm.devnet.rippletest.net",
  testAccount: "rN7n7otQDd6FczFgLdSqtcsAUxDkw6fzRH",
}

async function runTest() {
  // Set up test scenario
  // Execute contract with test data
  // Verify expected results
  console.log("Test passed!")
}

runTest().catch(console.error)
```

## Project Structure

```
xrpl-wasm-std/
├── src/                    # Library source code
├── examples/smart-escrows/ # Example smart contracts
├── scripts/                # Development and CI scripts
├── ui/                     # Testing web interface
├── e2e-tests/              # Integration tests
└── docs/                   # Documentation
```

## Adding New Examples

1. **Create directory:** `examples/smart-escrows/my-example/`

2. **Set up project structure:**

   ```shell
   # Use existing example as template
   cp -r examples/smart-escrows/hello_world examples/smart-escrows/my-example
   cd examples/smart-escrows/my-example
   ```

3. **Essential files:**
   - `Cargo.toml` - Package configuration with proper WASM settings
   - `src/lib.rs` - Contract implementation with `#![no_std]` and `#![no_main]`
   - `README.md` - Comprehensive documentation (see other examples for a template)
   - `run_test.js` - Integration test

4. **Test and integrate:**

   ```shell
   # Test your example
   ./scripts/run-tests.sh examples/smart-escrows/my-example

   # Add to main README examples list
   # Update comprehensive guide if significant
   ```

## Release Process (Maintainers)

```shell
# Update version and changelog
vim Cargo.toml CHANGELOG.md

# Full test suite
./scripts/run-all.sh

# Tag and release
git tag v0.x.y
git push origin v0.x.y
```

## Getting Help

- Check [Complete Developer Guide](https://ripple.github.io/xrpl-wasm-std/xrpl_wasm_std/guide/index.html)
- Search existing GitHub issues
- Create new issue with "question" label
- Reference related issues in PRs

## Community Guidelines

- Be respectful and constructive
- Help newcomers learn
- Focus on technical discussions
- Provide clear reproduction steps for bugs

Thank you for contributing to the XRPL WebAssembly Standard Library!
