#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_wasm_std::core::locator::Locator;
use xrpl_wasm_std::core::types::account_id::AccountID;
use xrpl_wasm_std::core::types::amount::currency_code::CurrencyCode;
use xrpl_wasm_std::core::types::hash_256::Hash256;
use xrpl_wasm_std::core::types::keylets::{
    KeyletBytes, check_keylet, credential_keylet, delegate_keylet, deposit_preauth_keylet,
    did_keylet, escrow_keylet, line_keylet, nft_offer_keylet, offer_keylet, oracle_keylet,
    paychan_keylet, signers_keylet, ticket_keylet,
};
use xrpl_wasm_std::host::trace::{DataRepr, trace, trace_account_buf, trace_data, trace_num};
use xrpl_wasm_std::host::{
    cache_ledger_obj, get_ledger_obj_array_len, get_ledger_obj_field, get_ledger_obj_nested_field,
};
use xrpl_wasm_std::sfield::{
    Account, AccountTxnID, Balance, CredentialType, Domain, EmailHash, Flags, Issuer,
    LedgerEntryType, MessageKey, OwnerCount, PreviousTxnID, PreviousTxnLgrSeq, RegularKey,
    Sequence, Subject, TakerGets, TicketCount, TicketSequence, TransferRate,
};
use xrpl_wasm_std::{decode_hex_20, decode_hex_32, host, sfield};

fn test_account_root() {
    let _ = trace("\n$$$ test_account_root $$$");
    let keylet =
        decode_hex_32(b"13F1A95D7AAB7108D5CE7EEAF504B2894B8C674E6D68499076441C4837282BF8").unwrap();

    let slot = unsafe { cache_ledger_obj(keylet.as_ptr(), keylet.len(), 0) };

    let mut out_buf = [0u8; 20];
    let out_len = unsafe {
        get_ledger_obj_field(slot, Account, out_buf.as_mut_ptr(), out_buf.len()) as usize
    };
    let _ = trace_account_buf("  Account:", &out_buf);

    let mut out_buf = [0u8; 32];
    let out_len = unsafe {
        get_ledger_obj_field(slot, AccountTxnID, out_buf.as_mut_ptr(), out_buf.len()) as usize
    };
    let _ = trace_data("  AccountTxnID:", &out_buf[0..out_len], DataRepr::AsHex);

    let mut out_buf = [0u8; 48];
    let out_len = unsafe {
        get_ledger_obj_field(slot, Balance, out_buf.as_mut_ptr(), out_buf.len()) as usize
    };
    let _ = trace_data("  Balance:", &out_buf[0..out_len], DataRepr::AsHex);

    let mut out_buf = [0u8; 20];
    let out_len =
        unsafe { get_ledger_obj_field(slot, Domain, out_buf.as_mut_ptr(), out_buf.len()) as usize };
    let _ = trace_data("  Domain:", &out_buf[0..out_len], DataRepr::AsHex);

    let mut out_buf = [0u8; 16];
    let out_len = unsafe {
        get_ledger_obj_field(slot, EmailHash, out_buf.as_mut_ptr(), out_buf.len()) as usize
    };
    let _ = trace_data("  EmailHash:", &out_buf[0..out_len], DataRepr::AsHex);

    let mut out_buf = 0i32;
    let out_len = unsafe {
        get_ledger_obj_field(slot, Flags, (&mut out_buf) as *mut i32 as *mut u8, 4) as usize
    };
    let _ = trace_num("  Flags:", out_buf as i64);

    let mut out_buf = 0i16;
    let out_len = unsafe {
        get_ledger_obj_field(
            slot,
            LedgerEntryType,
            (&mut out_buf) as *mut i16 as *mut u8,
            2,
        ) as usize
    };
    let _ = trace_num("  LedgerEntryType:", out_buf as i64);

    let mut out_buf = [0u8; 32];
    let out_len = unsafe {
        get_ledger_obj_field(slot, MessageKey, out_buf.as_mut_ptr(), out_buf.len()) as usize
    };
    let _ = trace_data("  MessageKey:", &out_buf[0..out_len], DataRepr::AsHex);

    let mut out_buf = 0i32;
    let out_len = unsafe {
        get_ledger_obj_field(slot, OwnerCount, (&mut out_buf) as *mut i32 as *mut u8, 4) as usize
    };
    let _ = trace_num("  OwnerCount:", out_buf as i64);

    let mut out_buf = [0u8; 32];
    let out_len = unsafe {
        get_ledger_obj_field(slot, PreviousTxnID, out_buf.as_mut_ptr(), out_buf.len()) as usize
    };
    let _ = trace_data("  PreviousTxnID:", &out_buf[0..out_len], DataRepr::AsHex);

    let mut out_buf = 0i32;
    let out_len = unsafe {
        get_ledger_obj_field(
            slot,
            PreviousTxnLgrSeq,
            (&mut out_buf) as *mut i32 as *mut u8,
            4,
        ) as usize
    };
    let _ = trace_num("  PreviousTxnLgrSeq:", out_buf as i64);

    let mut out_buf = [0u8; 20];
    let out_len = unsafe {
        get_ledger_obj_field(slot, RegularKey, out_buf.as_mut_ptr(), out_buf.len()) as usize
    };
    let _ = trace_account_buf("  RegularKey:", &out_buf);

    let mut out_buf = 0i32;
    let out_len = unsafe {
        get_ledger_obj_field(slot, Sequence, (&mut out_buf) as *mut i32 as *mut u8, 4) as usize
    };
    let _ = trace_num("  Sequence:", out_buf as i64);

    let mut out_buf = 0i32;
    let out_len = unsafe {
        get_ledger_obj_field(slot, TicketCount, (&mut out_buf) as *mut i32 as *mut u8, 4) as usize
    };
    let _ = trace_num("  TicketCount:", out_buf as i64);

    let mut out_buf = 0i64;
    let out_len = unsafe {
        get_ledger_obj_field(slot, TransferRate, (&mut out_buf) as *mut i64 as *mut u8, 4) as usize
    };
    let _ = trace_num("  TransferRate:", out_buf);

    //TODO urlgravatar is not an sfield, double check
}

