#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_wasm_stdlib::core::constants::{ACCOUNT_ONE, ACCOUNT_ZERO};
use xrpl_wasm_stdlib::core::current_tx::escrow_finish::{EscrowFinish, get_current_escrow_finish};
use xrpl_wasm_stdlib::core::current_tx::traits::{EscrowFinishFields, TransactionCommonFields};
use xrpl_wasm_stdlib::core::locator::Locator;
use xrpl_wasm_stdlib::core::types::account_id::AccountID;
use xrpl_wasm_stdlib::core::types::blob::Blob;
use xrpl_wasm_stdlib::core::types::public_key::PublicKey;
use xrpl_wasm_stdlib::core::types::transaction_type::TransactionType;
use xrpl_wasm_stdlib::core::types::uint::Hash256;
use xrpl_wasm_stdlib::host;
use xrpl_wasm_stdlib::host::trace::{
    DataRepr, trace, trace_account, trace_account_buf, trace_amount, trace_data, trace_num,
};
use xrpl_wasm_stdlib::{assert_eq, sfield};

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let _ = trace("$$$$$ STARTING WASM EXECUTION $$$$$");
    let _ = trace("");

    // The transaction prompting execution of this contract.
    let escrow_finish: EscrowFinish = get_current_escrow_finish();

    // ########################################
    // Trace All EscrowFinish Fields
    // ########################################
    {
        let _ = trace("### Trace All EscrowFinish Fields");
        let _ = trace("{ ");
        let _ = trace("  -- Common Fields");

        // Trace Field: TransactionID
        let current_tx_id: Hash256 = escrow_finish.get_id().unwrap();
        let _ = trace_data("  EscrowFinish TxId:", &current_tx_id.0, DataRepr::AsHex);
        assert_eq!(current_tx_id, EXPECTED_TX_ID.into());

        // Trace Field: Account
        let account = escrow_finish.get_account().unwrap();
        let _ = trace_account("  Account:", &account);
        if account.0.eq(&ACCOUNT_ONE.0) {
            let _ = trace("    AccountID == ACCOUNT_ONE => TRUE");
        } else {
            let _ = trace("    AccountID == ACCOUNT_ONE => FALSE");
            assert_eq!(account, ACCOUNT_ONE);
        }

        // Trace Field: TransactionType
        let transaction_type: TransactionType = escrow_finish.get_transaction_type().unwrap();
        assert_eq!(transaction_type, TransactionType::EscrowFinish);
        let tx_type_bytes: [u8; 2] = transaction_type.into();
        let _ = trace_data(
            "  TransactionType (EscrowFinish):",
            &tx_type_bytes,
            DataRepr::AsHex,
        );

        // Trace Field: ComputationAllowance
        let computation_allowance: u32 = escrow_finish.get_computation_allowance().unwrap();
        assert_eq!(computation_allowance, 1000001);
        let _ = trace_num("  ComputationAllowance:", computation_allowance as i64);

        // Trace Field: Fee
        let fee = escrow_finish.get_fee().unwrap();
        let _ = trace_amount("  Fee:", &fee);

        // Trace Field: Sequence
        let sequence: u32 = escrow_finish.get_sequence().unwrap();
        assert_eq!(sequence, 4294967295);
        let _ = trace_num("  Sequence:", sequence as i64);

        // Trace Field: AccountTxnID
        let opt_account_txn_id = escrow_finish.get_account_txn_id().unwrap();
        if let Some(account_txn_id) = opt_account_txn_id {
            assert_eq!(account_txn_id.0, EXPECTED_ACCOUNT_TXN_ID);
            let _ = trace_data("  AccountTxnID:", &account_txn_id.0, DataRepr::AsHex);
        }

        // Trace Field: Flags
        let opt_flags = escrow_finish.get_flags().unwrap();
        if let Some(flags) = opt_flags {
            assert_eq!(flags, 4294967294);
            let _ = trace_num("  Flags:", flags as i64);
        }

        // Trace Field: LastLedgerSequence
        let opt_last_ledger_sequence = escrow_finish.get_last_ledger_sequence().unwrap();
        if let Some(last_ledger_sequence) = opt_last_ledger_sequence {
            assert_eq!(last_ledger_sequence, 4294967292);
            let _ = trace_num("  LastLedgerSequence:", last_ledger_sequence as i64);
        }

        // Trace Field: NetworkID
        let opt_network_id = escrow_finish.get_network_id().unwrap();
        if let Some(network_id) = opt_network_id {
            assert_eq!(network_id, 4294967291);
            let _ = trace_num("  NetworkID:", network_id as i64);
        }

        // Trace Field: SourceTag
        let opt_source_tag = escrow_finish.get_source_tag().unwrap();
        if let Some(source_tag) = opt_source_tag {
            assert_eq!(source_tag, 4294967290);
            let _ = trace_num("  SourceTag:", source_tag as i64);
        }

        // Trace Field: SigningPubKey
        let signing_pub_key = escrow_finish.get_signing_pub_key().unwrap();
        assert_eq!(signing_pub_key.0, EXPECTED_TX_SIGNING_PUB_KEY);
        let _ = trace_data("  SigningPubKey:", &signing_pub_key.0, DataRepr::AsHex);

        // Trace Field: TicketSequence
        let opt_ticket_sequence = escrow_finish.get_ticket_sequence().unwrap();
        if let Some(ticket_sequence) = opt_ticket_sequence {
            assert_eq!(ticket_sequence, 4294967289);
            let _ = trace_num("  TicketSequence:", ticket_sequence as i64);
        }

        let array_len = unsafe { host::get_tx_array_len(sfield::Memos) };
        assert_eq!(array_len, 1);
        let _ = trace_num("  Memos array len:", array_len as i64);

        let mut memo_buf = [0u8; 1024];
        let mut locator = Locator::new();
        locator.pack(sfield::Memos);
        locator.pack(0);
        locator.pack(sfield::Memo);
        locator.pack(sfield::MemoType);
        let output_len = unsafe {
            host::get_tx_nested_field(
                locator.as_ptr(),
                locator.num_packed_bytes(),
                memo_buf.as_mut_ptr(),
                memo_buf.len(),
            )
        };
        let _ = trace("    Memo #: 1");
        let _ = trace_data(
            "      MemoType:",
            &memo_buf[..output_len as usize],
            DataRepr::AsHex,
        );

        locator.repack_last(sfield::MemoData);
        let output_len = unsafe {
            host::get_tx_nested_field(
                locator.as_ptr(),
                locator.num_packed_bytes(),
                memo_buf.as_mut_ptr(),
                memo_buf.len(),
            )
        };
        let _ = trace_data(
            "      MemoData:",
            &memo_buf[..output_len as usize],
            DataRepr::AsHex,
        );

        locator.repack_last(sfield::MemoFormat);
        let output_len = unsafe {
            host::get_tx_nested_field(
                locator.as_ptr(),
                locator.num_packed_bytes(),
                memo_buf.as_mut_ptr(),
                memo_buf.len(),
            )
        };
        let _ = trace_data(
            "      MemoFormat:",
            &memo_buf[..output_len as usize],
            DataRepr::AsHex,
        );

        let array_len = unsafe { host::get_tx_array_len(sfield::Signers) };
        assert_eq!(array_len, 2);
        let _ = trace_num("  Signers array len:", array_len as i64);

        for i in 0..array_len {
            let mut buf = [0x00; 64];
            let mut locator = Locator::new();
            locator.pack(sfield::Signers);
            locator.pack(i);
            locator.pack(sfield::Account);
            let output_len = unsafe {
                host::get_tx_nested_field(
                    locator.as_ptr(),
                    locator.num_packed_bytes(),
                    buf.as_mut_ptr(),
                    buf.len(),
                )
            };
            if output_len < 0 {
                let _ = trace_num("  cannot get Account, error:", output_len as i64);
                panic!()
            }
            let _ = trace_num("    Signer #:", i as i64);
            let _ = trace_account_buf("     Account:", &buf[..20].try_into().unwrap());

            locator.repack_last(sfield::TxnSignature);
            let output_len = unsafe {
                host::get_tx_nested_field(
                    locator.as_ptr(),
                    locator.num_packed_bytes(),
                    buf.as_mut_ptr(),
                    buf.len(),
                )
            };
            if output_len < 0 {
                let _ = trace_num("  cannot get TxnSignature, error:", output_len as i64);
                panic!()
            }
            let _ = trace_data(
                "     TxnSignature:",
                &buf[..output_len as usize],
                DataRepr::AsHex,
            );

            locator.repack_last(sfield::SigningPubKey);
            let output_len = unsafe {
                host::get_tx_nested_field(
                    locator.as_ptr(),
                    locator.num_packed_bytes(),
                    buf.as_mut_ptr(),
                    buf.len(),
                )
            };
            let signing_pub_key: PublicKey = buf.into();
            assert_eq!(signing_pub_key.0, EXPECTED_TX_SIGNING_PUB_KEY);

            if output_len < 0 {
                let _ = trace_num(
                    "  Error getting SigningPubKey. error_code = ",
                    output_len as i64,
                );
                break;
            }
            let _ = trace_data(
                "     SigningPubKey:",
                &buf[..output_len as usize],
                DataRepr::AsHex,
            );
        }

        let txn_signature: Blob = escrow_finish.get_txn_signature().unwrap();
        let mut signature_bytes = [0u8; 71];
        signature_bytes.copy_from_slice(&txn_signature.data[..71]);
        assert_eq!(signature_bytes, EXPECTED_TXN_SIGNATURE);
        let _ = trace_data("  TxnSignature:", &signature_bytes, DataRepr::AsHex);

        let _ = trace("  -- EscrowFinish Fields");

        // Trace Field: Account
        let owner: AccountID = escrow_finish.get_owner().unwrap();
        let _ = trace_account("  Owner:", &owner);
        if owner.0[0].eq(&ACCOUNT_ZERO.0[0]) {
            let _ = trace("    AccountID == ACCOUNT_ZERO => TRUE");
        } else {
            let _ = trace("    AccountID == ACCOUNT_ZERO => FALSE");
            assert_eq!(owner, ACCOUNT_ZERO);
        }

        // Trace Field: OfferSequence
        let offer_sequence: u32 = escrow_finish.get_offer_sequence().unwrap();
        assert_eq!(offer_sequence, 4294967293);
        let _ = trace_num("  OfferSequence:", offer_sequence as i64);

        // Trace Field: Condition
        let opt_condition = escrow_finish.get_condition().unwrap();
        if let Some(condition) = opt_condition {
            assert_eq!(condition.0, EXPECTED_ESCROW_FINISH_CONDITION);
            let _ = trace_data("  Condition:", &condition.0, DataRepr::AsHex);
        }

        let opt_fulfillment = escrow_finish.get_fulfillment().unwrap();
        if let Some(fulfillment) = opt_fulfillment {
            assert_eq!(
                &fulfillment.data[..fulfillment.len],
                EXPECTED_ESCROW_FINISH_FULFILLMENT
            );
            let _ = trace_data(
                "  Fulfillment:",
                &fulfillment.data[..fulfillment.len],
                DataRepr::AsHex,
            );
        }

        // CredentialIDs (Array of Hashes)
        let array_len = unsafe { host::get_tx_array_len(sfield::CredentialIDs) };
        let _ = trace_num("  CredentialIDs array len:", array_len as i64);
        for i in 0..array_len {
            let mut buf = [0x00; 32];
            let mut locator = Locator::new();
            locator.pack(sfield::CredentialIDs);
            locator.pack(i);
            let output_len = unsafe {
                host::get_tx_nested_field(
                    locator.as_ptr(),
                    locator.num_packed_bytes(),
                    buf.as_mut_ptr(),
                    buf.len(),
                )
            };
            if i == 0 {
                assert_eq!(buf, EXPECTED_CURRENT_ESCROW_CREDENTIAL1);
            } else if i == 1 {
                assert_eq!(buf, EXPECTED_CURRENT_ESCROW_CREDENTIAL2);
            } else if i == 2 {
                assert_eq!(buf, EXPECTED_CURRENT_ESCROW_CREDENTIAL3);
            } else {
                panic!()
            }

            let _ = trace_data(
                "  CredentialID:",
                &buf[..output_len as usize],
                DataRepr::AsHex,
            );
        }

        let _ = trace("}");
        let _ = trace(""); // Newline
    }

    let _ = trace("$$$$$ WASM EXECUTION COMPLETE $$$$$");
    1 // <-- Finish the escrow to indicate a successful outcome
}

