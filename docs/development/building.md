# Building and Development

Complete guide for building, testing, and developing with xrpl-wasm-std.

## Quick Setup

### 1. Install Dependencies

```shell
# Run the setup script (recommended)
./scripts/setup.sh

# Or manually install:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32v1-none
npm install
```

### 2. Verify Installation

```shell
# Check Rust and target
rustc --version
rustup target list --installed | grep wasm32v1-none

# Check Node.js
node --version
npm --version
```

## Build System

### Individual Project Builds

```shell
# Debug build
cargo build --target wasm32v1-none

# Release build (optimized for size)
cargo build --target wasm32v1-none --release

# Build specific example
cd examples/smart-escrows/oracle
cargo build --target wasm32v1-none --release
```

### Workspace Builds

```shell
# Build all examples
./scripts/build.sh

# Build all examples in release mode
./scripts/build.sh release

# Build and test everything
./scripts/build-and-test.sh
```

### Build Configuration

Recommended `Cargo.toml` for smart escrows:

```toml
[package]
name = "my-escrow"
version = "0.1.0"
edition = "2021"

[dependencies]
xrpl-wasm-std = { path = "../../../xrpl-wasm-std" }

[lib]
crate-type = ["cdylib"]

[profile.release]
opt-level = "s"        # Optimize for size
lto = true            # Link-time optimization
panic = "abort"       # Reduce binary size
codegen-units = 1     # Better optimization
```

### Build Targets

| Target    | Description                  | Use Case               |
| --------- | ---------------------------- | ---------------------- |
| `debug`   | Unoptimized, with debug info | Development, debugging |
| `release` | Size-optimized               | Production deployment  |

## Testing System

### Automated Testing

```shell
# Run all tests (equivalent to CI)
./scripts/run-all.sh

# Run integration tests only
./scripts/run-tests.sh

# Test specific example
./scripts/run-tests.sh examples/smart-escrows/oracle

# Test with environment variables
CI=1 ./scripts/run-tests.sh examples/smart-escrows/hello_world
```

### Manual Testing

#### Using the Web UI

1. **Start the UI**:

   ```shell
   # Open in browser
   open ui/index.html

   # Or serve locally
   cd ui && python3 -m http.server 8000
   ```

2. **Update embedded WASM** (after building):
   ```shell
   ./ui/embed-wasm.sh
   ```

#### Using Node.js Scripts

```shell
# Run individual test
cd examples/smart-escrows/oracle
node run_test.js

# Run with custom parameters
RIPPLED_HOST=localhost RIPPLED_PORT=6006 node run_test.js
```

### Test Configuration

#### Environment Variables

| Variable       | Default      | Description                       |
| -------------- | ------------ | --------------------------------- |
| `CI`           | `false`      | Enable CI mode (stricter testing) |
| `RIPPLED_HOST` | WASM Devnet  | Custom rippled host               |
| `RIPPLED_PORT` | `51233`      | Custom rippled port               |
| `RUSTFLAGS`    | `-Dwarnings` | Rust compilation flags            |

#### Test Networks

| Network         | Endpoint                                 | Purpose             |
| --------------- | ---------------------------------------- | ------------------- |
| **WASM Devnet** | `wss://wasm.devnet.rippletest.net:51233` | Integration testing |
| **Local Node**  | `ws://localhost:6006`                    | Development         |
| **Testnet**     | `wss://s.altnet.rippletest.net:51233`    | Staging             |

## Code Quality

### Linting and Formatting

```shell
# Check code formatting
./scripts/fmt.sh

# Run Clippy linting
./scripts/clippy.sh

# Fix formatting automatically
cargo fmt

# Fix Clippy warnings automatically
cargo clippy --fix --allow-dirty --allow-staged
```

### Pre-commit Hooks

```shell
# Install pre-commit hooks
pre-commit install

# Run hooks manually
pre-commit run --all-files
```

## Development Workflow

### 1. Project Structure

```text
my-project/
├── Cargo.toml           # Package configuration
├── src/
│   └── lib.rs          # Main contract code
├── README.md           # Documentation
├── run_test.js         # Integration test
└── target/
    └── wasm32v1-none/
        └── release/
            └── my_project.wasm
```

### 2. Development Cycle

```shell
# 1. Write code
vim src/lib.rs

# 2. Check formatting and linting
./scripts/fmt.sh && ./scripts/clippy.sh

# 3. Build
cargo build --target wasm32v1-none --release

# 4. Test
./scripts/run-tests.sh examples/my-project

# 5. Debug if needed
./ui/embed-wasm.sh && open ui/index.html
```

### 3. Debugging

#### Build Debugging

```shell
# Verbose build output
cargo build --target wasm32v1-none --release --verbose

# Check WASM exports
./scripts/check-wasm-exports.sh target/wasm32v1-none/release/my_project.wasm

# Inspect WASM binary
wasm-objdump -x target/wasm32v1-none/release/my_project.wasm
```

#### Runtime Debugging

```rust
// Add trace statements
use xrpl_wasm_std::host::trace::{trace, trace_data, DataRepr};

#[no_mangle]
pub extern "C" fn finish() -> i32 {
    trace("Starting execution")?;

    let account = match EscrowFinish.get_account() {
        Ok(acc) => {
            trace_data("Account", &acc, DataRepr::AsHex)?;
            acc
        },
        Err(e) => {
            trace(&format!("Error getting account: {:?}", e))?;
            return 0;
        }
    };

    // Rest of logic...
}
```

