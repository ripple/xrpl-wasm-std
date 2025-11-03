#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_wasm_stdlib::core::current_tx::escrow_finish::{EscrowFinish, get_current_escrow_finish};
use xrpl_wasm_stdlib::core::current_tx::traits::TransactionCommonFields;
use xrpl_wasm_stdlib::host::Result;
use xrpl_wasm_stdlib::host::error_codes::{
    match_result_code, match_result_code_optional, match_result_code_with_expected_bytes,
    match_result_code_with_expected_bytes_optional,
};
use xrpl_wasm_stdlib::host::trace::trace;
use xrpl_wasm_stdlib::{decode_hex_20, decode_hex_32};

/// Main entry point for the gas benchmark contract
///
/// This contract exercises all optimized helper functions with controlled workloads
/// to measure gas consumption. Each benchmark section is marked with trace() calls
/// to help identify gas usage patterns.
///
/// Benchmarks covered:
/// - Transaction field access (get_account, get_fee)
/// - Result type operations (is_ok, is_err, ok, err)
/// - Error code matching (match_result_code, match_result_code_optional, etc.)
/// - Hex decoding (decode_hex_32, decode_hex_20)
#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let _ = trace("$$$$$ GAS BENCHMARK START $$$$$");

    // Get the current transaction
    let escrow_finish: EscrowFinish = get_current_escrow_finish();

    // Accumulate results to prevent compiler optimization
    let mut accumulator: u64 = 0;

    // Transaction field access benchmarks
    let _ = trace("BENCHMARK: get_account_id_field");
    accumulator = accumulator.wrapping_add(benchmark_account_id_field(&escrow_finish));

    let _ = trace("BENCHMARK: get_fee_field");
    accumulator = accumulator.wrapping_add(benchmark_fee_field(&escrow_finish));

    // Error code matching benchmarks
    let _ = trace("BENCHMARK: match_result_code");
    accumulator = accumulator.wrapping_add(benchmark_match_result_code());

    let _ = trace("BENCHMARK: match_result_code_optional");
    accumulator = accumulator.wrapping_add(benchmark_match_result_code_optional());

    let _ = trace("BENCHMARK: match_result_code_with_expected_bytes");
    accumulator = accumulator.wrapping_add(benchmark_match_result_code_with_expected_bytes());

    let _ = trace("BENCHMARK: match_result_code_with_expected_bytes_optional");
    accumulator =
        accumulator.wrapping_add(benchmark_match_result_code_with_expected_bytes_optional());

    // Result type method benchmarks
    let _ = trace("BENCHMARK: is_ok");
    accumulator = accumulator.wrapping_add(benchmark_is_ok(&escrow_finish));

    let _ = trace("BENCHMARK: is_err");
    accumulator = accumulator.wrapping_add(benchmark_is_err(&escrow_finish));

    let _ = trace("BENCHMARK: result_ok");
    accumulator = accumulator.wrapping_add(benchmark_result_ok(&escrow_finish));

    let _ = trace("BENCHMARK: result_err");
    accumulator = accumulator.wrapping_add(benchmark_result_err(&escrow_finish));

    // Hex decoding benchmarks
    let _ = trace("BENCHMARK: decode_hex_32");
    accumulator = accumulator.wrapping_add(benchmark_decode_hex_32());

    let _ = trace("BENCHMARK: decode_hex_20");
    accumulator = accumulator.wrapping_add(benchmark_decode_hex_20());

    let _ = trace("$$$$$ GAS BENCHMARK END $$$$$");

    // Return 1 if accumulator is non-zero (it always will be), preventing optimization
    if accumulator > 0 { 1 } else { 0 }
}

/// Benchmark get_account_id_field by repeatedly calling get_account()
fn benchmark_account_id_field(escrow_finish: &EscrowFinish) -> u64 {
    const ITERATIONS: usize = 100;
    let mut count = 0u64;
    for _ in 0..ITERATIONS {
        if escrow_finish.get_account().is_ok() {
            count += 1;
        }
    }
    count
}

