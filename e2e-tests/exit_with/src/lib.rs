#![allow(unused_imports)]
#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_wasm_std::host::trace::{trace};
use xrpl_wasm_std::host::{exit_with};

use xrpl_wasm_std::require;

const SUCCESS: i32 = 0;
const BAD_PARAM: i32 = -1;

#[unsafe(no_mangle)]
pub extern "C" fn exit() -> i32 {
    let _ = trace("$$$$$ STARTING WASM EXECUTION $$$$$");
    require!(1 == 2, BAD_PARAM, "Transfer Failed");
    
    unsafe {
        exit_with(BAD_PARAM, b"Bad Parameter\0".as_ptr(), 14);
        // unreachable!("Execution should not continue after exit_with");
    }

    let _ = trace("SHOULD NOT REACH HERE");
    return SUCCESS;
}