## Performance Optimization

### Binary Size Optimization

```toml
# Cargo.toml optimizations
[profile.release]
opt-level = "s"           # Optimize for size
lto = true               # Link-time optimization
panic = "abort"          # Remove panic handling
codegen-units = 1        # Single codegen unit
strip = true             # Strip debug symbols
```

### Runtime Optimization

```rust
// Minimize host function calls
let account = tx.get_account()?;  // Call once
let balance = get_account_balance(&account)?;  // Use cached account

// Use fixed-size buffers
let mut buffer = [0u8; 32];  // Stack allocation
let len = get_field(sfield::Hash, 0, &mut buffer)?;

// Efficient error handling
match operation() {
    Ok(result) => result,
    Err(_) => return 0,  // Fast path for errors
}
```

### Memory Usage

```rust
// Good: Stack-based allocation
let mut accounts = [AccountID::default(); 10];

// Bad: Would require heap (not available in no_std)
// let mut accounts = Vec::new();

// Cache expensive operations
let slot = cache_ledger_obj(&keylet)?;
let field1 = get_field_from_slot(slot, sfield::Field1)?;
let field2 = get_field_from_slot(slot, sfield::Field2)?;
```

## CI/CD Integration

### GitHub Actions

The project includes GitHub Actions that mirror the local scripts:

```yaml
# .github/workflows/test.yml
- name: Setup environment
  run: ./scripts/setup.sh

- name: Run all tests
  run: ./scripts/run-all.sh
```

### Local CI Simulation

```shell
# Run the same checks as CI
./scripts/run-all.sh

# Individual CI steps
./scripts/setup.sh
./scripts/fmt.sh
./scripts/clippy.sh
./scripts/build-and-test.sh
./scripts/run-tests.sh
```

## Advanced Configuration

### Custom Build Scripts

Create custom build scripts for complex workflows:

```shell
#!/bin/bash
# scripts/custom-build.sh

set -euo pipefail

echo "Building custom configuration..."

# Custom environment variables
export RUSTFLAGS="-C target-feature=+simd128"

# Build with custom features
cargo build --target wasm32v1-none --release --features "custom-feature"

# Post-process WASM
wasm-opt -O4 target/wasm32v1-none/release/my_project.wasm \
  -o target/wasm32v1-none/release/my_project_optimized.wasm

echo "Custom build complete"
```

### Development Dependencies

```toml
# Cargo.toml
[dev-dependencies]
# Add development-only dependencies here
# Note: These won't be included in WASM builds
```

### Feature Flags

```toml
# Cargo.toml
[features]
default = []
debug-traces = []
testing = []

# Enable conditional compilation
[dependencies]
xrpl-wasm-std = { path = "../xrpl-wasm-std", features = ["testing"] }
```

```rust
// Conditional compilation
#[cfg(feature = "debug-traces")]
trace("Debug mode enabled")?;

#[cfg(feature = "testing")]
fn test_helper() -> bool {
    // Testing-only code
    true
}
```

## Troubleshooting

### Common Build Issues

| Issue                            | Solution                                      |
| -------------------------------- | --------------------------------------------- |
| `wasm32v1-none` target not found | `rustup target add wasm32v1-none`             |
| Link errors                      | Check `crate-type = ["cdylib"]` in Cargo.toml |
| Size too large                   | Use release profile optimizations             |
| Missing exports                  | Ensure `#[no_mangle]` on `finish()` function  |

### Common Runtime Issues

| Issue                    | Solution                                        |
| ------------------------ | ----------------------------------------------- |
| Function not found       | Check WASM exports with `check-wasm-exports.sh` |
| Memory access violation  | Verify buffer sizes and bounds checking         |
| Cache full (NoFreeSlots) | Minimize ledger object caching                  |
| Field not found          | Handle optional fields appropriately            |

### Development Environment Issues

| Issue                     | Solution                                     |
| ------------------------- | -------------------------------------------- |
| Node.js tests fail        | `npm install` and check Node.js version      |
| Pre-commit hooks fail     | `pre-commit install` and fix reported issues |
| Permission denied         | `chmod +x scripts/*.sh`                      |
| Network connection failed | Check firewall and network connectivity      |

## Next Steps

- **[Testing Guide](testing.md)** - Comprehensive testing strategies
- **[API Reference](../api-reference.md)** - Complete API documentation
- **[Examples](../examples/README.md)** - Study working examples
- **[Contributing Guide](contributing.md)** - How to contribute to the project

## Documentation Website

The documentation is automatically built and deployed to GitHub Pages. You can also build it locally:

```shell
# Build the complete documentation website
cargo +nightly doc --no-deps --workspace

# Open the generated documentation
open target/doc/xrpl_wasm_std/index.html
```

The generated documentation includes:

- **Main README** - Overview and quick start (at the crate level)
- **API Documentation** - Generated from Rust code comments
- **Guides Module** - All markdown documentation embedded as submodules:
  - `guides::getting_started` - Setup and first contract tutorial
  - `guides::api_reference` - Complete API documentation
  - `guides::examples` - Smart escrow examples and tutorials
  - `guides::building` - This build guide
  - `guides::testing` - Testing strategies
  - `guides::contributing` - Contribution guidelines
