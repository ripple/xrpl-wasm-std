use crate::host::exit_with;

#[macro_export]
macro_rules! require {
    ($condition:expr, $error_code:expr, $error_msg:expr) => {
        if !$condition {
            unsafe {
                $crate::host::exit_with($error_code, $error_msg.as_ptr(), $error_msg.len());
            }
        }
    };
    ($condition:expr, $error_msg:expr) => {
        $crate::require!($condition, -1, $error_msg);
    };
}