fn test_amendments() {
    let _ = trace("\n$$$ test_amendments $$$");
    let keylet =
        decode_hex_32(b"7DB0788C020F02780A673DC74757F23823FA3014C1866E72CC4CD8B226CD6EF4").unwrap();

    let slot = unsafe { cache_ledger_obj(keylet.as_ptr(), keylet.len(), 0) };

    let array_len = unsafe { get_ledger_obj_array_len(slot, sfield::Amendments) };
    let _ = trace_num("  Amendments array len:", array_len as i64);
    for i in 0..if array_len > 2 { 2 } else { array_len } {
        let mut buf = [0x00; 32];
        let mut locator = Locator::new();
        locator.pack(sfield::Amendments);
        locator.pack(i);
        let output_len = unsafe {
            get_ledger_obj_nested_field(
                slot,
                locator.get_addr(),
                locator.num_packed_bytes(),
                buf.as_mut_ptr(),
                buf.len(),
            )
        };
        let _ = trace_data("  Amendment:", &buf[..output_len as usize], DataRepr::AsHex);
    }

    let mut out_buf = 0i16;
    let out_len = unsafe {
        get_ledger_obj_field(
            slot,
            LedgerEntryType,
            (&mut out_buf) as *mut i16 as *mut u8,
            2,
        ) as usize
    };
    let _ = trace_num("  LedgerEntryType:", out_buf as i64);

    let mut buf = [0x00; 32];
    let mut locator = Locator::new();
    locator.pack(sfield::Majorities);
    locator.pack(0);
    locator.pack(sfield::Majority);
    locator.pack(sfield::Amendment);
    let output_len = unsafe {
        get_ledger_obj_nested_field(
            slot,
            locator.get_addr(),
            locator.num_packed_bytes(),
            buf.as_mut_ptr(),
            buf.len(),
        )
    };
    let _ = trace_data(
        "  Majority Amendment:",
        &buf[..output_len as usize],
        DataRepr::AsHex,
    );

    locator.repack_last(sfield::CloseTime);
    let mut out_buf = 0i64;
    let out_len = unsafe {
        get_ledger_obj_nested_field(
            slot,
            locator.get_addr(),
            locator.num_packed_bytes(),
            (&mut out_buf) as *mut i64 as *mut u8,
            4,
        ) as usize
    };
    let _ = trace_num("  Majority CloseTime:", out_buf);
}

fn test_amm() {
    let _ = trace("\n$$$ test_amm $$$");

    let keylet =
        decode_hex_32(b"97DD92D4F3A791254A530BA769F6669DEBF6B2FC8CCA46842B9031ADCD4D1ADA").unwrap();

    let slot = unsafe { cache_ledger_obj(keylet.as_ptr(), keylet.len(), 0) };

    let mut buf = [0x00; 48];
    let output_len =
        unsafe { get_ledger_obj_field(slot, sfield::LPTokenBalance, buf.as_mut_ptr(), buf.len()) };
    let _ = trace_data(
        "  get LPTokenBalance:",
        &buf[..output_len as usize],
        DataRepr::AsHex,
    );

    let mut locator = Locator::new();
    locator.pack(sfield::AuctionSlot);
    locator.pack(sfield::Price);
    let output_len = unsafe {
        get_ledger_obj_nested_field(
            slot,
            locator.get_addr(),
            locator.num_packed_bytes(),
            buf.as_mut_ptr(),
            buf.len(),
        )
    };
    let _ = trace_data(
        "  AuctionSlot Price:",
        &buf[..output_len as usize],
        DataRepr::AsHex,
    );
}

