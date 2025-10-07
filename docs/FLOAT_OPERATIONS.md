# XRPL Float Operations Technical Reference

## Overview

Float operations in XRPL are used exclusively for **fungible token (IOU)** amounts.

Computations use rippled's Number class via FFI for exact consensus compatibility.


## XRPL Amount Types

The XRPL Amount type (STAmount) can represent three different types of assets:

### 1. XRP
- 64-bit unsigned integer (big-endian)
- Most significant bit: always 0
- Second bit: 1 (positive indicator)
- Third bit: 0 (not an MPT)
- Remaining 61 bits: quantity in drops
- Standard format: 64-bit unsigned integer OR'd with `0x4000000000000000`

### 2. Fungible Tokens (IOUs)
- **First 64 bits**: Amount in custom float format (detailed below)
- **Next 160 bits**: Currency code
- **Last 160 bits**: Issuer's Account ID
- Total: 384 bits (48 bytes)

### 3. Multi-Purpose Tokens (MPTs)
- **First 8 bits**: `0x60` (indicates MPT)
- **Next 64 bits**: Quantity as unsigned integer
- **Last 192 bits**: MPT Issuance ID

You can identify the amount type by examining the first and third bits:
- First bit = 1: Fungible token (IOU)
- First bit = 0, third bit = 0: XRP
- First bit = 0, third bit = 1: MPT

## Current Architecture

### Number Implementation (via rippled Number)

As of PR #139, float/IOU computations are performed by the rippled Number class, wrapped by the xrpld-number crate and invoked from host functions.

```text
Current flow:
WASM Module -> Host Function -> xrpld-number (rippled Number via FFI) -> Result
```

**Characteristics:**
- Deterministic decimal arithmetic matching rippled semantics
- Explicit rounding modes (ToNearest, TowardsZero, Downward, Upward)
- Bit/byte compatibility with XRPL serialization for IOU values
- Better performance than BigDecimal (with minor FFI overhead)

### Notes and caveats

1. **Rounding Modes**: The rounding_mode parameter is honored; host functions set Number's thread-local rounding mode per call.
2. **Compatibility**: Designed to match rippled exactly; please report any divergences found in tests.
3. **Range/Normalization**: IOU format normalization and exponent range [-96, +80] are enforced.
4. **FFI boundary**: Calls cross the Rust<->C++ FFI; keep buffers valid and sizes correct.

## Fungible Token Float Format

See: <https://xrpl.org/docs/references/protocol/binary-format#amount-fields>

XRPL uses a custom 64-bit floating-point encoding for fungible token amounts:

```text
Bit Layout:
[T][S][EEEEEEEE][MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM]
 │  │  └8 bits┘  └──────────────────54 bits───────────────────────────┘
 │  └─ Sign (1=positive, 0=negative)
 └─ Type (1=fungible token, 0=XRP/MPT)
```

### Encoding Details

1. **Type bit**: Always 1 for fungible tokens
2. **Sign bit**: 1 = positive, 0 = negative
3. **Exponent**: 8 bits, biased by 97
   - Actual exponent = stored value - 97
   - Valid range when normalized: -96 to +80 (inclusive)
4. **Mantissa**: 54 bits of precision
   - Normalized to 16 decimal digits

### Special Values

- **Zero**: Special encoding `0x8000000000000000`
- **Maximum**: ~9.999999999999999 × 10^80
- **Minimum positive**: ~1.0 × 10^-81
- **Precision**: 16 significant decimal digits

## Available Operations

### Creation Functions

```rust,ignore
// Create from integer
float_from_int(value: i64, out: *mut u8, rounding_mode: i32) -> i32

// Create from unsigned integer
float_from_uint(value: *const u8, out: *mut u8, rounding_mode: i32) -> i32

// Create from exponent and mantissa
float_set(exponent: i32, mantissa: i64, out: *mut u8, rounding_mode: i32) -> i32
```

### Arithmetic Operations

```rust,ignore
// Addition: out = a + b
float_add(a: *const u8, b: *const u8, out: *mut u8, rounding_mode: i32) -> i32

// Subtraction: out = a - b
float_subtract(a: *const u8, b: *const u8, out: *mut u8, rounding_mode: i32) -> i32

// Multiplication: out = a × b
float_multiply(a: *const u8, b: *const u8, out: *mut u8, rounding_mode: i32) -> i32

// Division: out = a ÷ b
float_divide(a: *const u8, b: *const u8, out: *mut u8, rounding_mode: i32) -> i32
```

### Mathematical Functions

```rust,ignore
// Nth power: out = aⁿ
float_pow(a: *const u8, n: i32, out: *mut u8, rounding_mode: i32) -> i32

// Nth root: out = ⁿ√a
float_root(a: *const u8, n: i32, out: *mut u8, rounding_mode: i32) -> i32

// Base-10 logarithm: out = log₁₀(a)
float_log(a: *const u8, out: *mut u8, rounding_mode: i32) -> i32
```

### Comparison

```rust,ignore
// Compare two floats
// Returns: 0 (equal), 1 (a > b), 2 (a < b)
float_compare(a: *const u8, b: *const u8) -> i32
```


## Implementation Notes

- Host functions use rippled's Number class via the xrpld-number FFI wrapper
- Rounding modes are applied per-operation using thread-local state
- All operations maintain bit-exact compatibility with rippled consensus

## Rounding Modes

| Mode | Name | Description |
|------|------|-------------|
| 0 | ToNearest | Round to nearest, ties to even |
| 1 | TowardsZero | Truncate towards zero |
| 2 | Downward | Round towards -∞ |
| 3 | Upward | Round towards +∞ |

## See Also

- [XRPL Amount Fields Specification](https://xrpl.org/docs/references/protocol/binary-format#amount-fields)
- [rippled Number Implementation](https://github.com/XRPLF/rippled/blob/develop/src/ripple/basics/Number.h)
- xrpl-wasm-std documentation: `cargo doc --open -p xrpl-wasm-std`
