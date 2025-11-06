//! # Trace Escrow Account Test
//!
//! This test ensures that every field on an AccountRoot ledger object can be successfully
//! traced from within a WASM smart contract.
//!
//! The test script configures an account with all possible AccountRoot fields, creates an
//! escrow with this contract as the finish condition, then finishes the escrow. This contract
//! loads the AccountRoot and traces every field to verify the WASM stdlib can access all
//! account data correctly.
#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_wasm_stdlib::assert_eq;
use xrpl_wasm_stdlib::core::current_tx::escrow_finish::{EscrowFinish, get_current_escrow_finish};
use xrpl_wasm_stdlib::core::current_tx::traits::TransactionCommonFields;
use xrpl_wasm_stdlib::core::ledger_objects::account_root::AccountRoot;
use xrpl_wasm_stdlib::core::ledger_objects::traits::{AccountFields, LedgerObjectCommonFields};
use xrpl_wasm_stdlib::core::types::account_id::AccountID;
use xrpl_wasm_stdlib::core::types::amount::Amount;
use xrpl_wasm_stdlib::core::types::keylets::account_keylet;
use xrpl_wasm_stdlib::host::cache_ledger_obj;
use xrpl_wasm_stdlib::host::trace::{DataRepr, trace, trace_data, trace_num};

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let _ = trace("$$$$$ STARTING WASM EXECUTION $$$$$");
    let _ = trace("");

    // The transaction prompting execution of this contract.
    let escrow_finish: EscrowFinish = get_current_escrow_finish();

    // ########################################
    // [EscrowFinish Account]: Trace AccountRoot Fields.
    // ########################################
    {
        // Get the account that's finishing the escrow (our configured test account)
        let account_id: AccountID = escrow_finish.get_account().unwrap();

        // Compute the keylet for this account's AccountRoot object
        // AccountRoot keylet = 0x61 (a) + SHA512Half(account_id)
        // use xrpl_wasm_stdlib::core::keylet::account_root_keylet;
        let account_keylet = account_keylet(&account_id).unwrap();

        // Try to cache the ledger object inside rippled
        let slot = unsafe { cache_ledger_obj(account_keylet.as_ptr(), 32, 0) };
        if slot < 0 {
            let _ = trace_num("Error slotting Account object", slot as i64);
            panic!()
        } else {
            let _ = trace_num("Account object slotted at", slot as i64);
        }

        // We use the trait-bound implementation so as not to duplicate accessor logic.
        let account = AccountRoot { slot_num: slot };

        let _ = trace("### Step #2: Trace AccountRoot Ledger Object");
        let _ = trace("{ ");
        let _ = trace("  -- Common Fields");

        // Trace the `Flags`
        let flags = account.get_flags().unwrap();
        // Flags can vary based on account settings, just trace the value
        let _ = trace_num("  Flags:", flags as i64);

        // Trace the `LedgerEntryType`
        let ledger_entry_type = account.ledger_entry_type().unwrap();
        assert_eq!(ledger_entry_type, 97); // 97 is the code for "AccountRoot"
        let _ = trace_num("  LedgerEntryType (AccountRoot):", ledger_entry_type as i64);
        let _ = trace("} ");

        let _ = trace("{ ");
        let _ = trace("  -- Account Specific Fields");

        // Trace the `Account`
        let account_id = account.get_account().unwrap();
        // Account is the hardcoded keylet we're looking up - just verify it's 20 bytes
        assert_eq!(account_id.0.len(), 20);
        let _ = trace_data("  Account:", &account_id.0, DataRepr::AsHex);

        // Trace the `AccountTxnID` (optional - required for testing)
        let account_txn_id_opt = account.account_txn_id().unwrap();
        let account_txn_id =
            account_txn_id_opt.expect("AccountTxnID should be present for testing");
        // AccountTxnID is system-generated - just verify it's 32 bytes
        assert_eq!(account_txn_id.0.len(), 32);
        let _ = trace_data("  AccountTxnID:", &account_txn_id.0, DataRepr::AsHex);

        // Trace `AMMID` (optional - only present on AMM AccountRoot entries)
        // Note: This is a regular account, not an AMM account, so AMMID should be None
        // The AMM we created has its own separate AccountRoot with an AMMID
        let amm_id_opt = account.amm_id().unwrap();
        assert_eq!(
            amm_id_opt, None,
            "AMMID should be None (not an AMM account)"
        );
        let _ = trace("  AMMID: None (not an AMM account)");

        // Trace the `Balance` (required)
        let balance_amount = account
            .balance()
            .unwrap()
            .expect("Balance should be present");
        match balance_amount {
            Amount::XRP { num_drops } => {
                // Balance is system-generated, just verify it's reasonable
                let _ = trace_num("  Balance of Account Finishing the Escrow:", num_drops);
            }
            Amount::IOU { .. } => {
                panic!("IOU Balance encountered, but should have been XRP.")
            }
            Amount::MPT { .. } => {
                panic!("MPT Balance encountered, but should have been XRP.")
            }
        }
        // Trace and assert the `BurnedNFTokens` (optional)
        let burned_nf_tokens_opt = account.burned_nf_tokens().unwrap();
        let burned_nf_tokens = burned_nf_tokens_opt.unwrap_or(0);
        let _ = trace_num("  BurnedNFTokens:", burned_nf_tokens as i64);
        assert_eq!(burned_nf_tokens, 0, "Expected 0 burned NFTokens");

        // Trace the `Domain` (optional - required for testing)
        let domain_opt = account.domain().unwrap();
        let domain = domain_opt.expect("Domain should be set for testing");
        // Domain is user-set, just verify it exists and has reasonable length
        let _ = trace_data("  Domain:", &domain.data[..domain.len], DataRepr::AsHex);

        // Trace the `EmailHash` (optional - required for testing)
        let email_hash_opt = account.email_hash().unwrap();
        let email_hash = email_hash_opt.expect("EmailHash should be set for testing");
        // EmailHash is 16 bytes (MD5 hash)
        assert_eq!(email_hash.0.len(), 16);
        let _ = trace_data("  EmailHash:", &email_hash.0, DataRepr::AsHex);

        // Trace the `FirstNFTokenSequence` (optional - required for testing)
        let first_nf_token_sequence = account
            .first_nf_token_sequence()
            .unwrap()
            .expect("FirstNFTokenSequence should be set for testing");
        let _ = trace_num("  FirstNFTokenSequence:", first_nf_token_sequence as i64);

        // Trace the `MessageKey` (optional - required for testing)
        let message_key_opt = account.message_key().unwrap();
        let message_key = message_key_opt.expect("MessageKey should be set for testing");
        // MessageKey should be 33 bytes (public key)
        let _ = trace_data(
            "  MessageKey:",
            &message_key.data[..message_key.len],
            DataRepr::AsHex,
        );

        // Trace the `MintedNFTokens` (optional - required for testing)
        let minted_nf_tokens = account
            .minted_nf_tokens()
            .unwrap()
            .expect("MintedNFTokens should be set for testing");
        let _ = trace_num("  MintedNFTokens:", minted_nf_tokens as i64);

        // Trace the `NFTokenMinter` (optional - required for testing)
        let nf_token_minter = account
            .nf_token_minter()
            .unwrap()
            .expect("NFTokenMinter should be set for testing");
        // NFTokenMinter is an AccountID - verify it's 20 bytes
        assert_eq!(nf_token_minter.0.len(), 20);
        let _ = trace_data("  NFTokenMinter:", &nf_token_minter.0, DataRepr::AsHex);

        // Trace the `OwnerCount` (required)
        let owner_count = account.owner_count().unwrap();
        // OwnerCount is system-generated based on owned objects
        let _ = trace_num("  OwnerCount:", owner_count as i64);

        // Trace the `PreviousTxnID` (required)
        let previous_txn_id = account.previous_txn_id().unwrap();
        // PreviousTxnID is system-generated - just verify it's 32 bytes
        assert_eq!(previous_txn_id.0.len(), 32);
        let _ = trace_data("  PreviousTxnID:", &previous_txn_id.0, DataRepr::AsHex);

        // Trace the `PreviousTxnLgrSeq` (required)
        let previous_txn_lgr_seq = account.previous_txn_lgr_seq().unwrap();
        // PreviousTxnLgrSeq is system-generated
        let _ = trace_num("  PreviousTxnLgrSeq:", previous_txn_lgr_seq as i64);

        // Trace the `RegularKey` (optional - required for testing)
        let regular_key = account
            .regular_key()
            .unwrap()
            .expect("RegularKey should be set for testing");
        // RegularKey is an AccountID - verify it's 20 bytes
        assert_eq!(regular_key.0.len(), 20);
        let _ = trace_data("  RegularKey:", &regular_key.0, DataRepr::AsHex);

        // Trace the `Sequence` (required)
        let sequence = account.sequence().unwrap();
        // Sequence is system-generated
        let _ = trace_num("  Sequence:", sequence as i64);

        // Trace the `TicketCount` (optional - required for testing)
        let ticket_count = account
            .ticket_count()
            .unwrap()
            .expect("TicketCount should be set for testing");
        let _ = trace_num("  TicketCount:", ticket_count as i64);

        // Trace the `TickSize` (optional - required for testing)
        let tick_size = account
            .tick_size()
            .unwrap()
            .expect("TickSize should be set for testing");
        // TickSize must be 3-15 if present
        assert!(tick_size >= 3 && tick_size <= 15, "TickSize must be 3-15");
        let _ = trace_num("  TickSize:", tick_size as i64);

        // Trace the `TransferRate` (optional - required for testing)
        let transfer_rate = account
            .transfer_rate()
            .unwrap()
            .expect("TransferRate should be set for testing");
        let _ = trace_num("  TransferRate:", transfer_rate as i64);

        // Trace the `WalletLocator` (optional - required for testing)
        let wallet_locator = account
            .wallet_locator()
            .unwrap()
            .expect("WalletLocator should be set for testing");
        // WalletLocator is a 256-bit value (32 bytes)
        assert_eq!(wallet_locator.0.len(), 32);
        let _ = trace_data("  WalletLocator:", &wallet_locator.0, DataRepr::AsHex);

        let _ = trace("}");
        let _ = trace("");
    }

    let _ = trace("$$$$$ WASM EXECUTION COMPLETE $$$$$");
    1 // <-- Finish the escrow to indicate a successful outcome
}
