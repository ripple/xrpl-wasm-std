# XRPL WebAssembly Standard Library Documentation

Complete documentation for the xrpl-wasm-std library - the standard library for building smart escrows on the XRP Ledger.

## Documentation Structure

### Getting Started

| Document                                  | Description                                         | Audience       |
| ----------------------------------------- | --------------------------------------------------- | -------------- |
| **[Getting Started](getting-started.md)** | Complete setup guide, first contract, core concepts | Beginners      |
| **[API Reference](api-reference.md)**     | Complete API documentation and usage patterns       | All developers |

### Examples and Tutorials

| Document                                                           | Description                          | Audience       |
| ------------------------------------------------------------------ | ------------------------------------ | -------------- |
| **[Examples Overview](examples/README.md)**                        | Smart escrow examples with tutorials | All developers |
| **[Hello World](../examples/smart-escrows/hello_world/README.md)** | Your first smart escrow              | Beginners      |
| **[Oracle Example](../examples/smart-escrows/oracle/README.md)**   | Price-based conditional logic        | Intermediate   |
| **[KYC Example](../examples/smart-escrows/kyc/README.md)**         | Credential-based verification        | Intermediate   |

### Development and Testing

| Document                                              | Description                       | Audience     |
| ----------------------------------------------------- | --------------------------------- | ------------ |
| **[Building Guide](development/building.md)**         | Build system, optimization, CI/CD | Developers   |
| **[Testing Guide](development/testing.md)**           | Comprehensive testing strategies  | Developers   |
| **[Contributing Guide](development/contributing.md)** | How to contribute to the project  | Contributors |

### Reference Documentation

| Document                                                    | Description                 | Audience       |
| ----------------------------------------------------------- | --------------------------- | -------------- |
| **[Rust API Docs](../target/doc/xrpl_wasm_std/index.html)** | Generated API documentation | All developers |
| **[Scripts Documentation](../scripts/README.md)**           | Build and test scripts      | Developers     |
| **[UI Documentation](../ui/README.md)**                     | Testing interface           | All users      |

## Quick Navigation

### By Use Case

**I want to...**