/// The following are private constants used for testing purposes to enforce value checks in this
/// contract (to ensure that code changes don't break this contract).
const EXPECTED_TX_ID: [u8; 32] = [
    0x74, 0x46, 0x51, 0x21, 0x37, 0x28, 0x13, 0xCB, 0xA4, 0xC7, 0x7E, 0x31, 0xF1, 0x2E, 0x13, 0x71,
    0x63, 0xF5, 0xB2, 0x50, 0x9B, 0x16, 0xAC, 0x17, 0x03, 0xEC, 0xF0, 0xDA, 0x19, 0x4B, 0x2D, 0xD4,
];

const EXPECTED_ACCOUNT_TXN_ID: [u8; 32] = [0xDD; 32];

const EXPECTED_TX_SIGNING_PUB_KEY: [u8; 33] = [
    0x03, 0x30, 0xE7, 0xFC, 0x9D, 0x56, 0xBB, 0x25, 0xD6, 0x89, 0x3B, 0xA3, 0xF3, 0x17, 0xAE, 0x5B,
    0xCF, 0x33, 0xB3, 0x29, 0x1B, 0xD6, 0x3D, 0xB3, 0x26, 0x54, 0xA3, 0x13, 0x22, 0x2F, 0x7F, 0xD0,
    0x20,
];

