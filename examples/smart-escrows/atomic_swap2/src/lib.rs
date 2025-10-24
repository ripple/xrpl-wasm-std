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

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let current_escrow = current_escrow::get_current_escrow();

    // Get the current data field
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

    // Check if this is the first run (data length <= 32 bytes) or second run (data length > 32 bytes)
    if current_data.len <= XRPL_KEYLET_SIZE {
        // First run: Initialize with CancelAfter

        // Validate that the data contains a valid escrow ID (32 bytes)
        if current_data.len != XRPL_KEYLET_SIZE {
            let _ = trace_num(
                "Invalid data length for first run, expected 32 bytes, got:",
                current_data.len as i64,
            );
            return 0;
        }

        let first_escrow_id: [u8; XRPL_KEYLET_SIZE] = current_data.data[0..32].try_into().unwrap();
        let _ = trace_data(
            "First escrow ID from data:",
            &first_escrow_id,
            DataRepr::AsHex,
        );

        // Verify the first escrow exists
        let first_escrow_slot =
            unsafe { host::cache_ledger_obj(first_escrow_id.as_ptr(), first_escrow_id.len(), 0) };
        if first_escrow_slot < 0 {
            let _ = trace_num(
                "Failed to cache first escrow, error:",
                first_escrow_slot as i64,
            );
            return 0;
        }

        // Get current escrow's CancelAfter
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

        // Append CancelAfter to the data (4 bytes, little-endian)
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

        // Update the data field
        match <CurrentEscrow as CurrentEscrowFields>::update_current_escrow_data(current_data) {
            Ok(()) => {
                let _ = trace_num("Successfully updated escrow data", 0);
            }
            Err(e) => {
                let _ = trace_num("Error updating escrow data:", e.code() as i64);
                return e.code();
            }
        }

        // Return 0 to indicate failure (wait for second run)
        0
    } else {
        // Second run: Validate timing

        // Data should be at least 36 bytes (32 bytes escrow ID + 4 bytes CancelAfter)
        if current_data.len < XRPL_KEYLET_SIZE + 4 {
            let _ = trace_num(
                "Invalid data length for second run, expected at least 36 bytes, got:",
                current_data.len as i64,
            );
            return 0;
        }

        // Extract CancelAfter from the last 4 bytes
        let cancel_after_bytes: [u8; 4] = current_data.data[current_data.len - 4..current_data.len]
            .try_into()
            .unwrap();
        let cancel_after = u32::from_le_bytes(cancel_after_bytes);
        let _ = trace_num("Extracted CancelAfter:", cancel_after as i64);

        // Get current ledger time
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

        // Check if current time is before CancelAfter
        if current_time < cancel_after {
            let _ = trace_num("Atomic swap executed before CancelAfter - success!", 0);
            1 // Success
        } else {
            let _ = trace_num("Atomic swap attempted after CancelAfter - failed", 0);
            0 // Failure
        }
    }
}
