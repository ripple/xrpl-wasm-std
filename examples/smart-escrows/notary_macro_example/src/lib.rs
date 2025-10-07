#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_wasm_std::core::current_tx::escrow_finish;
use xrpl_wasm_std::core::current_tx::traits::TransactionCommonFields;
use xrpl_wasm_std::host::trace::trace_num;
use xrpl_wasm_std::host::{Result::Err, Result::Ok};
use xrpl_wasm_std::r_address;

// The r_address! macro converts the address at compile time to a [u8; 20] array.
// This means zero runtime overhead - the final WASM binary contains only the raw bytes.
// Multiple accounts can be defined this way:
const NOTARY_ACCOUNT: [u8; 20] = r_address!("rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh");

// You could also define multiple accounts easily:
// const BACKUP_NOTARY: [u8; 20] = r_address!("rN7n7otQDd6FczFgLdSqtcsAUxDkw6fzRH");
// const ADMIN_ACCOUNT: [u8; 20] = r_address!("rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn");

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let escrow_finish = escrow_finish::get_current_escrow_finish();
    let tx_account = match escrow_finish.get_account() {
        Ok(v) => v,
        Err(e) => {
            let _ = trace_num("Error in Notary contract", e.code() as i64);
            return e.code();
        }
    };

    (tx_account.0 == NOTARY_ACCOUNT) as i32 // <-- Finish the escrow to indicate a successful outcome
}