const EXPECTED_TXN_SIGNATURE: [u8; 71] = [
    0x30, 0x45, 0x02, 0x21, 0x00, 0x8A, 0xD5, 0xEE, 0x48, 0xF7, 0xF1, 0x04, 0x78, 0x13, 0xE7, 0x9C,
    0x17, 0x4F, 0xE4, 0x01, 0xD0, 0x23, 0xA4, 0xB4, 0xA7, 0xB9, 0x9A, 0xF8, 0x26, 0xE0, 0x81, 0xDB,
    0x1D, 0xFF, 0x7B, 0x9C, 0x51, 0x02, 0x20, 0x13, 0x3F, 0x05, 0xB7, 0xFD, 0x3D, 0x7D, 0x7F, 0x16,
    0x3E, 0x8C, 0x77, 0xEE, 0x0A, 0x49, 0xD0, 0x26, 0x19, 0xAB, 0x6C, 0x77, 0xCC, 0x34, 0x87, 0xD0,
    0x09, 0x5C, 0x9B, 0x34, 0x03, 0x3C, 0x1C,
];

const EXPECTED_ESCROW_FINISH_CONDITION: [u8; 32] = [0x33; 32];
const EXPECTED_ESCROW_FINISH_FULFILLMENT: [u8; 32] = [0x21; 32];

