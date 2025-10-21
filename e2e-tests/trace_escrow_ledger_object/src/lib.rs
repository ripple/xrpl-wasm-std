#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_wasm_std::assert_eq;
use xrpl_wasm_std::core::ledger_objects::current_escrow::{CurrentEscrow, get_current_escrow};
use xrpl_wasm_std::core::ledger_objects::traits::{
    CurrentEscrowFields, CurrentLedgerObjectCommonFields,
};
use xrpl_wasm_std::host::trace::{DataRepr, trace, trace_amount, trace_data, trace_num};
use xrpl_wasm_std::host::{Result::Err, Result::Ok};

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let _ = trace("$$$$$ STARTING WASM EXECUTION $$$$$");
    let _ = trace("");

    let current_escrow: CurrentEscrow = get_current_escrow();

    // ########################################
    // Trace All Current Escrow Ledger Object Fields
    // ########################################
    {
        let _ = trace("### Trace Current Escrow Ledger Object Fields");
        let _ = trace("{ ");
        let _ = trace("  -- Common Fields");

        // Trace Field: Account
        let account = current_escrow.get_account().unwrap();
        assert_eq!(account.0, EXPECTED_CURRENT_ESCROW_ACCOUNT_ID);
        let _ = trace_data("  Account:", &account.0, DataRepr::AsHex);

        // Trace Field: Amount
        let amount = current_escrow.get_amount().unwrap();
        let _ = trace_amount("  Amount:", &amount);

        // Trace Field: LedgerEntryType
        let ledger_entry_type = current_escrow.get_ledger_entry_type().unwrap();
        assert_eq!(ledger_entry_type, 117);
        let _ = trace_num("  LedgerEntryType:", ledger_entry_type as i64);

        // Trace Field: CancelAfter
        let opt_cancel_after = current_escrow.get_cancel_after().unwrap();
        if let Some(cancel_after) = opt_cancel_after {
            assert_eq!(cancel_after, 545440232);
            let _ = trace_num("  CancelAfter:", cancel_after as i64);
        }

        // Trace Field: Condition
        let opt_condition = current_escrow.get_condition().unwrap();
        if let Some(condition) = opt_condition {
            assert_eq!(condition.0, EXPECTED_ESCROW_CONDITION);
            let _ = trace_data("  Condition:", &condition.0, DataRepr::AsHex);
        }

        // Trace Field: Destination
        let destination = current_escrow.get_destination().unwrap();
        const EXPECTED_DESTINATION: [u8; 20] = [
            0x3E, 0x9D, 0x4A, 0x2B, 0x8A, 0xA0, 0x78, 0x0F, 0x68, 0x2D, 0x13, 0x6F, 0x7A, 0x56,
            0xD6, 0x72, 0x4E, 0xF5, 0x37, 0x54,
        ];
        assert_eq!(destination.0, EXPECTED_DESTINATION);
        let _ = trace_data("  Destination:", &destination.0, DataRepr::AsHex);

        // Trace Field: DestinationTag
        let opt_destination_tag = current_escrow.get_destination_tag().unwrap();
        if let Some(destination_tag) = opt_destination_tag {
            assert_eq!(destination_tag, 23480);
            let _ = trace_num("  DestinationTag:", destination_tag as i64);
        }

        // Trace Field: FinishAfter
        let opt_finish_after = current_escrow.get_finish_after().unwrap();
        if let Some(finish_after) = opt_finish_after {
            assert_eq!(finish_after, 545354132);
            let _ = trace_num("  FinishAfter:", finish_after as i64);
        }

        // Trace Field: Flags
        let result = current_escrow.get_flags();
        if let Ok(flags) = result {
            assert_eq!(flags, 0);
            let _ = trace_num("  Flags:", flags as i64);
        } else if let Err(error) = result {
            let _ = trace_num("  Error getting Flags. error_code = ", error.code() as i64);
        }

        // Trace Field: FinishFunction
        let opt_finish_function = current_escrow.get_finish_function().unwrap();
        if let Some(finish_function) = opt_finish_function {
            assert_eq!(finish_function.len, 172);
            let _ = trace_data(
                "  FinishFunction:",
                &finish_function.data[..finish_function.len],
                DataRepr::AsHex,
            );
        }

        // Trace Field: OwnerNode
        let owner_node = current_escrow.get_owner_node().unwrap();
        assert_eq!(owner_node, 0);
        let _ = trace_num("  OwnerNode:", owner_node as i64);

        // Trace Field: DestinationNode
        let opt_destination_node = current_escrow.get_destination_node().unwrap();
        if let Some(destination_node) = opt_destination_node {
            assert_eq!(destination_node, 0);
            let _ = trace_num("  DestinationNode:", destination_node as i64);
        }

        // Trace Field: PreviousTxnID
        let previous_txn_id = current_escrow.get_previous_txn_id().unwrap();
        assert_eq!(
            previous_txn_id.0,
            [
                0xC4, 0x4F, 0x2E, 0xB8, 0x41, 0x96, 0xB9, 0xAD, 0x82, 0x03, 0x13, 0xDB, 0xEB, 0xA6,
                0x31, 0x6A, 0x15, 0xC9, 0xA2, 0xD3, 0x57, 0x87, 0x57, 0x9E, 0xD1, 0x72, 0xB8, 0x7A,
                0x30, 0x13, 0x1D, 0xA7,
            ]
        );
        let _ = trace_data("  PreviousTxnID:", &previous_txn_id.0, DataRepr::AsHex);

        // Trace Field: PreviousTxnLgrSeq
        let previous_txn_lgr_seq = current_escrow.get_previous_txn_lgr_seq().unwrap();
        assert_eq!(previous_txn_lgr_seq, 28991004);
        let _ = trace_num("  PreviousTxnLgrSeq:", previous_txn_lgr_seq as i64);

        // Trace Field: SourceTag
        let opt_source_tag = current_escrow.get_source_tag().unwrap();
        if let Some(source_tag) = opt_source_tag {
            assert_eq!(source_tag, 11747);
            let _ = trace_num("  SourceTag:", 11747);
        }

        // Trace Field: `index` or `LedgerIndex`
        // The current decision is that this field should not be accessible from the ledger object.
        // let ledger_index = current_escrow.get_ledger_index().unwrap();
        // let _ = trace_data("  index:", &ledger_index.0, DataRepr::AsHex);

        let _ = trace("}");
        let _ = trace("");
    }

    let _ = trace("$$$$$ WASM EXECUTION COMPLETE $$$$$");
    1 // <-- Finish the escrow to indicate a successful outcome
}

/// The following are private constants used for testing purposes to enforce value checks in this
/// contract (to ensure that code changes don't break this contract).
const EXPECTED_ESCROW_CONDITION: [u8; 32] = [
    0xA0, 0x25, 0x80, 0x20, 0xA8, 0x2A, 0x88, 0xB2, 0xDF, 0x84, 0x3A, 0x54, 0xF5, 0x87, 0x72, 0xE4,
    0xA3, 0x86, 0x18, 0x66, 0xEC, 0xDB, 0x41, 0x57, 0x64, 0x5D, 0xD9, 0xAE, 0x52, 0x8C, 0x1D, 0x3A,
];

/// Represents rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn
const EXPECTED_CURRENT_ESCROW_ACCOUNT_ID: [u8; 20] = [
    0x4B, 0x4E, 0x9C, 0x06, 0xF2, 0x42, 0x96, 0x07, 0x4F, 0x7B, 0xC4, 0x8F, 0x92, 0xA9, 0x79, 0x16,
    0xC6, 0xDC, 0x5E, 0xA9,
];
