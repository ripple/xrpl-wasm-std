#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_parameter_macro::wasm_export;
use xrpl_wasm_std::core::types::account_id::AccountID;
use xrpl_wasm_std::core::types::amount::Amount;
use xrpl_wasm_std::host::trace::{trace, trace_num};

const SUCCESS: i32 = 0;
const BAD_PARAM: i32 = -1;
const MAX_LIMIT: i32 = -2;

fn exit(message: &str, error_code: i32) -> i32 {
    let _ = trace(message);
    let _ = trace_num("Error Code:", error_code as i64);
    error_code
}

#[wasm_export(
    exit = exit,
    instance(initialBalance: Amount)
)]
fn my_function(
    account: AccountID,
    amount: Amount
) -> i32 {
    let tx_id = amount.transfer(&account);
    if tx_id < 0 {
        return exit("Transfer failed", tx_id);
    }

    SUCCESS
}