fn test_check() {
    let _ = trace("\n$$$ test_check $$$");
    let (slot, keylet) =
        get_slot(b"F23D83EA49474537F7A15EF79DD140DEAE2CD0F4BF2D6383979C863100F9660F");
    let acc = get_account(slot, Account);

    let mut sqn_buf = 0i32;
    let sqn_len = unsafe {
        get_ledger_obj_field(slot, Sequence, (&mut sqn_buf) as *mut i32 as *mut u8, 4) as usize
    };
    let _ = trace_num("  Sequence:", sqn_buf as i64);
    process_keylet_result(check_keylet(&acc, sqn_buf), keylet);
}

fn test_credential() {
    let _ = trace("\n$$$ test_credential $$$");
    let (slot, keylet) =
        get_slot(b"DC2FCCBB773244E981576CF509E2463B435F5B46F3EF4684E2ED2EC9C575A110");
    let slot = unsafe { cache_ledger_obj(keylet.as_ptr(), keylet.len(), 0) };
    let iss = get_account(slot, Issuer);
    let subj = get_account(slot, Subject);

    let mut cred_type_buf = [0u8; 32];
    let len = unsafe {
        get_ledger_obj_field(
            slot,
            CredentialType,
            cred_type_buf.as_mut_ptr(),
            cred_type_buf.len(),
        ) as usize
    };
    let _ = trace_data("  CredentialType:", &cred_type_buf[0..len], DataRepr::AsHex);

    process_keylet_result(
        credential_keylet(&subj, &iss, &cred_type_buf[0..len]),
        keylet,
    );
}

fn test_delegate() {
    let _ = trace("\n$$$ test_delegate $$$");
    let (slot, keylet) =
        get_slot(b"572E861CF66227EC90CE789014531EE3336D28C411BA8E18F660B65F1BE68B49");
    let acc = get_account(slot, Account);
    let auth = get_account(slot, sfield::Authorize);
    process_keylet_result(delegate_keylet(&acc, &auth), keylet);
}

fn test_deposit_preauth() {
    let _ = trace("\n$$$ test_deposit_preauth $$$");
    let (slot, keylet) =
        get_slot(b"A43898B685C450DE8E194B24D9D54E62530536A770CCB311BFEE15A27381ABB2");
    let acc = get_account(slot, Account);
    let auth = get_account(slot, sfield::Authorize);
    process_keylet_result(deposit_preauth_keylet(&acc, &auth), keylet);
}

fn test_did() {
    let _ = trace("\n$$$ test_did $$$");
    let (slot, keylet) =
        get_slot(b"C3535E58FC564D3B4FBD67DDEA367ABA5C6A97901E7D14F1A316AA492E999D92");
    let acc = get_account(slot, Account);
    process_keylet_result(did_keylet(&acc), keylet);
}

fn test_escrow() {
    let _ = trace("\n$$$ test_escrow $$$");
    let (slot, keylet) =
        get_slot(b"41CF1FD65F1A10642BB9ED3258B36B130D9C6EB12B2175A6F3137665AF12B9FD");
    let acc = get_account(slot, Account);
    let sqn = 4882021i32;
    process_keylet_result(escrow_keylet(&acc, sqn), keylet);
}

fn test_issue() {
    let _ = trace("\n$$$ test_issue $$$");

    let keylet =
        decode_hex_32(b"4444444444444444444444444444444444444444444444444444444444444444").unwrap();
    let slot = unsafe { cache_ledger_obj(keylet.as_ptr(), keylet.len(), 0) };

    // XRP
    let mut buf = [0x00; 20];
    let output_len =
        unsafe { get_ledger_obj_field(slot, sfield::Asset, buf.as_mut_ptr(), buf.len()) };
    let _ = trace_data("  XRP Asset:", &buf[..output_len as usize], DataRepr::AsHex);

    // MPT
    let mut buf = [0x00; 24];
    let output_len =
        unsafe { get_ledger_obj_field(slot, sfield::Asset2, buf.as_mut_ptr(), buf.len()) };
    let _ = trace_data("  MPT Asset:", &buf[..output_len as usize], DataRepr::AsHex);

    let keylet =
        decode_hex_32(b"97DD92D4F3A791254A530BA769F6669DEBF6B2FC8CCA46842B9031ADCD4D1ADA").unwrap();
    let slot = unsafe { cache_ledger_obj(keylet.as_ptr(), keylet.len(), 0) };

    //IOU
    let mut buf = [0x00; 40];
    let output_len =
        unsafe { get_ledger_obj_field(slot, sfield::Asset2, buf.as_mut_ptr(), buf.len()) };
    let _ = trace_data("  IOU Asset:", &buf[..output_len as usize], DataRepr::AsHex);
}

