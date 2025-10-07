# xrpl-address-macro

A compile-time macro for converting XRPL classic addresses (r-addresses) to 20-byte arrays.

This is an internal procedural macro crate used by `xrpl-wasm-std`. Users should import the macro from `xrpl-wasm-std`
directly.

## Features

- **Zero runtime overhead**: Address decoding happens at compile time
- **Type safe**: Invalid addresses cause compilation errors
- **No binary bloat**: The final WASM contains only the raw 20-byte array, no decoding logic
- **no-std compatible**: The macro runs at compile time on the host, so its dependencies never affect the target
  environment

## no-std Compatibility

**Important**: Procedural macros run at compile time on your development machine, NOT in the target environment. This
means:

- The macro's dependencies (`bs58`, `sha2`, `syn`, `quote`) run during compilation only
- These dependencies are NEVER included in your final WASM binary
- The `xrpl-wasm-std` library remains fully `no-std` compatible
- The macro only outputs a simple `[u8; 20]` array literal in your code

For example, this code:

```rust
const ACCOUNT: [u8; 20] = r_address!("rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh");
```

Gets expanded at compile time to:

```rust
const ACCOUNT: [u8; 20] = [132, 45, 67, 89, ...]; // actual 20 bytes
```

No runtime code from the macro or its dependencies exists in the final binary.

## Usage

```rust
use xrpl_wasm_std::r_address;

// Convert r-address to [u8; 20] at compile time
const ACCOUNT: [u8; 20] = r_address!("rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh");

// Multiple accounts can be defined
const NOTARY: [u8; 20] = r_address!("rN7n7otQDd6FczFgLdSqtcsAUxDkw6fzRH");
const ADMIN: [u8; 20] = r_address!("rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn");
```

## Why Use This Macro?

This macro provides a clean, compile-time solution for embedding XRPL addresses in smart contracts:

- **Simple**: Just use `r_address!("r...")` directly in your code
- **Safe**: Invalid addresses are caught at compile time
- **Efficient**: No runtime overhead or extra dependencies in the final WASM