/// Benchmark get_fee_field by repeatedly calling get_fee()
fn benchmark_fee_field(escrow_finish: &EscrowFinish) -> u64 {
    const ITERATIONS: usize = 100;
    let mut count = 0u64;
    for _ in 0..ITERATIONS {
        if escrow_finish.get_fee().is_ok() {
            count += 1;
        }
    }
    count
}

/// Benchmark match_result_code (basic success case)
fn benchmark_match_result_code() -> u64 {
    const ITERATIONS: usize = 100;
    let mut count = 0u64;
    for _ in 0..ITERATIONS {
        let result: Result<u32> = match_result_code(1, || 42u32);
        if result.is_ok() {
            count += 1;
        }
    }
    count
}

/// Benchmark match_result_code_optional (success with Some)
fn benchmark_match_result_code_optional() -> u64 {
    const ITERATIONS: usize = 100;
    let mut count = 0u64;
    for _ in 0..ITERATIONS {
        let result: Result<Option<u32>> = match_result_code_optional(1, || Some(42u32));
        if result.is_ok() {
            count += 1;
        }
    }
    count
}

/// Benchmark match_result_code_with_expected_bytes
fn benchmark_match_result_code_with_expected_bytes() -> u64 {
    const ITERATIONS: usize = 100;
    let mut count = 0u64;
    for _ in 0..ITERATIONS {
        let result: Result<u32> = match_result_code_with_expected_bytes(20, 20, || 42u32);
        if result.is_ok() {
            count += 1;
        }
    }
    count
}

/// Benchmark match_result_code_with_expected_bytes_optional
fn benchmark_match_result_code_with_expected_bytes_optional() -> u64 {
    const ITERATIONS: usize = 100;
    let mut count = 0u64;
    for _ in 0..ITERATIONS {
        let result: Result<Option<u32>> =
            match_result_code_with_expected_bytes_optional(20, 20, || Some(42u32));
        if result.is_ok() {
            count += 1;
        }
    }
    count
}

/// Benchmark is_ok() checks
fn benchmark_is_ok(escrow_finish: &EscrowFinish) -> u64 {
    const ITERATIONS: usize = 100;
    let mut count = 0u64;
    for _ in 0..ITERATIONS {
        if escrow_finish.get_account().is_ok() {
            count += 1;
        }
    }
    count
}

/// Benchmark is_err() checks
fn benchmark_is_err(escrow_finish: &EscrowFinish) -> u64 {
    const ITERATIONS: usize = 100;
    let mut count = 0u64;
    for _ in 0..ITERATIONS {
        if !escrow_finish.get_account().is_err() {
            count += 1;
        }
    }
    count
}

/// Benchmark Result::ok() conversion
fn benchmark_result_ok(escrow_finish: &EscrowFinish) -> u64 {
    const ITERATIONS: usize = 100;
    let mut count = 0u64;
    for _ in 0..ITERATIONS {
        if escrow_finish.get_account().ok().is_some() {
            count += 1;
        }
    }
    count
}

/// Benchmark Result::err() conversion
fn benchmark_result_err(escrow_finish: &EscrowFinish) -> u64 {
    const ITERATIONS: usize = 100;
    let mut count = 0u64;
    for _ in 0..ITERATIONS {
        if escrow_finish.get_account().err().is_none() {
            count += 1;
        }
    }
    count
}

/// Benchmark decode_hex_32
fn benchmark_decode_hex_32() -> u64 {
    const ITERATIONS: usize = 100;
    let hex = *b"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    let mut count = 0u64;
    for _ in 0..ITERATIONS {
        if decode_hex_32(&hex).is_some() {
            count += 1;
        }
    }
    count
}

/// Benchmark decode_hex_20
fn benchmark_decode_hex_20() -> u64 {
    const ITERATIONS: usize = 100;
    let hex = *b"00112233445566778899aabbccddeeff00112233";
    let mut count = 0u64;
    for _ in 0..ITERATIONS {
        if decode_hex_20(&hex).is_some() {
            count += 1;
        }
    }
    count
}