- **Learn the basics** → [Getting Started](getting-started.md) → [Hello World Example](../examples/smart-escrows/hello_world/README.md)
- **Build price-based escrows** → [Oracle Example](../examples/smart-escrows/oracle/README.md) → [API Reference](api-reference.md#keylet-generation)
- **Implement compliance** → [KYC Example](../examples/smart-escrows/kyc/README.md) → [Credentials API](api-reference.md#keylet-generation)
- **Set up development** → [Building Guide](development/building.md) → [Testing Guide](development/testing.md)
- **Contribute code** → [Contributing Guide](development/contributing.md) → [Development Workflow](development/building.md#development-workflow)

### By Experience Level

**Beginner**

1. [Getting Started](getting-started.md) - Setup and first contract
2. [Hello World](../examples/smart-escrows/hello_world/README.md) - Simplest example
3. [Building Guide](development/building.md) - Development environment
4. [Examples Overview](examples/README.md) - Study more examples

**Intermediate**

1. [API Reference](api-reference.md) - Complete API documentation
2. [Oracle Example](../examples/smart-escrows/oracle/README.md) - Data integration
3. [KYC Example](../examples/smart-escrows/kyc/README.md) - Compliance patterns
4. [Testing Guide](development/testing.md) - Testing strategies

**Advanced**

1. [Contributing Guide](development/contributing.md) - Project contribution
2. [Rust API Docs](../target/doc/xrpl_wasm_std/index.html) - Low-level APIs
3. All examples for patterns and best practices
4. [Smart Escrows XLS Proposal](https://github.com/XRPLF/XRPL-Standards/discussions/270) - Specification

## Key Concepts

### Smart Escrows

Smart escrows are WebAssembly modules that implement conditional logic for XRPL Escrow objects:

- **Entry Point**: `finish()` function returns 1 (release) or 0 (keep locked)
- **Read-Only**: Access ledger data but can't modify (except escrow data field)
- **Deterministic**: Same inputs must produce same outputs across all validators
- **Resource-Limited**: Bounded by computation and memory allowances

### Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                        XRP Ledger                               │
│                                                                 │
│  ┌─────────────────┐              ┌─────────────────────────┐   │
│  │   EscrowFinish  │              │    Smart Escrow         │   │
│  │   Transaction   │              │    (WASM Module)        │   │
│  │                 │              │                         │   │
│  │ ┌─────────────┐ │              │ finish() function:      │   │
│  │ │ FinishFunc  │ │─────────────►│                         │   │
│  │ │ (WASM)      │ │              │ • Read transaction      │   │
│  │ │             │ │              │ • Access ledger         │   │
│  │ └─────────────┘ │              │ • Evaluate conditions   │   │
│  └─────────────────┘              │ • Return decision       │   │
│                                   │   (1=release, 0=lock)   │   │
│                                   └─────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

### Development Workflow

1. **Setup Environment** - Install Rust, WASM target, Node.js
2. **Choose Template** - Start with appropriate example
3. **Implement Logic** - Write conditional escrow logic
4. **Build Contract** - Compile to optimized WASM
5. **Test Locally** - Use integration tests and UI
6. **Deploy to Testnet** - Test on WASM Devnet
7. **Deploy to Mainnet** - Production deployment

## Common Patterns

### Error Handling

```rust
// Always handle errors gracefully
let account = match tx.get_account() {
    Ok(acc) => acc,
    Err(_) => return 0, // Safe default: keep locked
};
```

### Data Access

```rust
// Cache ledger objects for multiple field access
let slot = cache_ledger_obj(&keylet.data)?;
let field1 = get_field_from_slot(slot, sfield::Field1)?;
let field2 = get_field_from_slot(slot, sfield::Field2)?;
```

### Debugging

```rust
// Use trace functions for debugging
use xrpl_wasm_std::host::trace::{trace, trace_data, DataRepr};

trace("Starting validation")?;
trace_data("Account", &account, DataRepr::AsHex)?;
```

## Integration Points

### XRPL Features

Smart escrows integrate with various XRPL features:

- **Accounts** - Balance checks, account validation
- **Escrows** - Conditional release logic
- **Oracles** - External data integration
- **Credentials** - Identity and compliance verification
- **NFTs** - Asset ownership verification
- **Time** - Ledger sequence and timestamp access

### Host Functions

The library provides safe access to XRPL host functions:

- **Transaction Access** - Read current transaction fields
- **Ledger Access** - Query ledger objects and state
- **Cryptography** - Hashing and signature verification
- **Keylets** - Generate unique object identifiers
- **Tracing** - Debug output and logging

## Getting Help

### Documentation Issues

- **Missing information** → [Open an issue](https://github.com/ripple/xrpl-wasm-std/issues)
- **Incorrect examples** → [Submit a fix](development/contributing.md)
- **Unclear explanations** → [Suggest improvements](development/contributing.md)

### Development Issues

- **Build problems** → [Building Guide](development/building.md#troubleshooting)
- **Test failures** → [Testing Guide](development/testing.md#troubleshooting)
- **Runtime errors** → [API Reference](api-reference.md#error-handling)

### Community Resources

- **GitHub Issues** - Bug reports and feature requests
- **GitHub Discussions** - Questions and design discussions
- **Examples** - Working code for common patterns
- **XRPL Developer Discord** - Community support

## Contributing to Documentation

We welcome documentation improvements:

1. **Fix typos and errors** - Submit PRs for quick fixes
2. **Add examples** - Create new example contracts with documentation
3. **Improve explanations** - Make concepts clearer for newcomers
4. **Expand coverage** - Document undocumented features

See [Contributing Guide](development/contributing.md) for details.

## License

All documentation is licensed under the same terms as the xrpl-wasm-std library - either Apache License 2.0 or MIT License at your option.