const EXPECTED_CURRENT_ESCROW_CREDENTIAL1: [u8; 32] = [
    0x0A, 0xBA, 0x05, 0xA3, 0x49, 0x49, 0xF2, 0xCE, 0xD4, 0x10, 0x25, 0x91, 0x4F, 0xC4, 0xF2, 0x67,
    0x88, 0x3F, 0x1D, 0x38, 0x8A, 0x65, 0x45, 0xAF, 0xB4, 0x86, 0x34, 0x66, 0xFA, 0xA6, 0xF2, 0x8C,
];

const EXPECTED_CURRENT_ESCROW_CREDENTIAL2: [u8; 32] = [
    0xD0, 0xA0, 0x63, 0xDE, 0xE0, 0xB0, 0xEC, 0x95, 0x22, 0xCF, 0x35, 0xCD, 0x55, 0x77, 0x1B, 0x5D,
    0xCA, 0xFA, 0x19, 0xA1, 0x33, 0xEE, 0x46, 0xA0, 0x29, 0x5E, 0x4D, 0x08, 0x9A, 0xF8, 0x64, 0x38,
];

const EXPECTED_CURRENT_ESCROW_CREDENTIAL3: [u8; 32] = [
    0xD2, 0xEF, 0xD3, 0x85, 0x89, 0x60, 0x9A, 0xE5, 0x70, 0xD1, 0x7E, 0x99, 0x57, 0xCE, 0x60, 0x02,
    0xE7, 0x64, 0xA6, 0x3E, 0xE6, 0x6F, 0xE8, 0xCA, 0xA2, 0x76, 0x89, 0x76, 0xAB, 0xD6, 0x0B, 0xFF,
];
