# Float Operations Test Module

This WebAssembly module tests floating-point operations in wasm-host-simulator.

## Building

Build using:

```bash
cargo build --target wasm32v1-none
cargo build --target wasm32v1-none --release
```

The resulting WASM file will be located at:

```
./target/wasm32v1-none/release/float_tests.wasm
```

## Running with wasm-host-simulator

Run the contract using the wasm-host-simulator application:

```bash
cd ../../../
cargo run --package wasm-host-simulator --bin wasm-host-simulator -- --dir projects/e2e-tests/float_tests --project float_tests
```

## Rounding Modes

The host honors the `rounding_mode` parameter for float operations by setting rippled Number's rounding mode per call. Behavior should match rippled.

Supported rounding modes:

- `0`: Round to nearest (ties to even)
- `1`: Round towards zero
- `2`: Round down (floor)
- `3`: Round up (ceiling)

### Precision and Compatibility

Float operations are backed by rippled's Number implementation to ensure compatibility with validator behavior, including edge cases around limits and rounding.

### Performance Considerations

The FFI-backed implementation should be performant for typical contract workloads.

## Guarantees

1. **Rounding modes are fully supported** and honored by the host
2. **Bit-for-bit compatibility** with rippled validators for IOU values
3. **Consistent behavior** across all XRPL nodes, per the XRPL protocol

## Test Coverage

The test module covers:

- ✅ Float creation from integers
- ✅ Float comparison operations
- ✅ Basic arithmetic (add, subtract, multiply, divide)
- ✅ Mathematical functions (root, log)
- ✅ Special values (positive/negative)
- ✅ Rounding mode variations
- ✅ Edge cases matching rippled behavior

## Notes

- The wasm-host-simulator uses mock implementations and may not reflect production behavior
- For production testing, use devnet or a standalone rippled node
- Float values are represented in XRPL's custom 64-bit format, not IEEE 754
