#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_wasm_std::core::ledger_objects::current_escrow::{self, CurrentEscrow};
use xrpl_wasm_std::core::ledger_objects::traits::CurrentEscrowFields;
use xrpl_wasm_std::core::types::contract_data::XRPL_CONTRACT_DATA_SIZE;
use xrpl_wasm_std::core::types::keylets::XRPL_KEYLET_SIZE;
use xrpl_wasm_std::host::trace::{DataRepr, trace_data, trace_num};
use xrpl_wasm_std::host::{Result::Err, Result::Ok};
use xrpl_wasm_std::{host, host::error_codes::match_result_code};

/// Main finish function for data field-based atomic swap with two-phase execution.
///
/// This function implements a stateful atomic swap using the escrow's data field:
///
/// PHASE 1 (data.len <= 32):
/// 1. Validates the data field contains exactly 32 bytes (first escrow keylet)
/// 2. Verifies the referenced first escrow exists on the ledger
/// 3. Retrieves current escrow's CancelAfter field as the swap deadline
/// 4. Appends the CancelAfter timestamp to the data field (36 bytes total)
/// 5. Returns 0 (failure) to wait for the second execution
///
/// PHASE 2 (data.len > 32):
/// 1. Extracts the CancelAfter timestamp from the last 4 bytes of data
/// 2. Gets the current ledger time
/// 3. Validates that current time < CancelAfter (within deadline)
/// 4. Returns 1 (success) if within deadline, 0 (failure) if expired
///
/// The two-phase design provides built-in timing coordination and prevents
/// stale swap attempts after the deadline expires.
#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let current_escrow = current_escrow::get_current_escrow();

    // Get the current data field - this stores the atomic swap state
    let mut current_data = match current_escrow.get_data() {
        Ok(data) => data,
        Err(e) => {
            let _ = trace_num("Error getting current escrow data:", e.code() as i64);
            return e.code();
        }
    };

    let _ = trace_num("Current data length:", current_data.len as i64);
    let _ = trace_data(
        "Current data:",
        &current_data.data[0..current_data.len],
        DataRepr::AsHex,
    );

    // STATE MACHINE: Determine execution phase based on data field length
    // Phase 1: data.len <= 32 (contains only first escrow keylet)
    // Phase 2: data.len > 32 (contains first escrow keylet + timing data)
    if current_data.len <= XRPL_KEYLET_SIZE {
        // PHASE 1: Initialization - validate first escrow and set timing deadline

        // Validate that the data contains exactly 32 bytes (first escrow keylet)
        if current_data.len != XRPL_KEYLET_SIZE {
            let _ = trace_num(
                "Invalid data length for first run, expected 32 bytes, got:",
                current_data.len as i64,
            );
            return 0;
        }

        // Extract the first escrow keylet from data field
        let first_escrow_id: [u8; XRPL_KEYLET_SIZE] = current_data.data[0..32].try_into().unwrap();
        let _ = trace_data(
            "First escrow ID from data:",
            &first_escrow_id,
            DataRepr::AsHex,
        );

        // Verify the referenced first escrow exists on the ledger
        // This ensures we're referencing a valid counterpart for the atomic swap
        let first_escrow_slot =
            unsafe { host::cache_ledger_obj(first_escrow_id.as_ptr(), first_escrow_id.len(), 0) };
        if first_escrow_slot < 0 {
            let _ = trace_num(
                "Failed to cache first escrow, error:",
                first_escrow_slot as i64,
            );
            return 0;
        }

        // Get current escrow's CancelAfter field - this becomes our swap deadline
        let cancel_after = match current_escrow.get_cancel_after() {
            Ok(Some(cancel_after)) => cancel_after,
            Ok(None) => {
                let _ = trace_num("Current escrow has no CancelAfter field", 0);
                return 0;
            }
            Err(e) => {
                let _ = trace_num("Error getting CancelAfter:", e.code() as i64);
                return e.code();
            }
        };

        let _ = trace_num("Current escrow CancelAfter:", cancel_after as i64);

        // Append CancelAfter timestamp to data field (4 bytes, little-endian)
        // This stores the deadline for phase 2 validation
        let cancel_after_bytes = cancel_after.to_le_bytes();
        if current_data.len + 4 > XRPL_CONTRACT_DATA_SIZE {
            let _ = trace_num("Data would exceed maximum size", 0);
            return 0;
        }

        current_data.data[current_data.len..current_data.len + 4]
            .copy_from_slice(&cancel_after_bytes);
        current_data.len += 4;

        let _ = trace_num("Updated data length:", current_data.len as i64);
        let _ = trace_data(
            "Updated data:",
            &current_data.data[0..current_data.len],
            DataRepr::AsHex,
        );

        // Persist the updated data field to the escrow object
        match <CurrentEscrow as CurrentEscrowFields>::update_current_escrow_data(current_data) {
            Ok(()) => {
                let _ = trace_num("Successfully updated escrow data", 0);
            }
            Err(e) => {
                let _ = trace_num("Error updating escrow data:", e.code() as i64);
                return e.code();
            }
        }

        // Return 0 (failure) to indicate phase 1 complete, wait for phase 2
        0
    } else {
        // PHASE 2: Timing validation - check if we're within the deadline

        // Validate data field contains at least 36 bytes (32 bytes keylet + 4 bytes timing)
        if current_data.len < XRPL_KEYLET_SIZE + 4 {
            let _ = trace_num(
                "Invalid data length for second run, expected at least 36 bytes, got:",
                current_data.len as i64,
            );
            return 0;
        }

        // Extract the CancelAfter timestamp from the last 4 bytes of data field
        let cancel_after_bytes: [u8; 4] = current_data.data[current_data.len - 4..current_data.len]
            .try_into()
            .unwrap();
        let cancel_after = u32::from_le_bytes(cancel_after_bytes);
        let _ = trace_num("Extracted CancelAfter:", cancel_after as i64);

        // Get current ledger time for deadline comparison
        let current_time = unsafe {
            let result_code = host::get_parent_ledger_time();
            match_result_code(result_code, || Some(result_code as u32))
        };

        let current_time = match current_time {
            Ok(Some(time)) => time,
            Ok(None) => {
                let _ = trace_num("Failed to get parent ledger time", 0);
                return 0;
            }
            Err(e) => {
                let _ = trace_num("Error getting parent ledger time:", e.code() as i64);
                return e.code();
            }
        };

        let _ = trace_num("Current ledger time:", current_time as i64);

        // ATOMIC SWAP TIMING VALIDATION
        // Only allow completion if current time is before the deadline
        // This prevents stale swap attempts and enforces time-based coordination
        if current_time < cancel_after {
            let _ = trace_num("Atomic swap executed before CancelAfter - success!", 0);
            1 // Success - escrow completes within deadline
        } else {
            let _ = trace_num("Atomic swap attempted after CancelAfter - failed", 0);
            0 // Failure - deadline exceeded, swap expired
        }
    }
}
