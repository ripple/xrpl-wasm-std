# XRPL Field Access and Locators

This document explains how to access fields in XRPL transactions and ledger objects.

The locator system is used to navigate nested data structures. If you want to find a field in an XRPL data object, use locators.

## Table of Contents

- [Overview](#overview)
- [Field Types and Codes](#field-types-and-codes)
- [Simple Field Access](#simple-field-access)
- [Nested Field Access](#nested-field-access)
- [Understanding Locators](#understanding-locators)
- [JSON vs Internal Representation](#json-vs-internal-representation)
- [Common Patterns](#common-patterns)
- [Field Code Reference](#field-code-reference)
- [Troubleshooting](#troubleshooting)

## Overview

XRPL data structures can contain:
- **Simple fields**: Direct values (Account, Balance, Sequence)
- **Object fields**: Nested structures (Memo, Signer)
- **Array fields**: Lists of objects (Memos, Signers)

Accessing these fields involves:
1. Field codes (SFIELD constants)
2. Direct vs nested access patterns
3. The locator system for nested paths
4. Differences between JSON and internal representations

## Field Types and Codes

Every field has a unique field code that combines:
- **Type ID**: The data type (1-19)
- **Field ID**: The specific field (0-255)

```rust,ignore
// Field code calculation
const FIELD_CODE: i32 = (TYPE_ID << 16) | FIELD_ID;

// Examples
const SFIELD_ACCOUNT: i32 = 524289;      // (8 << 16) | 1
const SFIELD_BALANCE: i32 = 393218;      // (6 << 16) | 6
const SFIELD_MEMOS: i32 = 983049;        // (15 << 16) | 9

// Always use named constants from xrpl_wasm_std::sfield instead of raw integers
use xrpl_wasm_std::sfield;  // Provides sfield::Account, sfield::Balance, etc.
```

## Simple Field Access

For top-level fields, use direct access functions:

### Transaction Fields

```rust,ignore
use xrpl_wasm_std::host::get_tx_field;
use xrpl_wasm_std::sfield;

// Get account from transaction
let mut account_buf = [0u8; 20];
let len = unsafe {
    get_tx_field(
        sfield::Account,
        account_buf.as_mut_ptr(),
        account_buf.len()
    )
};

// Get fee amount
let mut fee_buf = [0u8; 8];
let len = unsafe {
    get_tx_field(
        sfield::Fee,
        fee_buf.as_mut_ptr(),
        fee_buf.len()
    )
};
```

### Ledger Object Fields

```rust,ignore
use xrpl_wasm_std::host::get_ledger_obj_field;

// First load object into cache
let slot = unsafe {
    cache_ledger_obj(
        keylet.as_ptr(),
        keylet.len(),
        0  // cache_num
    )
};

// Then access fields
let mut balance_buf = [0u8; 8];
let len = unsafe {
    get_ledger_obj_field(
        slot,
        sfield::Balance,
        balance_buf.as_mut_ptr(),
        balance_buf.len()
    )
};
```

## Nested Field Access

Complex objects require locators to navigate to nested fields. A locator is a byte array representing a packed array of (up to 16) i32 values.

Each i32 value can be an:

- SField: Every SField is a 32-bit integer that identifies a field in ledger entries and transactions.
- Array index: Some of the XRPL data types are arrays. The host can tell if the value is an array index by looking at its parent field.

### Building Locators

```rust,ignore
use xrpl_wasm_std::locator::Locator;

let mut locator = Locator::new();

// Add fields to the path
locator.pack(field1);
locator.pack(field2);
locator.pack(field3);

// Use the packed locator
let result = get_tx_nested_field(
    locator.get_addr(),
    locator.num_packed_bytes(),
    buffer.as_mut_ptr(),
    buffer.len()
)?;
```

### Locator Encoding Rules

1. **All fields**: 4 bytes each (stored as little-endian i32)
2. **Maximum depth**: 16 levels (64 bytes / 4 bytes per field)
3. **Maximum size**: 64 bytes
4. **Encoding**: Each field ID or array index is packed as a 4-byte integer

```rust,ignore
// Internal structure
pub struct Locator {
    buffer: [u8; 64],    // Packed locator data
    cur_buffer_index: usize,  // Current buffer index
}
```

## JSON vs Internal Representation

### Critical Difference

**JSON representation** includes wrapper objects for type safety:
```json
{
  "Memos": [
    {
      "Memo": {
        "MemoType": "test",
        "MemoData": "data"
      }
    }
  ]
}
```

**Internal representation** omits the wrapper:
```text
Memos[0].MemoType  // Direct access after array index
```

### Why This Matters

When building locators, use the **internal path**, not the JSON path:

```rust,ignore
// CORRECT: Internal representation
let mut locator = Locator::new();
locator.pack(sfield::Memos);     // Array field
locator.pack(0);                  // Array index
locator.pack(sfield::MemoType);   // Target field

// WRONG: Attempting to follow JSON structure
let mut locator = Locator::new();
locator.pack(sfield::Memos);     // Array field
locator.pack(0);                  // Array index
locator.pack(sfield::Memo);       // INCORRECT: wrapper
locator.pack(sfield::MemoType);   // Target field
```

### The Wrapper's Purpose

The JSON wrapper exists because:
1. **Encoding requirement**: Every STObject needs a field identifier
2. **Type safety**: Explicitly declares the object type
3. **Consistency**: All arrays follow this pattern
4. **Serialization**: Required for binary ↔ JSON conversion

However, internally, once you index into an array, you're already at the object level.

## Common Patterns

### Accessing Memo Fields

```rust,ignore
// Access first memo's type
let mut locator = Locator::new();
locator.pack(sfield::Memos);
locator.pack(0);
locator.pack(sfield::MemoType);

let mut buffer = [0u8; 256];
let len = get_tx_nested_field(
    locator.get_addr(),
    locator.num_packed_bytes(),
    buffer.as_mut_ptr(),
    buffer.len()
)?;
```

### Accessing Signer Fields

```rust,ignore
// Access second signer's account
let mut locator = Locator::new();
locator.pack(sfield::Signers);
locator.pack(1);  // Second signer (0-indexed)
locator.pack(sfield::Account);

let mut account = [0u8; 20];
let len = get_tx_nested_field(
    locator.get_addr(),
    locator.num_packed_bytes(),
    account.as_mut_ptr(),
    account.len()
)?;
```

### Accessing Oracle Data

```rust,ignore
// Access price from oracle document
let mut locator = Locator::new();
locator.pack(sfield::PriceDataSeries);
locator.pack(0);
locator.pack(sfield::AssetPrice);

let mut price_buf = [0u8; 8];
let len = get_ledger_obj_nested_field(
    oracle_slot,
    locator.get_addr(),
    locator.num_packed_bytes(),
    price_buf.as_mut_ptr(),
    price_buf.len()
)?;
```

### Iterating Arrays

```rust,ignore
// Get array length
let memo_count = get_tx_array_len(sfield::Memos)?;

// Process each memo
for i in 0..memo_count {
    // Build locator for memo fields
    let mut locator = Locator::new();
    locator.pack(sfield::Memos);
    locator.pack(i as i32);
    locator.pack(sfield::MemoType);

    // Read MemoType
    let type_len = get_tx_nested_field(
        locator.get_addr(),
        locator.num_packed_bytes(),
        type_buf.as_mut_ptr(),
        type_buf.len()
    )?;

    // Reuse locator for MemoData (same path, different field)
    locator.repack_last(sfield::MemoData);

    // Read MemoData
    let data_len = get_tx_nested_field(
        locator.get_addr(),
        locator.num_packed_bytes(),
        data_buf.as_mut_ptr(),
        data_buf.len()
    )?;
}
```

**Tip**: Use `repack_last()` to efficiently access different fields at the same nesting level without rebuilding the entire locator.

## Field Code Reference

The SField codes are in: `xrpl-wasm-std/src/sfield.rs`

Developers should use named SFields. For example, use `sfield::MemoType` instead of the int value.

### Common Transaction Fields

| Field | Code | Type | Size |
|-------|------|------|------|
| Account | 524289 | AccountID | 20 |
| TransactionType | 65538 | UInt16 | 2 |
| Fee | 393224 | Amount | 8 |
| Sequence | 131076 | UInt32 | 4 |
| Flags | 131074 | UInt32 | 4 |
| SourceTag | 131075 | UInt32 | 4 |
| DestinationTag | 131086 | UInt32 | 4 |
| SigningPubKey | 458755 | Blob | 33 |
| TxnSignature | 458756 | Blob | Variable |

### Escrow Fields

| Field | Code | Type | Size |
|-------|------|------|------|
| Owner | 524290 | AccountID | 20 |
| Destination | 524291 | AccountID | 20 |
| Amount | 393217 | Amount | 8 |
| Condition | 458769 | Blob | 32 |
| CancelAfter | 262156 | UInt32 | 4 |
| FinishAfter | 262157 | UInt32 | 4 |
| SourceTag | 196624 | UInt32 | 4 |
| DestinationTag | 196622 | UInt32 | 4 |

### Array Fields

| Field | Code | Type |
|-------|------|------|
| Memos | 983049 | Array of Memo |
| Signers | 983043 | Array of Signer |
| SignerEntries | 720900 | Array of SignerEntry |
| Paths | 65537 | Array of Path |

### Nested Object Fields

| Parent | Field | Code | Type |
|--------|-------|------|------|
| Memo | MemoType | 458764 | Blob |
| Memo | MemoData | 458765 | Blob |
| Memo | MemoFormat | 458766 | Blob |
| Signer | Account | 524291 | AccountID |
| Signer | TxnSignature | 524292 | Blob |
| Signer | SigningPubKey | 524293 | Blob |

## Troubleshooting

### Common Errors

#### FieldNotFound (-2)
- Field doesn't exist in the object
- Optional field not present
- Wrong field code used

**Solution**: Check if field is optional, verify field code

#### NotLeafField (-5)
- Trying to read an object/array as a value
- Missing array index in locator

**Solution**: Add array index or access sub-fields

#### LocatorMalformed (-6)
- Locator exceeds 64 bytes
- Invalid packing sequence
- Corrupted locator data

**Solution**: Check locator depth, verify packing order

#### NoArray (-4)
- Expected array field not found
- Wrong field code for array

**Solution**: Verify field is actually an array type

**Note**: This error code list is subject to expansion. Additional error codes may be added in future versions. For the most up-to-date list, see the [error code definitions in xrpl-wasm-std](https://github.com/ripple/craft/blob/main/xrpl-wasm-std/src/host/error_codes.rs).

### Debugging Tips

1. **Use trace functions** to output field values during development
2. **Check array lengths** before accessing elements
3. **Verify field existence** with proper error handling
4. **Test with fixtures** that have all optional fields
5. **Compare with JSON** to understand structure (but follow internal paths!)

### Best Practices

1. **Cache array lengths** to avoid repeated calls
2. **Reuse locators** when accessing multiple fields at same level
3. **Handle optional fields** gracefully
4. **Use constants** for field codes
5. **Document** expected fields

## Examples

### Complete Memo Processing

```rust,ignore
use xrpl_wasm_std::locator::Locator;
use xrpl_wasm_std::host::{get_tx_array_len, get_tx_nested_field};
use xrpl_wasm_std::sfield;

fn process_memos() -> Result<()> {
    // Get memo count
    let count = get_tx_array_len(sfield::Memos)?;

    for i in 0..count {
        // Build locator for MemoType
        let mut type_loc = Locator::new();
        type_loc.pack(sfield::Memos);
        type_loc.pack(i);
        type_loc.pack(sfield::MemoType);

        // Read MemoType
        let mut type_buf = [0u8; 256];
        let type_len = get_tx_nested_field(
            type_loc.get_addr(),
            type_loc.num_packed_bytes(),
            type_buf.as_mut_ptr(),
            type_buf.len()
        )?;

        // Reuse locator for MemoData (same path, different field)
        type_loc.repack_last(sfield::MemoData);

        // Read MemoData
        let mut data_buf = [0u8; 256];
        let data_len = get_tx_nested_field(
            type_loc.get_addr(),
            type_loc.num_packed_bytes(),
            data_buf.as_mut_ptr(),
            data_buf.len()
        )?;

        // Process memo
        let memo_type = &type_buf[..type_len as usize];
        let memo_data = &data_buf[..data_len as usize];

        // Your logic here...
    }

    Ok(())
}
```

### Oracle Price Access

```rust,ignore
fn get_oracle_price(oracle_slot: i32) -> Result<u64> {
    // Build locator for first price entry
    let mut locator = Locator::new();
    locator.pack(sfield::PriceDataSeries);
    locator.pack(0);
    locator.pack(sfield::AssetPrice);

    // Read price
    let mut price_buf = [0u8; 8];
    let len = get_ledger_obj_nested_field(
        oracle_slot,
        locator.get_addr(),
        locator.num_packed_bytes(),
        price_buf.as_mut_ptr(),
        price_buf.len()
    )?;

    // Convert to u64
    Ok(u64::from_le_bytes(price_buf))
}
```

## See Also

- [Binary Format Reference](https://xrpl.org/docs/references/protocol/binary-format)
- [Ledger Entry Types](https://xrpl.org/docs/references/protocol/ledger-data/ledger-entry-types)
