//! Simple macro-based mocking for host functions.
//!
//! # Usage
//!
//! ```rust
//! use xrpl_wasm_stdlib::mock_host;
//! use xrpl_wasm_stdlib::core::types::nft::NFToken;
//!
//! #[test]
//! fn test_nft_transfer_fee() {
//!     mock_host! {
//!         get_nft_transfer_fee(_ptr, _len) => 5000
//!     };
//!
//!     let nft = NFToken::new([0u8; 32]);
//!     assert_eq!(nft.transfer_fee().unwrap(), 5000);
//! }
//! ```

extern crate std;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::thread_local;

type MockFn = Rc<dyn Fn(&[*const u8]) -> i32>;

thread_local! {
    static MOCKS: RefCell<HashMap<&'static str, MockFn>> = RefCell::new(HashMap::new());
}

pub fn set_mock<F>(name: &'static str, f: F)
where
    F: Fn(&[*const u8]) -> i32 + 'static,
{
    MOCKS.with(|m| {
        m.borrow_mut().insert(name, Rc::new(f));
    });
}

/// Clear a specific mock.
pub fn clear_mock(name: &'static str) {
    MOCKS.with(|m| {
        m.borrow_mut().remove(name);
    });
}

/// Clear all mocks.
pub fn clear_all_mocks() {
    MOCKS.with(|m| {
        m.borrow_mut().clear();
    });
}

/// Get a mock function if it exists.
pub(crate) fn get_mock(name: &'static str) -> Option<MockFn> {
    MOCKS.with(|m| m.borrow().get(name).cloned())
}

/// Macro for easily setting up mocks in tests.
#[macro_export]
macro_rules! mock_host {
    // Simple value return
    ($($name:ident($($arg:ident),*) => $ret:expr),+ $(,)?) => {
        $({
            $crate::host::mock::set_mock(stringify!($name), move |_args| $ret);
        })+
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::ptr;

    #[test]
    fn test_simple_mock() {
        mock_host! {
            get_nft_transfer_fee(_ptr, _len) => 42
        };

        let result = unsafe { super::super::get_nft_transfer_fee(ptr::null(), 0) };
        assert_eq!(result, 42);

        clear_all_mocks();
    }

    #[test]
    fn test_multiple_mocks() {
        mock_host! {
            get_nft_transfer_fee(_ptr, _len) => 100,
            get_nft_flags(_ptr, _len) => 200
        };

        assert_eq!(
            unsafe { super::super::get_nft_transfer_fee(ptr::null(), 0) },
            100
        );
        assert_eq!(unsafe { super::super::get_nft_flags(ptr::null(), 0) }, 200);

        clear_all_mocks();
    }
}
