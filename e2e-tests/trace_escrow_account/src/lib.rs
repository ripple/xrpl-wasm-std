#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_wasm_std::core::current_tx::escrow_finish::{EscrowFinish, get_current_escrow_finish};
use xrpl_wasm_std::core::current_tx::traits::TransactionCommonFields;
use xrpl_wasm_std::core::ledger_objects::account_root::{AccountRoot, get_account_balance};
use xrpl_wasm_std::core::ledger_objects::traits::{AccountFields, LedgerObjectCommonFields};
use xrpl_wasm_std::core::types::account_id::AccountID;
use xrpl_wasm_std::core::types::amount::token_amount::TokenAmount;
use xrpl_wasm_std::host::cache_ledger_obj;
use xrpl_wasm_std::host::trace::{DataRepr, trace, trace_data, trace_num};
use xrpl_wasm_std::{assert_eq, decode_hex_32};

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let _ = trace("$$$$$ STARTING WASM EXECUTION $$$$$");
    let _ = trace("");

    // The transaction prompting execution of this contract.
    let escrow_finish: EscrowFinish = get_current_escrow_finish();

    // ########################################
    // Step #1 [EscrowFinish Account]: Trace Current Balance
    // ########################################
    {
        let _ = trace("### Step #1: Trace Account Balance for Account Finishing the Escrow");
        let _ = trace("{ ");
        let account: AccountID = escrow_finish.get_account().unwrap();
        let balance = match get_account_balance(&account).unwrap().unwrap() {
            TokenAmount::XRP { num_drops } => num_drops,
            TokenAmount::IOU { .. } => {
                panic!("IOU Balance encountered, but should have been XRP.")
            }
            TokenAmount::MPT { .. } => {
                panic!("MPT Balance encountered, but should have been XRP.")
            }
        };

        // GET some other field... can't due to not knowing

        let _ = trace_num("  Balance of Account Finishing the Escrow:", balance);
        assert_eq!(balance, 9999999988);
        let _ = trace("}");
        let _ = trace("");
    }

    // ########################################
    // Step #2 [Arbitrary Ledger Object]: Trace AccountRoot Fields.
    // ########################################
    {
        // Slot the account
        // "Account": "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh",
        let account_keylet =
            decode_hex_32(b"2B6AC232AA4C4BE41BF49D2459FA4A0347E1B543A4C92FCEE0821C0201E2E9A8")
                .unwrap();

        // Try to cache the ledger object inside rippled
        let slot = unsafe { cache_ledger_obj(account_keylet.as_ptr(), 32, 0) };
        if slot <= 0 {
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
        assert_eq!(flags, 0);
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
        let _ = trace_data("  Account:", &account_id.0, DataRepr::AsHex);
        assert_eq!(
            account_id.0,
            [
                0xB5, 0xF7, 0x62, 0x79, 0x8A, 0x53, 0xD5, 0x43, 0xA0, 0x14, 0xCA, 0xF8, 0xB2, 0x97,
                0xCF, 0xF8, 0xF2, 0xF9, 0x37, 0xE8
            ]
        );

        // Trace the `AccountTxnID`
        let account_txn_id = account.account_txn_id().unwrap().unwrap();
        let _ = trace_data("  AccountTxnID:", &account_txn_id.0, DataRepr::AsHex);
        assert_eq!(
            account_txn_id.0,
            [
                0xBC, 0x8E, 0x8B, 0x46, 0xD1, 0xC4, 0x03, 0xB1, 0x68, 0xEE, 0x64, 0x02, 0x76, 0x90,
                0x65, 0xEB, 0xDA, 0xD7, 0x8E, 0x5E, 0xA3, 0xA0, 0x43, 0xD8, 0xE0, 0x41, 0x37, 0x2E,
                0xDF, 0x14, 0xA1, 0x1E
            ]
        );

        // Trace `AMMID`
        let amm_id = account.amm_id().unwrap().unwrap();
        let _ = trace_data("  AMMID:", &amm_id.0, DataRepr::AsHex);
        assert_eq!(
            amm_id.0,
            [
                0xBC, 0x8E, 0x8B, 0x46, 0xD1, 0xC4, 0x03, 0xB1, 0x68, 0xEE, 0x64, 0x02, 0x76, 0x90,
                0x65, 0xEB, 0xDA, 0xD7, 0x8E, 0x5E, 0xA3, 0xA0, 0x43, 0xD8, 0xE0, 0x41, 0x37, 0x2E,
                0xDF, 0x14, 0xA1, 0x1E
            ]
        );

        // Trace the `Balance`
        let balance = match account.balance().unwrap().unwrap() {
            TokenAmount::XRP { num_drops } => num_drops,
            TokenAmount::IOU { .. } => {
                panic!("IOU Balance encountered, but should have been XRP.")
            }
            TokenAmount::MPT { .. } => {
                panic!("MPT Balance encountered, but should have been XRP.")
            }
        };
        let _ = trace_num("  Balance of arbitrary Account:", balance);
        assert_eq!(balance, 55426479402);

        // Trace the `BurnedNFTokens`
        let burned_nf_tokens = account.burned_nf_tokens().unwrap().unwrap();
        let _ = trace_num("  BurnedNFTokens:", burned_nf_tokens as i64);
        assert_eq!(burned_nf_tokens, 0);

        // Trace the `Domain`
        let domain = account.domain().unwrap().unwrap();
        assert_eq!(&domain.data[..domain.len], &[0xC8, 0xE8, 0xB4, 0x6E]);
        let _ = trace_data("  Domain:", &domain.data[..domain.len], DataRepr::AsHex);

        // Trace the `EmailHash`
        let email_hash = account.email_hash().unwrap().unwrap();
        assert_eq!(
            email_hash.0,
            [
                0xBC, 0x8E, 0x8B, 0x46, 0xD1, 0xC4, 0x03, 0xB1, 0x68, 0xEE, 0x64, 0x02, 0x76, 0x90,
                0x65, 0xEB
            ]
        );
        let _ = trace_data("  EmailHash:", &email_hash.0, DataRepr::AsHex);

        // Trace the `FirstNFTokenSequence`
        let first_nf_token_sequence = account.first_nf_token_sequence().unwrap().unwrap();
        assert_eq!(first_nf_token_sequence, 21);
        let _ = trace_num("  FirstNFTokenSequence:", first_nf_token_sequence as i64);

        // Trace the `MessageKey`
        let message_key = account.message_key().unwrap().unwrap();
        assert_eq!(
            &message_key.data[..message_key.len],
            &[0xC8, 0xE8, 0xB4, 0x6D]
        );
        let _ = trace_data(
            "  MessageKey:",
            &message_key.data[..message_key.len],
            DataRepr::AsHex,
        );

        // Trace the `MintedNFTokens`
        let minted_nf_tokens = account.minted_nf_tokens().unwrap().unwrap();
        assert_eq!(minted_nf_tokens, 22);
        let _ = trace_num("  MintedNFTokens:", minted_nf_tokens as i64);

        // Trace the `NFTokenMinter`
        let nf_token_minter = account.nf_token_minter().unwrap().unwrap();
        assert_eq!(
            nf_token_minter.0,
            [
                0xB5, 0xF7, 0x62, 0x79, 0x8A, 0x53, 0xD5, 0x43, 0xA0, 0x14, 0xCA, 0xF8, 0xB2, 0x97,
                0xCF, 0xF8, 0xF2, 0xF9, 0x37, 0xE8
            ]
        );
        let _ = trace_data("  NFTokenMinter:", &nf_token_minter.0, DataRepr::AsHex);

        // Trace the `OwnerCount`
        let owner_count = account.owner_count().unwrap();
        assert_eq!(owner_count, 1);
        let _ = trace_num("  OwnerCount:", owner_count as i64);

        // Trace the `PreviousTxnID`
        let previous_txn_id = account.previous_txn_id().unwrap();
        assert_eq!(
            previous_txn_id.0,
            [
                0xBC, 0x8E, 0x8B, 0x46, 0xD1, 0xC4, 0x03, 0xB1, 0x68, 0xEE, 0x64, 0x02, 0x76, 0x90,
                0x65, 0xEB, 0xDA, 0xD7, 0x8E, 0x5E, 0xA3, 0xA0, 0x43, 0xD8, 0xE0, 0x41, 0x37, 0x2E,
                0xDF, 0x14, 0xA1, 0x1F,
            ]
        );
        let _ = trace_data("  PreviousTxnID:", &previous_txn_id.0, DataRepr::AsHex);

        // Trace the `PreviousTxnLgrSeq`
        let previous_txn_lgr_seq = account.previous_txn_lgr_seq().unwrap();
        assert_eq!(previous_txn_lgr_seq, 95945324);
        let _ = trace_num("  PreviousTxnLgrSeq:", previous_txn_lgr_seq as i64);

        // Trace the `RegularKey`
        let regular_key = account.regular_key().unwrap().unwrap();
        assert_eq!(
            regular_key.0,
            [
                0x76, 0x1B, 0x18, 0xF3, 0x46, 0x11, 0x2D, 0xFC, 0xD6, 0xA9, 0x95, 0x92, 0x94, 0xE9,
                0xE9, 0x5D, 0x02, 0xDB, 0x7E, 0xE1
            ]
        );
        let _ = trace_data("  RegularKey:", &regular_key.0, DataRepr::AsHex);

        // Trace the `Sequence`
        let sequence = account.sequence().unwrap();
        assert_eq!(sequence, 44196);
        let _ = trace_num("  Sequence:", sequence as i64);

        // Trace the `TicketCount`
        let ticket_count = account.ticket_count().unwrap().unwrap();
        assert_eq!(ticket_count, 23);
        let _ = trace_num("  TicketCount:", ticket_count as i64);

        // Trace the `TickSize`
        let tick_size = account.tick_size().unwrap().unwrap();
        assert_eq!(tick_size, 24);
        let _ = trace_num("  TickSize:", tick_size as i64);

        // Trace the `TransferRate`
        let transfer_rate = account.transfer_rate().unwrap().unwrap();
        assert_eq!(transfer_rate, 1220000000);
        let _ = trace_num("  TransferRate:", transfer_rate as i64);

        // Trace the `WalletLocator`
        let wallet_locator = account.wallet_locator().unwrap().unwrap();
        assert_eq!(
            &wallet_locator.0,
            &[
                0xBC, 0x8E, 0x8B, 0x46, 0xD1, 0xC4, 0x03, 0xB1, 0x68, 0xEE, 0x64, 0x02, 0x76, 0x90,
                0x65, 0xEB, 0xDA, 0xD7, 0x8E, 0x5E, 0xA3, 0xA0, 0x43, 0xD8, 0xE0, 0x41, 0x37, 0x2E,
                0xDF, 0x14, 0xA1, 0x1D,
            ]
        );
        let _ = trace_data("  WalletLocator:", &wallet_locator.0, DataRepr::AsHex);

        // Trace the `WalletSize`
        let wallet_size = account.wallet_size().unwrap().unwrap();
        assert_eq!(wallet_size, 25);
        let _ = trace_num("  WalletSize:", wallet_size as i64);

        let _ = trace("}");
        let _ = trace("");
    }

    let _ = trace("$$$$$ WASM EXECUTION COMPLETE $$$$$");
    1 // <-- Finish the escrow to indicate a successful outcome
}
