#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_wasm_std::core::ledger_objects::current_escrow;
use xrpl_wasm_std::core::ledger_objects::traits::CurrentEscrowFields;
use xrpl_wasm_std::core::locator::Locator;
use xrpl_wasm_std::core::types::nft::{NFTID_SIZE, NFToken};
use xrpl_wasm_std::host::Error::InternalError;
use xrpl_wasm_std::host::get_tx_nested_field;
use xrpl_wasm_std::host::trace::{DataRepr, trace_data, trace_num};
use xrpl_wasm_std::host::{Error, Result, Result::Err, Result::Ok};
use xrpl_wasm_std::sfield;
use xrpl_wasm_std::types::{ContractData, XRPL_CONTRACT_DATA_SIZE};

#[unsafe(no_mangle)]
pub fn get_first_memo() -> Result<Option<ContractData>> {
    let mut data: ContractData = [0; XRPL_CONTRACT_DATA_SIZE];
    let mut locator = Locator::new();
    locator.pack(sfield::Memos);
    locator.pack(0);
    locator.pack(sfield::MemoData);
    let result_code = unsafe {
        get_tx_nested_field(
            locator.get_addr(),
            locator.num_packed_bytes(),
            data.as_mut_ptr(),
            data.len(),
        )
    };

    match result_code {
        result_code if result_code > 0 => {
            Ok(Some(data)) // <-- Move the buffer into an AccountID
        }
        0 => Err(InternalError),
        result_code => Err(Error::from_code(result_code)),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let memo: ContractData = match get_first_memo() {
        Ok(v) => {
            match v {
                Some(v) => v,
                None => return 0, // <-- Do not execute the escrow.
            }
        }
        Err(e) => {
            let _ = trace_num("Error getting first memo:", e.code() as i64);
            return e.code(); // <-- Do not execute the escrow.
        }
    };

    // Extract NFT ID from memo (first 32 bytes) and create NFToken
    let nft_id_bytes: [u8; NFTID_SIZE] = memo[0..32].try_into().unwrap();
    let nft_token = NFToken::new(nft_id_bytes);
    let _ = trace_data("NFT ID from memo:", nft_token.as_bytes(), DataRepr::AsHex);

    // Demonstrate NFToken field extraction
    if let Ok(flags) = nft_token.flags() {
        let _ = trace_num("NFT Flags:", flags as i64);
    }
    if let Ok(transfer_fee) = nft_token.transfer_fee() {
        let _ = trace_num("NFT Transfer Fee:", transfer_fee as i64);
    }
    if let Ok(issuer) = nft_token.issuer() {
        let _ = trace_data("NFT Issuer:", &issuer.0, DataRepr::AsHex);
    }
    if let Ok(taxon) = nft_token.taxon() {
        let _ = trace_num("NFT Taxon:", taxon as i64);
    }
    if let Ok(serial) = nft_token.serial() {
        let _ = trace_num("NFT Serial:", serial as i64);
    }

    let current_escrow = current_escrow::get_current_escrow();
    let destination = match current_escrow.get_destination() {
        Ok(destination) => destination,
        Err(e) => {
            let _ = trace_num("Error getting current ledger destination:", e.code() as i64);
            return e.code(); // <-- Do not execute the escrow.
        }
    };

    // Check if destination owns the NFT by attempting to retrieve its URI
    match nft_token.uri(&destination) {
        Ok(_uri) => {
            let _ = trace_data("NFT is owned by destination", &[], DataRepr::AsHex);
            1 // <-- Finish the escrow successfully
        }
        Err(e) => {
            let _ = trace_num("NFT is NOT owned by destination. Error code:", e.code() as i64);
            0 // <-- Do not execute the escrow
        }
    }
}
