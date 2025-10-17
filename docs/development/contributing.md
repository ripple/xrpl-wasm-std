# Contributing Guide

Guidelines for contributing to the xrpl-wasm-std project.

## Quick Start

1. **Fork and clone** the repository
2. **Setup development environment**: `./scripts/setup.sh`
3. **Create a feature branch**: `git checkout -b feature/my-feature`
4. **Make changes** and add tests
5. **Run tests**: `./scripts/run-all.sh`
6. **Submit a pull request**

## Development Setup

### Prerequisites

- **Rust** (latest stable)
- **Node.js** 18+
- **Git**

### Setup

```shell
# Clone your fork
git clone https://github.com/YOUR_USERNAME/xrpl-wasm-std.git
cd xrpl-wasm-std

# Setup development environment
./scripts/setup.sh

# Verify setup
./scripts/run-all.sh
```

## Contribution Areas

### 1. Core Library

**Location**: `xrpl-wasm-std/src/`

- **Host function bindings** - New XRPL host functions
- **Type definitions** - XRPL data types and structures
- **API improvements** - Better ergonomics and safety
- **Performance optimizations** - Reduce binary size and improve speed

### 2. Examples

**Location**: `examples/smart-escrows/`

- **New use cases** - Additional smart escrow patterns
- **Documentation improvements** - Better examples and explanations
- **Test coverage** - More comprehensive testing

### 3. Testing Infrastructure

**Location**: `e2e-tests/`, `scripts/`, `ui/`

- **Testing tools** - Better testing utilities
- **CI improvements** - Enhanced automation
- **UI enhancements** - Better developer experience

### 4. Documentation

**Location**: `docs/`, `README.md`

- **API documentation** - Better explanations and examples
- **Tutorials** - Step-by-step guides
- **Best practices** - Patterns and recommendations

## Code Standards

### Rust Code

1. **Follow Rust conventions**:

   ```shell
   # Format code
   cargo fmt

   # Check linting
   cargo clippy -- -D warnings
   ```

2. **Write safe code**:

   ```rust
   // Good: Handle errors explicitly
   match get_account_balance(&account) {
       Ok(balance) => balance,
       Err(_) => return 0,
   }

   // Bad: Use unwrap()
   let balance = get_account_balance(&account).unwrap();
   ```

3. **Document public APIs**:
   ````rust
   /// Gets the current escrow object being processed
   ///
   /// # Returns
   ///
   /// A `CurrentEscrow` instance for accessing escrow fields
   ///
   /// # Example
   ///
   /// ```rust
   /// let escrow = get_current_escrow();
   /// let amount = escrow.get_amount()?;
   /// ```
   pub fn get_current_escrow() -> CurrentEscrow {
       CurrentEscrow::new()
   }
   ````

### JavaScript/Node.js Code

1. **Use modern JavaScript**:

   ```javascript
   // Good: Use async/await
   async function testEscrow() {
     const result = await client.submitAndWait(tx, { wallet })
     return result
   }

   // Good: Use const/let appropriately
   const client = new xrpl.Client(WSS_URL)
   let retryCount = 0
   ```

2. **Handle errors properly**:
   ```javascript
   try {
     const result = await riskyOperation()
     return result
   } catch (error) {
     console.error("Operation failed:", error)
     throw error
   }
   ```

### Documentation

1. **Use clear, concise language**
2. **Include code examples**
3. **Provide context and rationale**
4. **Keep examples up to date**

## Testing Requirements

### All Contributions Must Include

1. **Passing tests**:

   ```shell
   ./scripts/run-all.sh
   ```

2. **New tests for new functionality**:
   - Unit tests for library functions
   - Integration tests for examples
   - Documentation examples that work

3. **No breaking changes** without justification

### Test Categories

| Change Type       | Required Tests                                        |
| ----------------- | ----------------------------------------------------- |
| **New API**       | Unit tests + integration test + documentation example |
| **Bug fix**       | Regression test + existing tests pass                 |
| **Example**       | Integration test + README with usage                  |
| **Documentation** | Examples compile and run                              |

## Pull Request Process

### 1. Before Submitting

- [ ] All tests pass locally: `./scripts/run-all.sh`
- [ ] Code is formatted: `./scripts/fmt.sh`
- [ ] No linting errors: `./scripts/clippy.sh`
- [ ] Documentation is updated
- [ ] Examples are tested

### 2. PR Description

Include:

- **What** - Brief description of changes
- **Why** - Motivation and use case
- **How** - Implementation approach
- **Testing** - How you tested the changes

Example:

```markdown
## Add Oracle Price Aggregation Example

### What

Adds a new smart escrow example that aggregates prices from multiple oracles.

### Why

Demonstrates how to build more robust price-based escrows by reducing single points of failure.

### How

- Queries multiple oracle objects
- Calculates median price
- Includes fallback logic for missing oracles

### Testing

- Integration test with 3 mock oracles
- Edge case testing with missing oracles
- Performance testing with 10 oracles
```

### 3. Review Process

1. **Automated checks** run on all PRs
2. **Maintainer review** for correctness and design
3. **Community feedback** for significant changes
4. **Approval and merge** by maintainers

## Specific Contribution Guidelines

### Adding New Examples

1. **Create example directory**:

   ```shell
   mkdir examples/smart-escrows/my-example
   cd examples/smart-escrows/my-example
   ```

2. **Required files**:
   - `Cargo.toml` - Package configuration
   - `src/lib.rs` - Main contract code
   - `README.md` - Documentation (use oracle example as template)
   - `run_test.js` - Integration test

3. **Example template**:

   ```rust
   #![no_std]
   #![no_main]

   use xrpl_wasm_std::core::current_tx::escrow_finish::EscrowFinish;

   #[no_mangle]
   pub extern "C" fn finish() -> i32 {
       // Your logic here
       1 // or 0
   }
   ```

4. **Documentation requirements**:
   - Clear use case description
   - Architecture overview
   - Build and test instructions
   - Configuration details
   - Security considerations

### Adding Library Features

1. **Design discussion** - Open an issue first for significant changes
2. **Backward compatibility** - Maintain existing APIs
3. **Performance impact** - Consider binary size and runtime cost
4. **Safety** - Ensure memory safety and deterministic behavior

### Host Function Bindings

When adding new host functions:

1. **Raw binding** in `host_bindings.rs`:

   ```rust
   extern "C" {
       fn new_host_function(param: i32) -> i32;
   }
   ```

2. **Safe wrapper** in appropriate module:

   ```rust
   pub fn safe_new_function(param: u32) -> Result<u32, Error> {
       let result = unsafe { new_host_function(param as i32) };
       match_result_code(result)
   }
   ```

3. **High-level API** if appropriate:

   ```rust
   impl SomeObject {
       pub fn get_new_field(&self) -> Result<SomeType, Error> {
           let raw_value = safe_new_function(self.slot)?;
           Ok(SomeType::from_raw(raw_value))
       }
   }
   ```

4. **Tests and documentation**

## Issue Reporting

### Bug Reports

Include:

- **Environment** - OS, Rust version, Node.js version
- **Steps to reproduce** - Minimal example
- **Expected behavior** - What should happen
- **Actual behavior** - What actually happens
- **Logs/traces** - Error messages and trace output

### Feature Requests

Include:

- **Use case** - Why is this needed?
- **Proposed solution** - How should it work?
- **Alternatives** - Other approaches considered
- **Impact** - Breaking changes, performance implications

## Code Review Checklist

### For Reviewers

- [ ] **Functionality** - Does it work as intended?
- [ ] **Safety** - No memory safety issues?
- [ ] **Performance** - Reasonable binary size and speed?
- [ ] **API design** - Consistent with existing patterns?
- [ ] **Tests** - Adequate test coverage?
- [ ] **Documentation** - Clear and complete?
- [ ] **Backward compatibility** - No breaking changes?

### For Contributors

Before requesting review:

- [ ] **Self-review** - Review your own changes
- [ ] **Test thoroughly** - All tests pass, edge cases covered
- [ ] **Update documentation** - README, API docs, examples
- [ ] **Clean commits** - Logical, well-described commits
- [ ] **No debug code** - Remove temporary debugging code

## Release Process

### Versioning

Following [SemVer](https://semver.org/):

- **MAJOR** - Breaking changes
- **MINOR** - New features, backward compatible
- **PATCH** - Bug fixes, backward compatible

### Release Checklist

1. **Update version** in `Cargo.toml`
2. **Update CHANGELOG** with changes
3. **Tag release** with version number
4. **Update documentation** if needed
5. **Announce** release with highlights

## Getting Help

### Resources

- **[API Reference](../api-reference.md)** - Complete API documentation
- **[Examples](../examples/README.md)** - Working code examples
- **[Building Guide](building.md)** - Development setup
- **[Testing Guide](testing.md)** - Testing strategies

### Community

- **GitHub Issues** - Bug reports and feature requests
- **GitHub Discussions** - Questions and design discussions
- **Pull Requests** - Code contributions and reviews

### Maintainers

Current maintainers are available for:

- **Design discussions** - Major changes and new features
- **Technical questions** - Implementation guidance
- **Code review** - Feedback on contributions

## Recognition

Contributors are recognized in:

- **Git history** - All commits are attributed
- **Release notes** - Contributors listed for each release
- **Documentation** - Major contributors acknowledged

Thank you for contributing to xrpl-wasm-std! 🚀