fn test_line() {
    let _ = trace("\n$$$ test_line $$$");
    let (slot, keylet) =
        get_slot(b"E5B2858F0D15ECB4622E7C613285CA0E84BF7209466A5D1FAA6606B7E6DAAA72");
    let slot = unsafe { cache_ledger_obj(keylet.as_ptr(), keylet.len(), 0) };

    let mut buf = [0x00; 48];
    let output_len =
        unsafe { get_ledger_obj_field(slot, sfield::LowLimit, buf.as_mut_ptr(), buf.len()) };
    let _ = trace_data("  LowLimit:", &buf[..output_len as usize], DataRepr::AsHex);
    let data: [u8; 20] = buf[28..48].try_into().unwrap();
    let acc1 = AccountID::from(data);

    let output_len =
        unsafe { get_ledger_obj_field(slot, sfield::HighLimit, buf.as_mut_ptr(), buf.len()) };
    let _ = trace_data("  HighLimit:", &buf[..output_len as usize], DataRepr::AsHex);
    let data: [u8; 20] = buf[28..48].try_into().unwrap();
    let acc2 = AccountID::from(data);

    let output_len = unsafe { get_ledger_obj_field(slot, Balance, buf.as_mut_ptr(), buf.len()) };
    let _ = trace_data("  Balance:", &buf[..output_len as usize], DataRepr::AsHex);
    let data: [u8; 20] = buf[8..28].try_into().unwrap();
    let currency = CurrencyCode::from(data);

    process_keylet_result(line_keylet(&acc1, &acc2, &currency), keylet);
}

fn test_mpt_amount() {
    let _ = trace("\n$$$ test_mpt_amount, access an MPT Amount $$$");

    let keylet =
        decode_hex_32(b"4444444444444444444444444444444444444444444444444444444444444444").unwrap();

    let slot = unsafe { cache_ledger_obj(keylet.as_ptr(), keylet.len(), 0) };

    let mut buf = [0x00; 48];
    let output_len =
        unsafe { get_ledger_obj_field(slot, sfield::Amount2, buf.as_mut_ptr(), buf.len()) };
    let _ = trace_data(
        "  MPT Amount2:",
        &buf[..output_len as usize],
        DataRepr::AsHex,
    );
}

fn test_mpt_fields() {
    let _ = trace("\n$$$ test_mpt_fields, access individual fields $$$");

    let keylet =
        decode_hex_32(b"22F99DCD55BCCF3D68DC3E4D6CF12602006A7563A6BE93FC57FD63298BCCEB13").unwrap();

    let slot = unsafe { cache_ledger_obj(keylet.as_ptr(), keylet.len(), 0) };

    let mut buf = [0x00; 24];
    let output_len = unsafe {
        get_ledger_obj_field(slot, sfield::MPTokenIssuanceID, buf.as_mut_ptr(), buf.len())
    };
    let _ = trace_data(
        "  MPTokenIssuanceID:",
        &buf[..output_len as usize],
        DataRepr::AsHex,
    );

    let mut value = 0u64;
    let output_len = unsafe {
        get_ledger_obj_field(
            slot,
            sfield::MPTAmount,
            (&mut value) as *mut u64 as *mut u8,
            8,
        )
    };
    let _ = trace_num("  MPTAmount:", value as i64);
}

fn test_nft_offer() {
    let _ = trace("\n$$$ test_nft_offer $$$");
    let (slot, keylet) =
        get_slot(b"E7DB3BE2E00EA4BFD61B43B96D96EF627823A0C093F14A95E3AD68708B63696B");
    let acc = get_account(slot, sfield::Owner);
    let sqn = 4882024i32;
    process_keylet_result(nft_offer_keylet(&acc, sqn), keylet);
}

