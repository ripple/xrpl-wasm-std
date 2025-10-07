//! Assertion macros for WASM environments.
//!
//! This module provides assertion macros that work in WASM environments by using
//! the trace functions to emit readable error messages when assertions fail.

use crate::host::trace::{DataRepr, trace_data, trace_num};

/// Trait for numeric types that can be traced with trace_num
pub trait NumericTrace {
    fn trace_as_num(msg: &str, value: &Self);
}

/// Trace a value of any type.
///
/// This function handles different types of values:
/// - For array types like [u8; N], it uses trace_data with hex representation
/// - For other types, it falls back to a generic message
pub fn trace_value<T>(msg: &str, value: &T) {
    // Use trace_data as the default for all types
    let data_ptr = value as *const T as *const u8;
    let data_len = core::mem::size_of::<T>();
    let data_slice = unsafe { core::slice::from_raw_parts(data_ptr, data_len) };
    let _ = trace_data(msg, data_slice, DataRepr::AsHex);
}

/// Trace a numeric value.
///
/// This function is specifically for numeric types that implement NumericTrace.
/// It uses trace_num to display the value.
pub fn trace_numeric_value<T: NumericTrace>(msg: &str, value: &T) {
    NumericTrace::trace_as_num(msg, value);
}

// Specialized implementations for numeric types
macro_rules! impl_numeric_trace {
    ($($t:ty),*) => {
        $(
            impl NumericTrace for $t {
                #[inline]
                fn trace_as_num(msg: &str, value: &$t) {
                    let _ = trace_num(msg, *value as i64);
                }
            }
        )*
    }
}

// Implement for common numeric types
impl_numeric_trace!(i8, i16, i32, i64, u8, u16, u32, u64);

/// Asserts that two expressions are equal.
///
/// If the assertion fails, a trace message is emitted with the values of both expressions,
/// and then the program panics.
///
/// # Examples
///
/// ```
/// // This will pass
/// assert_eq!(1, 1);
///
/// // This would fail and emit a trace message
/// // assert_eq!(1, 2);
/// ```
#[macro_export]
macro_rules! assert_eq {
    ($left:expr, $right:expr) => {
        {
            let left_val = $left;
            let right_val = $right;
            if left_val != right_val {
                let _ = $crate::host::trace::trace(concat!("Assertion failed: ", stringify!($left), " != ", stringify!($right)));
                $crate::host::assert::trace_value("  left: ", &left_val);
                $crate::host::assert::trace_value("  right: ", &right_val);
                panic!("assertion failed: {} != {}", stringify!($left), stringify!($right));
            }
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        {
            let left_val = $left;
            let right_val = $right;
            if left_val != right_val {
                let _ = $crate::host::trace::trace(concat!("Assertion failed: ", stringify!($left), " != ", stringify!($right)));
                $crate::host::assert::trace_value("  left: ", &left_val);
                $crate::host::assert::trace_value("  right: ", &right_val);
                let _ = $crate::host::trace::trace("  message: (see panic message for details)");
                panic!("assertion failed: {} != {}: {}", stringify!($left), stringify!($right), format_args!($($arg)+));
            }
        }
    };
}

/// Asserts that a condition is true.
///
/// If the assertion fails, a trace message is emitted with the condition,
/// and then the program panics.
///
/// # Examples
///
/// ```
/// // This will pass
/// assert!(true);
///
/// // This would fail and emit a trace message
/// // assert!(false);
/// ```
#[macro_export]
macro_rules! assert {
    ($cond:expr) => {
        if !$cond {
            let _ = $crate::host::trace::trace(concat!("Assertion failed: ", stringify!($cond)));
            panic!("assertion failed: {}", stringify!($cond));
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        if !$cond {
            let _ = $crate::host::trace::trace(concat!("Assertion failed: ", stringify!($cond)));
            let _ = $crate::host::trace::trace("  message: (see panic message for details)");
            panic!("assertion failed: {}: {}", stringify!($cond), format_args!($($arg)+));
        }
    };
}

/// Asserts that two expressions are not equal.
///
/// If the assertion fails, a trace message is emitted with the values of both expressions,
/// and then the program panics.
///
/// # Examples
///
/// ```
/// // This will pass
/// assert_ne!(1, 2);
///
/// // This would fail and emit a trace message
/// // assert_ne!(1, 1);
/// ```
#[macro_export]
macro_rules! assert_ne {
    ($left:expr, $right:expr) => {
        {
            let left_val = $left;
            let right_val = $right;
            if left_val == right_val {
                let _ = $crate::host::trace::trace(concat!("Assertion failed: ", stringify!($left), " == ", stringify!($right)));
                $crate::host::assert::trace_value("  value: ", &left_val);
                panic!("assertion failed: {} == {}", stringify!($left), stringify!($right));
            }
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        {
            let left_val = $left;
            let right_val = $right;
            if left_val == right_val {
                let _ = $crate::host::trace::trace(concat!("Assertion failed: ", stringify!($left), " == ", stringify!($right)));
                $crate::host::assert::trace_value("  value: ", &left_val);
                let _ = $crate::host::trace::trace("  message: (see panic message for details)");
                panic!("assertion failed: {} == {}: {}", stringify!($left), stringify!($right), format_args!($($arg)+));
            }
        }
    };

}
