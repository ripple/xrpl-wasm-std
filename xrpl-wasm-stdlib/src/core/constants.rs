use crate::core::types::account_id::AccountID;
use crate::r_address;

/// The 20 bytes of Account Zero (rrrrrrrrrrrrrrrrrrrrrhoLvTp)
pub const ACCOUNT_ZERO: AccountID = AccountID([0u8; 20]);

/// The 20 bytes of Account One (rrrrrrrrrrrrrrrrrrrrBZbvji)
pub const ACCOUNT_ONE: AccountID = {
    // Create a mutable array *only* during compile-time evaluation
    let mut arr = [0x00; 20];
    arr[19] = 0x01;
    // The final value of the block is the initialized array
    AccountID(arr)
};

/// Example: Common XRPL account using the r_address! macro
/// This demonstrates compile-time conversion of r-addresses to 20-byte arrays.
/// The macro validates the address at compile time and converts it to raw bytes,
/// with zero runtime overhead.
///
/// # Example
/// ```
/// use xrpl_wasm_stdlib::core::constants::EXAMPLE_ACCOUNT;
/// use xrpl_wasm_stdlib::core::types::account_id::AccountID;
///
/// // Use the pre-defined account constant
/// let account: AccountID = EXAMPLE_ACCOUNT;
/// ```
pub const EXAMPLE_ACCOUNT: AccountID = AccountID(r_address!("rN7n7otQDd6FczFgLdSqtcsAUxDkw6fzRH"));

/// Indivisible unit of XRP
pub const ONE_DROP: u64 = 1;

/// 100 billion XRP
pub const MAX_XRP: u64 = 100_000_000_000u64;
/// Maximum possible drops of XRP
pub const MAX_DROPS: u64 = MAX_XRP * 1_000_000;