fn test_offer() {
    let _ = trace("\n$$$ test_offer $$$");
    let (slot, keylet) =
        get_slot(b"D0A063DEE0B0EC9522CF35CD55771B5DCAFA19A133EE46A0295E4D089AF86438");

    let mut buf = [0x00; 48];
    let output_len =
        unsafe { get_ledger_obj_field(slot, sfield::TakerPays, buf.as_mut_ptr(), buf.len()) };
    let _ = trace_data("  TakerPays:", &buf[..output_len as usize], DataRepr::AsHex);

    let acc = get_account(slot, Account);

    let mut sqn_buf = 0i32;
    let sqn_len = unsafe {
        get_ledger_obj_field(slot, Sequence, (&mut sqn_buf) as *mut i32 as *mut u8, 4) as usize
    };
    let _ = trace_num("  Sequence:", sqn_buf as i64);

    process_keylet_result(offer_keylet(&acc, sqn_buf), keylet);
}

fn test_oracle() {
    let _ = trace("\n$$$ test_oracle $$$");
    let (slot, keylet) =
        get_slot(b"FB0D2E2B5C7240772C2D2A3E7B99BF44EA955C7C4977A368F704FE879F0E0612");
    let acc = get_account(slot, sfield::Owner);
    let sqn = 1234i32;
    process_keylet_result(oracle_keylet(&acc, sqn), keylet);
}

fn test_pay_channel() {
    let _ = trace("\n$$$ test_pay_channel $$$");
    let (slot, keylet) =
        get_slot(b"C7F634794B79DB40E87179A9D1BF05D05797AE7E92DF8E93FD6656E8C4BE3AE7");
    let acc = get_account(slot, Account);
    let dest = get_account(slot, sfield::Destination);
    let sqn = 382i32; //got from RPC the mainnet
    process_keylet_result(paychan_keylet(&acc, &dest, sqn), keylet);
}

fn test_signers() {
    let _ = trace("\n$$$ test_signers $$$");
    let (_, keylet) = get_slot(b"A6D94867F6EA7D78BDD941DB2A57E4797288EAE5BB93423FA10F8602577B0504");
    let acc = AccountID::from(decode_hex_20(b"7AED8B5479456A3491033E9EC80879CDA5104AD6").unwrap());
    process_keylet_result(signers_keylet(&acc), keylet);
}

fn test_ticket() {
    let _ = trace("\n$$$ test_ticket $$$");
    let (slot, keylet) =
        get_slot(b"B603682BC36F474F708E1A150B7C034C6C13D838C3F2F135CDB7BEA6E5B5ACEF");
    let acc = get_account(slot, Account);

    let mut sqn_buf = 0i32;
    let sqn_len = unsafe {
        get_ledger_obj_field(
            slot,
            TicketSequence,
            (&mut sqn_buf) as *mut i32 as *mut u8,
            4,
        ) as usize
    };
    let _ = trace_num("  TicketSequence:", sqn_buf as i64);
    process_keylet_result(ticket_keylet(&acc, sqn_buf), keylet);
}

fn get_slot(keylet_hex: &[u8; 64]) -> (i32, [u8; 32]) {
    let keylet = decode_hex_32(keylet_hex).unwrap();
    (
        unsafe { cache_ledger_obj(keylet.as_ptr(), keylet.len(), 0) },
        keylet,
    )
}

fn get_account(slot: i32, field: i32) -> AccountID {
    let mut acc_buf = [0u8; 20];
    let acc_len =
        unsafe { get_ledger_obj_field(slot, field, acc_buf.as_mut_ptr(), acc_buf.len()) as usize };
    let _ = trace_account_buf("  accountID:", &acc_buf);
    AccountID::from(acc_buf)
}

fn process_keylet_result(result: host::Result<KeyletBytes>, expected: [u8; 32]) {
    match result {
        host::Result::Ok(computed_keylet) => {
            if computed_keylet == expected {
                let _ = trace("  keylet match");
            } else {
                let _ = trace("  keylet doesn't match: ");
                let _ = trace_data("    computed:", &computed_keylet[..], DataRepr::AsHex);
                let _ = trace_data("    expected:", &expected[..], DataRepr::AsHex);
            }
        }
        host::Result::Err(_) => {
            let _ = trace("  keylet function error");
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    test_account_root();
    test_amendments();
    test_amm();
    test_check();
    test_credential();
    test_delegate();
    test_deposit_preauth();
    test_did();
    test_escrow();
    test_issue();
    test_line();
    test_mpt_amount();
    test_mpt_fields();
    test_nft_offer();
    test_offer();
    test_oracle();
    test_pay_channel();
    test_signers();
    test_ticket();

    1 // <-- Finish the escrow to indicate a successful outcome
}
