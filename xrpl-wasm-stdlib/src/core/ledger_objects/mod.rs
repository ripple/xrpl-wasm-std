pub mod account_root;
pub mod array_object;
pub mod current_escrow;
pub mod escrow;
pub mod traits;

use crate::core::types::account_id::{ACCOUNT_ID_SIZE, AccountID};
use crate::core::types::amount::Amount;
use crate::core::types::blob::Blob;
use crate::core::types::currency::Currency;
use crate::core::types::issue::{Issue, XrpIssue};
use crate::core::types::uint::{
    HASH128_SIZE, HASH160_SIZE, HASH192_SIZE, HASH256_SIZE, Hash128, Hash160, Hash192, Hash256,
};
use crate::host::error_codes::{
    match_result_code, match_result_code_optional, match_result_code_with_expected_bytes,
    match_result_code_with_expected_bytes_optional,
};
use crate::host::{Result, get_current_ledger_obj_field, get_ledger_obj_field};

/// Trait for types that can be retrieved from ledger object fields.
///
/// This trait provides a unified interface for retrieving typed data from XRPL ledger objects,
/// replacing the previous collection of type-specific functions with a generic, type-safe approach.
///
/// ## Supported Types
///
/// The following types implement this trait:
/// - `u16` - 16-bit unsigned integers (2 bytes)
/// - `u32` - 32-bit unsigned integers (4 bytes)
/// - `u64` - 64-bit unsigned integers (8 bytes)
/// - `AccountID` - 20-byte account identifiers
/// - `Amount` - XRP amounts and token amounts (variable size, up to 48 bytes)
/// - `Hash128` - 128-bit cryptographic hashes (16 bytes)
/// - `Hash256` - 256-bit cryptographic hashes (32 bytes)
/// - `Blob<N>` - Variable-length binary data (generic over buffer size `N`)
///
/// ## Usage Patterns
///
/// ```rust,no_run
/// use xrpl_wasm_stdlib::core::ledger_objects::{ledger_object, current_ledger_object};
/// use xrpl_wasm_stdlib::core::types::account_id::AccountID;
/// use xrpl_wasm_stdlib::core::types::amount::Amount;
/// use xrpl_wasm_stdlib::sfield;
///
/// fn example() {
///   let slot = 0;
///   // Get a required field from a specific ledger object
///   let balance = ledger_object::get_field(slot, sfield::Balance).unwrap();
///   let account = ledger_object::get_field(slot, sfield::Account).unwrap();
///
///   // Get an optional field from the current ledger object
///   let flags = current_ledger_object::get_field_optional(sfield::Flags).unwrap();
/// }
/// ```
///
/// ## Error Handling
///
/// - Required field methods return `Result<T>` and error if the field is missing
/// - Optional field methods return `Result<Option<T>>` and return `None` if the field is missing
/// - All methods return appropriate errors for buffer size mismatches or other retrieval failures
///
/// ## Safety Considerations
///
/// - All implementations use appropriately sized buffers for their data types
/// - Buffer sizes are validated against expected field sizes where applicable
/// - Unsafe operations are contained within the host function calls
pub trait FieldGetter: Sized {
    /// Get a required field from the current ledger object.
    ///
    /// # Arguments
    ///
    /// * `field_code` - The field code identifying which field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<Self>` where:
    /// * `Ok(Self)` - The field value for the specified field
    /// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self>;

    /// Get an optional field from the current ledger object.
    ///
    /// # Arguments
    ///
    /// * `field_code` - The field code identifying which field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<Self>>` where:
    /// * `Ok(Some(Self))` - The field value for the specified field
    /// * `Ok(None)` - If the field is not present
    /// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>>;

    /// Get a required field from a specific ledger object.
    ///
    /// # Arguments
    ///
    /// * `register_num` - The register number holding the ledger object
    /// * `field_code` - The field code identifying which field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<Self>` where:
    /// * `Ok(Self)` - The field value for the specified field
    /// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self>;

    /// Get an optional field from a specific ledger object.
    ///
    /// # Arguments
    ///
    /// * `register_num` - The register number holding the ledger object
    /// * `field_code` - The field code identifying which field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<Self>>` where:
    /// * `Ok(Some(Self))` - The field value for the specified field
    /// * `Ok(None)` - If the field is not present in the ledger object
    /// * `Err(Error)` - If the field retrieval operation failed
    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>>;
}

/// Implementation of `FieldGetter` for 16-bit unsigned integers.
///
/// This implementation handles 2-byte integer fields in XRPL ledger objects.
/// Common use cases include ledger entry types and small enumerated values.
///
/// # Buffer Management
///
/// Uses a 2-byte buffer and validates that exactly 2 bytes are returned
/// from the host function to ensure data integrity.
impl FieldGetter for u16 {
    #[inline]
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        let mut value = core::mem::MaybeUninit::<u16>::uninit();
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, value.as_mut_ptr().cast(), 2) };
        match_result_code_with_expected_bytes(result_code, 2, || unsafe { value.assume_init() })
    }

    #[inline]
    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        let mut value = core::mem::MaybeUninit::<u16>::uninit();
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, value.as_mut_ptr().cast(), 2) };
        match_result_code_with_expected_bytes_optional(result_code, 2, || {
            Some(unsafe { value.assume_init() })
        })
    }

    #[inline]
    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        let mut value = core::mem::MaybeUninit::<u16>::uninit();
        let result_code =
            unsafe { get_ledger_obj_field(register_num, field_code, value.as_mut_ptr().cast(), 2) };
        match_result_code_with_expected_bytes(result_code, 2, || unsafe { value.assume_init() })
    }

    #[inline]
    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        let mut value = core::mem::MaybeUninit::<u16>::uninit();
        let result_code =
            unsafe { get_ledger_obj_field(register_num, field_code, value.as_mut_ptr().cast(), 2) };
        match_result_code_with_expected_bytes_optional(result_code, 2, || {
            Some(unsafe { value.assume_init() })
        })
    }
}

/// Implementation of `FieldGetter` for 32-bit unsigned integers.
///
/// This implementation handles 4-byte integer fields in XRPL ledger objects.
/// Common use cases include sequence numbers, flags, timestamps, and various counters.
///
/// # Buffer Management
///
/// Uses a 4-byte buffer and validates that exactly 4 bytes are returned
/// from the host function to ensure data integrity.
impl FieldGetter for u32 {
    #[inline]
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        let mut value = core::mem::MaybeUninit::<u32>::uninit();
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, value.as_mut_ptr().cast(), 4) };
        match_result_code_with_expected_bytes(result_code, 4, || unsafe { value.assume_init() })
    }

    #[inline]
    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        let mut value = core::mem::MaybeUninit::<u32>::uninit();
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, value.as_mut_ptr().cast(), 4) };
        match_result_code_with_expected_bytes_optional(result_code, 4, || {
            Some(unsafe { value.assume_init() })
        })
    }

    #[inline]
    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        let mut value = core::mem::MaybeUninit::<u32>::uninit();
        let result_code =
            unsafe { get_ledger_obj_field(register_num, field_code, value.as_mut_ptr().cast(), 4) };
        match_result_code_with_expected_bytes(result_code, 4, || unsafe { value.assume_init() })
    }

    #[inline]
    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        let mut value = core::mem::MaybeUninit::<u32>::uninit();
        let result_code =
            unsafe { get_ledger_obj_field(register_num, field_code, value.as_mut_ptr().cast(), 4) };
        match_result_code_with_expected_bytes_optional(result_code, 4, || {
            Some(unsafe { value.assume_init() })
        })
    }
}

/// Implementation of `FieldGetter` for 64-bit unsigned integers.
///
/// This implementation handles 8-byte integer fields in XRPL ledger objects.
/// Common use cases include large numeric values, balances represented as integers,
/// and 64-bit identifiers.
///
/// # Buffer Management
///
/// Uses an 8-byte buffer and validates that exactly 8 bytes are returned
/// from the host function to ensure data integrity.
impl FieldGetter for u64 {
    #[inline]
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        let mut value = core::mem::MaybeUninit::<u64>::uninit();
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, value.as_mut_ptr().cast(), 8) };
        match_result_code_with_expected_bytes(result_code, 8, || unsafe { value.assume_init() })
    }

    #[inline]
    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        let mut value = core::mem::MaybeUninit::<u64>::uninit();
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, value.as_mut_ptr().cast(), 8) };
        match_result_code_with_expected_bytes_optional(result_code, 8, || {
            Some(unsafe { value.assume_init() })
        })
    }

    #[inline]
    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        let mut value = core::mem::MaybeUninit::<u64>::uninit();
        let result_code =
            unsafe { get_ledger_obj_field(register_num, field_code, value.as_mut_ptr().cast(), 8) };
        match_result_code_with_expected_bytes(result_code, 8, || unsafe { value.assume_init() })
    }

    #[inline]
    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        let mut value = core::mem::MaybeUninit::<u64>::uninit();
        let result_code =
            unsafe { get_ledger_obj_field(register_num, field_code, value.as_mut_ptr().cast(), 8) };
        match_result_code_with_expected_bytes_optional(result_code, 8, || {
            Some(unsafe { value.assume_init() })
        })
    }
}

/// Implementation of `FieldGetter` for XRPL account identifiers.
///
/// This implementation handles 20-byte account ID fields in XRPL ledger objects.
/// Account IDs uniquely identify accounts on the XRPL network and are derived
/// from public keys using cryptographic hashing.
///
/// # Buffer Management
///
/// Uses a 20-byte buffer (ACCOUNT_ID_SIZE) and validates that exactly 20 bytes
/// are returned from the host function. The buffer is converted to an AccountID
/// using the `From<[u8; 20]>` implementation.
impl FieldGetter for AccountID {
    #[inline]
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        let mut buffer = core::mem::MaybeUninit::<[u8; ACCOUNT_ID_SIZE]>::uninit();
        let result_code = unsafe {
            get_current_ledger_obj_field(field_code, buffer.as_mut_ptr().cast(), ACCOUNT_ID_SIZE)
        };
        match_result_code_with_expected_bytes(result_code, ACCOUNT_ID_SIZE, || {
            unsafe { buffer.assume_init() }.into()
        })
    }

    #[inline]
    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        let mut buffer = core::mem::MaybeUninit::<[u8; ACCOUNT_ID_SIZE]>::uninit();
        let result_code = unsafe {
            get_current_ledger_obj_field(field_code, buffer.as_mut_ptr().cast(), ACCOUNT_ID_SIZE)
        };
        match_result_code_with_expected_bytes_optional(result_code, ACCOUNT_ID_SIZE, || {
            Some(unsafe { buffer.assume_init() }.into())
        })
    }

    #[inline]
    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        let mut buffer = core::mem::MaybeUninit::<[u8; ACCOUNT_ID_SIZE]>::uninit();
        let result_code = unsafe {
            get_ledger_obj_field(
                register_num,
                field_code,
                buffer.as_mut_ptr().cast(),
                ACCOUNT_ID_SIZE,
            )
        };
        match_result_code_with_expected_bytes(result_code, ACCOUNT_ID_SIZE, || {
            unsafe { buffer.assume_init() }.into()
        })
    }

    #[inline]
    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        let mut buffer = core::mem::MaybeUninit::<[u8; ACCOUNT_ID_SIZE]>::uninit();
        let result_code = unsafe {
            get_ledger_obj_field(
                register_num,
                field_code,
                buffer.as_mut_ptr().cast(),
                ACCOUNT_ID_SIZE,
            )
        };
        match_result_code_with_expected_bytes_optional(result_code, ACCOUNT_ID_SIZE, || {
            Some(unsafe { buffer.assume_init() }.into())
        })
    }
}

/// Implementation of `FieldGetter` for XRPL amount values.
///
/// This implementation handles amount fields in XRPL ledger objects, which can represent
/// either XRP amounts (8 bytes) or token amounts (up to 48 bytes including currency code
/// and issuer information).
///
/// # Buffer Management
///
/// Uses a 48-byte buffer to accommodate the largest possible amount representation.
/// The Amount type handles the parsing of different amount formats internally.
/// No strict byte count validation is performed since amounts can vary in size.
impl FieldGetter for Amount {
    #[inline]
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        const BUFFER_SIZE: usize = 48;
        let mut buffer = core::mem::MaybeUninit::<[u8; BUFFER_SIZE]>::uninit();
        let result_code = unsafe {
            get_current_ledger_obj_field(field_code, buffer.as_mut_ptr().cast(), BUFFER_SIZE)
        };
        match_result_code(result_code, || {
            Amount::from(unsafe { buffer.assume_init() })
        })
    }

    #[inline]
    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        const BUFFER_SIZE: usize = 48;
        let mut buffer = core::mem::MaybeUninit::<[u8; BUFFER_SIZE]>::uninit();
        let result_code = unsafe {
            get_current_ledger_obj_field(field_code, buffer.as_mut_ptr().cast(), BUFFER_SIZE)
        };
        match_result_code_optional(result_code, || {
            Some(Amount::from(unsafe { buffer.assume_init() }))
        })
    }

    #[inline]
    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        const BUFFER_SIZE: usize = 48;
        let mut buffer = core::mem::MaybeUninit::<[u8; BUFFER_SIZE]>::uninit();
        let result_code = unsafe {
            get_ledger_obj_field(
                register_num,
                field_code,
                buffer.as_mut_ptr().cast(),
                BUFFER_SIZE,
            )
        };
        match_result_code(result_code, || {
            Amount::from(unsafe { buffer.assume_init() })
        })
    }

    #[inline]
    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        const BUFFER_SIZE: usize = 48;
        let mut buffer = core::mem::MaybeUninit::<[u8; BUFFER_SIZE]>::uninit();
        let result_code = unsafe {
            get_ledger_obj_field(
                register_num,
                field_code,
                buffer.as_mut_ptr().cast(),
                BUFFER_SIZE,
            )
        };
        match_result_code_optional(result_code, || {
            Some(Amount::from(unsafe { buffer.assume_init() }))
        })
    }
}

/// Implementation of `FieldGetter` for 128-bit cryptographic hashes.
///
/// This implementation handles 16-byte hash fields in XRPL ledger objects.
/// Hash128 values are commonly used for shorter identifiers and checksums
/// in XRPL, such as email hashes.
///
/// # Buffer Management
///
/// Uses a 16-byte buffer (HASH128_SIZE) and validates that exactly 16 bytes
/// are returned from the host function to ensure data integrity.
impl FieldGetter for Hash128 {
    #[inline]
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        let mut buffer = core::mem::MaybeUninit::<[u8; HASH128_SIZE]>::uninit();
        let result_code = unsafe {
            get_current_ledger_obj_field(field_code, buffer.as_mut_ptr().cast(), HASH128_SIZE)
        };
        match_result_code_with_expected_bytes(result_code, HASH128_SIZE, || {
            Hash128::from(unsafe { buffer.assume_init() })
        })
    }

    #[inline]
    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        let mut buffer = core::mem::MaybeUninit::<[u8; HASH128_SIZE]>::uninit();
        let result_code = unsafe {
            get_current_ledger_obj_field(field_code, buffer.as_mut_ptr().cast(), HASH128_SIZE)
        };
        match_result_code_with_expected_bytes_optional(result_code, HASH128_SIZE, || {
            Some(Hash128::from(unsafe { buffer.assume_init() }))
        })
    }

    #[inline]
    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        let mut buffer = core::mem::MaybeUninit::<[u8; HASH128_SIZE]>::uninit();
        let result_code = unsafe {
            get_ledger_obj_field(
                register_num,
                field_code,
                buffer.as_mut_ptr().cast(),
                HASH128_SIZE,
            )
        };
        match_result_code_with_expected_bytes(result_code, HASH128_SIZE, || {
            Hash128::from(unsafe { buffer.assume_init() })
        })
    }

    #[inline]
    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        let mut buffer = core::mem::MaybeUninit::<[u8; HASH128_SIZE]>::uninit();
        let result_code = unsafe {
            get_ledger_obj_field(
                register_num,
                field_code,
                buffer.as_mut_ptr().cast(),
                HASH128_SIZE,
            )
        };
        match_result_code_with_expected_bytes_optional(result_code, HASH128_SIZE, || {
            Some(Hash128::from(unsafe { buffer.assume_init() }))
        })
    }
}

/// Implementation of `FieldGetter` for 256-bit cryptographic hashes.
///
/// This implementation handles 32-byte hash fields in XRPL ledger objects.
/// Hash256 values are widely used throughout XRPL for transaction IDs,
/// ledger indexes, object IDs, and various cryptographic operations.
///
/// # Buffer Management
///
/// Uses a 32-byte buffer (HASH256_SIZE) and validates that exactly 32 bytes
/// are returned from the host function to ensure data integrity.
impl FieldGetter for Hash256 {
    #[inline]
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        let mut buffer = core::mem::MaybeUninit::<[u8; HASH256_SIZE]>::uninit();
        let result_code = unsafe {
            get_current_ledger_obj_field(field_code, buffer.as_mut_ptr().cast(), HASH256_SIZE)
        };
        match_result_code_with_expected_bytes(result_code, HASH256_SIZE, || {
            Hash256::from(unsafe { buffer.assume_init() })
        })
    }

    #[inline]
    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        let mut buffer = core::mem::MaybeUninit::<[u8; HASH256_SIZE]>::uninit();
        let result_code = unsafe {
            get_current_ledger_obj_field(field_code, buffer.as_mut_ptr().cast(), HASH256_SIZE)
        };
        match_result_code_with_expected_bytes_optional(result_code, HASH256_SIZE, || {
            Some(Hash256::from(unsafe { buffer.assume_init() }))
        })
    }

    #[inline]
    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        let mut buffer = core::mem::MaybeUninit::<[u8; HASH256_SIZE]>::uninit();
        let result_code = unsafe {
            get_ledger_obj_field(
                register_num,
                field_code,
                buffer.as_mut_ptr().cast(),
                HASH256_SIZE,
            )
        };
        match_result_code_with_expected_bytes(result_code, HASH256_SIZE, || {
            Hash256::from(unsafe { buffer.assume_init() })
        })
    }

    #[inline]
    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        let mut buffer = core::mem::MaybeUninit::<[u8; HASH256_SIZE]>::uninit();
        let result_code = unsafe {
            get_ledger_obj_field(
                register_num,
                field_code,
                buffer.as_mut_ptr().cast(),
                HASH256_SIZE,
            )
        };
        match_result_code_with_expected_bytes_optional(result_code, HASH256_SIZE, || {
            Some(Hash256::from(unsafe { buffer.assume_init() }))
        })
    }
}

/// Implementation of `FieldGetter` for 8-bit unsigned integers.
///
/// This implementation handles 1-byte integer fields in XRPL ledger objects.
/// Common use cases include small counters, flags, and other single-byte values.
///
/// # Buffer Management
///
/// Uses a 1-byte buffer and validates that exactly 1 byte is returned
/// from the host function to ensure data integrity.
impl FieldGetter for u8 {
    #[inline]
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        let mut value = core::mem::MaybeUninit::<u8>::uninit();
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, value.as_mut_ptr().cast(), 1) };
        match_result_code_with_expected_bytes(result_code, 1, || unsafe { value.assume_init() })
    }

    #[inline]
    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        let mut value = core::mem::MaybeUninit::<u8>::uninit();
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, value.as_mut_ptr().cast(), 1) };
        match_result_code_with_expected_bytes_optional(result_code, 1, || {
            Some(unsafe { value.assume_init() })
        })
    }

    #[inline]
    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        let mut value = core::mem::MaybeUninit::<u8>::uninit();
        let result_code =
            unsafe { get_ledger_obj_field(register_num, field_code, value.as_mut_ptr().cast(), 1) };
        match_result_code_with_expected_bytes(result_code, 1, || unsafe { value.assume_init() })
    }

    #[inline]
    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        let mut value = core::mem::MaybeUninit::<u8>::uninit();
        let result_code =
            unsafe { get_ledger_obj_field(register_num, field_code, value.as_mut_ptr().cast(), 1) };
        match_result_code_with_expected_bytes_optional(result_code, 1, || {
            Some(unsafe { value.assume_init() })
        })
    }
}

/// Implementation of `FieldGetter` for 160-bit cryptographic hashes.
///
/// This implementation handles 20-byte hash fields in XRPL ledger objects.
/// Hash160 values are used for various cryptographic operations and identifiers.
///
/// # Buffer Management
///
/// Uses a 20-byte buffer (HASH160_SIZE) and validates that exactly 20 bytes
/// are returned from the host function to ensure data integrity.
impl FieldGetter for Hash160 {
    #[inline]
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        let mut buffer = core::mem::MaybeUninit::<[u8; HASH160_SIZE]>::uninit();
        let result_code = unsafe {
            get_current_ledger_obj_field(field_code, buffer.as_mut_ptr().cast(), HASH160_SIZE)
        };
        match_result_code_with_expected_bytes(result_code, HASH160_SIZE, || {
            Hash160::from(unsafe { buffer.assume_init() })
        })
    }

    #[inline]
    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        let mut buffer = core::mem::MaybeUninit::<[u8; HASH160_SIZE]>::uninit();
        let result_code = unsafe {
            get_current_ledger_obj_field(field_code, buffer.as_mut_ptr().cast(), HASH160_SIZE)
        };
        match_result_code_with_expected_bytes_optional(result_code, HASH160_SIZE, || {
            Some(Hash160::from(unsafe { buffer.assume_init() }))
        })
    }

    #[inline]
    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        let mut buffer = core::mem::MaybeUninit::<[u8; HASH160_SIZE]>::uninit();
        let result_code = unsafe {
            get_ledger_obj_field(
                register_num,
                field_code,
                buffer.as_mut_ptr().cast(),
                HASH160_SIZE,
            )
        };
        match_result_code_with_expected_bytes(result_code, HASH160_SIZE, || {
            Hash160::from(unsafe { buffer.assume_init() })
        })
    }

    #[inline]
    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        let mut buffer = core::mem::MaybeUninit::<[u8; HASH160_SIZE]>::uninit();
        let result_code = unsafe {
            get_ledger_obj_field(
                register_num,
                field_code,
                buffer.as_mut_ptr().cast(),
                HASH160_SIZE,
            )
        };
        match_result_code_with_expected_bytes_optional(result_code, HASH160_SIZE, || {
            Some(Hash160::from(unsafe { buffer.assume_init() }))
        })
    }
}

/// Implementation of `FieldGetter` for 192-bit cryptographic hashes.
///
/// This implementation handles 24-byte hash fields in XRPL ledger objects.
/// Hash192 values are used for various cryptographic operations and identifiers.
///
/// # Buffer Management
///
/// Uses a 24-byte buffer (HASH192_SIZE) and validates that exactly 24 bytes
/// are returned from the host function to ensure data integrity.
impl FieldGetter for Hash192 {
    #[inline]
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        let mut buffer = core::mem::MaybeUninit::<[u8; HASH192_SIZE]>::uninit();
        let result_code = unsafe {
            get_current_ledger_obj_field(field_code, buffer.as_mut_ptr().cast(), HASH192_SIZE)
        };
        match_result_code_with_expected_bytes(result_code, HASH192_SIZE, || {
            Hash192::from(unsafe { buffer.assume_init() })
        })
    }

    #[inline]
    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        let mut buffer = core::mem::MaybeUninit::<[u8; HASH192_SIZE]>::uninit();
        let result_code = unsafe {
            get_current_ledger_obj_field(field_code, buffer.as_mut_ptr().cast(), HASH192_SIZE)
        };
        match_result_code_with_expected_bytes_optional(result_code, HASH192_SIZE, || {
            Some(Hash192::from(unsafe { buffer.assume_init() }))
        })
    }

    #[inline]
    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        let mut buffer = core::mem::MaybeUninit::<[u8; HASH192_SIZE]>::uninit();
        let result_code = unsafe {
            get_ledger_obj_field(
                register_num,
                field_code,
                buffer.as_mut_ptr().cast(),
                HASH192_SIZE,
            )
        };
        match_result_code_with_expected_bytes(result_code, HASH192_SIZE, || {
            Hash192::from(unsafe { buffer.assume_init() })
        })
    }

    #[inline]
    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        let mut buffer = core::mem::MaybeUninit::<[u8; HASH192_SIZE]>::uninit();
        let result_code = unsafe {
            get_ledger_obj_field(
                register_num,
                field_code,
                buffer.as_mut_ptr().cast(),
                HASH192_SIZE,
            )
        };
        match_result_code_with_expected_bytes_optional(result_code, HASH192_SIZE, || {
            Some(Hash192::from(unsafe { buffer.assume_init() }))
        })
    }
}

/// Implementation of `FieldGetter` for XRPL currency codes.
///
/// This implementation handles 20-byte currency code fields in XRPL ledger objects.
/// Currency codes uniquely identify different currencies and assets on the XRPL.
///
/// # Buffer Management
///
/// Uses a 20-byte buffer and validates that exactly 20 bytes are returned
/// from the host function to ensure data integrity.
impl FieldGetter for Currency {
    #[inline]
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        let mut buffer = core::mem::MaybeUninit::<[u8; 20]>::uninit();
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, buffer.as_mut_ptr().cast(), 20) };
        match_result_code_with_expected_bytes(result_code, 20, || {
            Currency::from(unsafe { buffer.assume_init() })
        })
    }

    #[inline]
    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        let mut buffer = core::mem::MaybeUninit::<[u8; 20]>::uninit();
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, buffer.as_mut_ptr().cast(), 20) };
        match_result_code_with_expected_bytes_optional(result_code, 20, || {
            Some(Currency::from(unsafe { buffer.assume_init() }))
        })
    }

    #[inline]
    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        let mut buffer = core::mem::MaybeUninit::<[u8; 20]>::uninit();
        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, buffer.as_mut_ptr().cast(), 20)
        };
        match_result_code_with_expected_bytes(result_code, 20, || {
            Currency::from(unsafe { buffer.assume_init() })
        })
    }

    #[inline]
    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        let mut buffer = core::mem::MaybeUninit::<[u8; 20]>::uninit();
        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, buffer.as_mut_ptr().cast(), 20)
        };
        match_result_code_with_expected_bytes_optional(result_code, 20, || {
            Some(Currency::from(unsafe { buffer.assume_init() }))
        })
    }
}

/// Implementation of `FieldGetter` for XRPL issues (XRP variant only).
///
/// This implementation handles issue fields in XRPL ledger objects.
/// Currently only supports the XRP variant of Issue.
///
/// # Buffer Management
///
/// Uses a 20-byte buffer for XRP issues and validates that exactly 20 bytes
/// are returned from the host function to ensure data integrity.
impl FieldGetter for Issue {
    #[inline]
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        let mut buffer = core::mem::MaybeUninit::<[u8; 20]>::uninit();
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, buffer.as_mut_ptr().cast(), 20) };
        match_result_code_with_expected_bytes(result_code, 20, || {
            // For now, always return XRP issue
            Issue::XRP(XrpIssue {})
        })
    }

    #[inline]
    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        let mut buffer = core::mem::MaybeUninit::<[u8; 20]>::uninit();
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, buffer.as_mut_ptr().cast(), 20) };
        match_result_code_with_expected_bytes_optional(result_code, 20, || {
            // For now, always return XRP issue
            Some(Issue::XRP(XrpIssue {}))
        })
    }

    #[inline]
    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        let mut buffer = core::mem::MaybeUninit::<[u8; 20]>::uninit();
        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, buffer.as_mut_ptr().cast(), 20)
        };
        match_result_code_with_expected_bytes(result_code, 20, || {
            // For now, always return XRP issue
            Issue::XRP(XrpIssue {})
        })
    }

    #[inline]
    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        let mut buffer = core::mem::MaybeUninit::<[u8; 20]>::uninit();
        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, buffer.as_mut_ptr().cast(), 20)
        };
        match_result_code_with_expected_bytes_optional(result_code, 20, || {
            // For now, always return XRP issue
            Some(Issue::XRP(XrpIssue {}))
        })
    }
}

/// Implementation of `FieldGetter` for variable-length binary data.
///
/// This implementation handles blob fields in XRPL ledger objects, which can contain
/// arbitrary binary data such as memos, signatures, public keys, and other
/// variable-length content.
///
/// # Buffer Management
///
/// Uses a buffer of size `N` to accommodate blob field data. The actual
/// length of the data is determined by the return value from the host function
/// and stored in the Blob's `len` field. No strict byte count validation is
/// performed since blobs can vary significantly in size.
///
/// # Type Parameters
///
/// * `N` - The maximum capacity of the blob buffer in bytes
impl<const N: usize> FieldGetter for Blob<N> {
    #[inline]
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        let mut buffer = core::mem::MaybeUninit::<[u8; N]>::uninit();
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, buffer.as_mut_ptr().cast(), N) };
        match_result_code(result_code, || Blob {
            data: unsafe { buffer.assume_init() },
            len: result_code as usize,
        })
    }

    #[inline]
    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        let mut buffer = core::mem::MaybeUninit::<[u8; N]>::uninit();
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, buffer.as_mut_ptr().cast(), N) };
        match_result_code_optional(result_code, || {
            Some(Blob {
                data: unsafe { buffer.assume_init() },
                len: result_code as usize,
            })
        })
    }

    #[inline]
    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        let mut buffer = core::mem::MaybeUninit::<[u8; N]>::uninit();
        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, buffer.as_mut_ptr().cast(), N)
        };
        match_result_code(result_code, || Blob {
            data: unsafe { buffer.assume_init() },
            len: result_code as usize,
        })
    }

    #[inline]
    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        let mut buffer = core::mem::MaybeUninit::<[u8; N]>::uninit();
        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, buffer.as_mut_ptr().cast(), N)
        };
        match_result_code_optional(result_code, || {
            Some(Blob {
                data: unsafe { buffer.assume_init() },
                len: result_code as usize,
            })
        })
    }
}

pub mod current_ledger_object {
    use super::FieldGetter;
    use crate::host::Result;
    use crate::sfield::SField;

    /// Retrieves a field from the current ledger object.
    ///
    /// # Arguments
    ///
    /// * `field` - An SField constant that encodes both the field code and expected type
    ///
    /// # Returns
    ///
    /// Returns a `Result<T>` where:
    /// * `Ok(T)` - The field value for the specified field
    /// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use xrpl_wasm_stdlib::core::ledger_objects::current_ledger_object;
    /// use xrpl_wasm_stdlib::sfield;
    ///
    /// // Type is automatically inferred from the SField constant
    /// let flags = current_ledger_object::get_field(sfield::Flags).unwrap();  // u32
    /// let balance = current_ledger_object::get_field(sfield::Balance).unwrap();  // u64
    /// ```
    #[inline]
    pub fn get_field<T: FieldGetter, const CODE: i32>(_field: SField<T, CODE>) -> Result<T> {
        T::get_from_current_ledger_obj(CODE)
    }

    /// Retrieves an optionally present field from the current ledger object.
    ///
    /// # Arguments
    ///
    /// * `field` - An SField constant that encodes both the field code and expected type
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<T>>` where:
    /// * `Ok(Some(T))` - The field value for the specified field
    /// * `Ok(None)` - If the field is not present
    /// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
    #[inline]
    pub fn get_field_optional<T: FieldGetter, const CODE: i32>(
        _field: SField<T, CODE>,
    ) -> Result<Option<T>> {
        T::get_from_current_ledger_obj_optional(CODE)
    }
}

pub mod ledger_object {
    use super::FieldGetter;
    use crate::host::Result;
    use crate::sfield::SField;

    /// Retrieves a field from a specified ledger object.
    ///
    /// # Arguments
    ///
    /// * `register_num` - The register number holding the ledger object to look for data in
    /// * `field` - An SField constant that encodes both the field code and expected type
    ///
    /// # Returns
    ///
    /// Returns a `Result<T>` where:
    /// * `Ok(T)` - The field value for the specified field
    /// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use xrpl_wasm_stdlib::core::ledger_objects::ledger_object;
    /// use xrpl_wasm_stdlib::sfield;
    ///
    /// // Type is automatically inferred from the SField constant
    /// let balance = ledger_object::get_field(0, sfield::Balance).unwrap();  // u64
    /// let account = ledger_object::get_field(0, sfield::Account).unwrap();  // AccountID
    /// ```
    #[inline]
    pub fn get_field<T: FieldGetter, const CODE: i32>(
        register_num: i32,
        _field: SField<T, CODE>,
    ) -> Result<T> {
        T::get_from_ledger_obj(register_num, CODE)
    }

    /// Retrieves an optionally present field from a specified ledger object.
    ///
    /// # Arguments
    ///
    /// * `register_num` - The register number holding the ledger object to look for data in
    /// * `field` - An SField constant that encodes both the field code and expected type
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<T>>` where:
    /// * `Ok(Some(T))` - The field value for the specified field
    /// * `Ok(None)` - If the field is not present in the ledger object
    /// * `Err(Error)` - If the field retrieval operation failed
    #[inline]
    pub fn get_field_optional<T: FieldGetter, const CODE: i32>(
        register_num: i32,
        _field: SField<T, CODE>,
    ) -> Result<Option<T>> {
        T::get_from_ledger_obj_optional(register_num, CODE)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::core::ledger_objects::{current_ledger_object, ledger_object};
        use crate::core::types::account_id::{ACCOUNT_ID_SIZE, AccountID};
        use crate::core::types::amount::Amount;
        use crate::core::types::blob::{Blob, DEFAULT_BLOB_SIZE};
        use crate::core::types::public_key::PUBLIC_KEY_BUFFER_SIZE;
        use crate::core::types::uint::{HASH128_SIZE, HASH256_SIZE, Hash128, Hash256};
        use crate::sfield;

        // ========================================
        // Basic smoke tests for FieldGetter implementations
        // These tests verify that the trait implementations compile and work with the test host.
        // Note: The test host returns buffer_len as success, so these only verify basic functionality.
        // ========================================

        #[test]
        fn test_field_getter_basic_types() {
            // Test that all basic integer types work
            assert!(u16::get_from_current_ledger_obj(65537i32).is_ok());
            assert!(u32::get_from_current_ledger_obj(65538i32).is_ok());
            assert!(u64::get_from_current_ledger_obj(393216i32).is_ok());
        }

        #[test]
        fn test_field_getter_xrpl_types() {
            // Test that XRPL-specific types work
            assert!(AccountID::get_from_current_ledger_obj(524289i32).is_ok());
            assert!(Amount::get_from_current_ledger_obj(393216i32).is_ok());
            assert!(Hash128::get_from_current_ledger_obj(262145i32).is_ok());
            assert!(Hash256::get_from_current_ledger_obj(262146i32).is_ok());

            let blob: Blob<DEFAULT_BLOB_SIZE> =
                Blob::get_from_current_ledger_obj(sfield::PublicKey.into()).unwrap();
            // The test host returns buffer length as the result
            assert_eq!(blob.len, DEFAULT_BLOB_SIZE);
        }

        #[test]
        fn test_field_getter_optional_variants() {
            // Test optional field retrieval
            let result = u32::get_from_current_ledger_obj_optional(65538i32);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());

            let result = AccountID::get_from_current_ledger_obj_optional(524289i32);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        #[test]
        fn test_field_getter_with_slot() {
            // Test ledger object field retrieval with slot numbers
            let slot = 0;
            assert!(u32::get_from_ledger_obj(slot, 65538i32).is_ok());
            assert!(u64::get_from_ledger_obj(slot, 393216i32).is_ok());
            assert!(AccountID::get_from_ledger_obj(slot, 524289i32).is_ok());
        }

        #[test]
        fn test_field_getter_optional_with_slot() {
            // Test optional field retrieval with slot numbers
            let slot = 0;
            let result = u32::get_from_ledger_obj_optional(slot, 65538i32);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        // ========================================
        // Tests for module-level convenience functions
        // ========================================

        #[test]
        fn test_current_ledger_object_module() {
            // Test the current_ledger_object module's convenience functions
            assert!(current_ledger_object::get_field(sfield::Flags).is_ok());
            assert!(current_ledger_object::get_field(sfield::Account).is_ok());

            let result = current_ledger_object::get_field_optional(sfield::Flags);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        #[test]
        fn test_ledger_object_module() {
            // Test the ledger_object module's convenience functions
            let slot = 0;
            assert!(ledger_object::get_field(slot, sfield::LedgerEntryType).is_ok());
            assert!(ledger_object::get_field(slot, sfield::Flags).is_ok());
            assert!(ledger_object::get_field(slot, sfield::Balance).is_ok());
            assert!(ledger_object::get_field(slot, sfield::Account).is_ok());
            assert!(ledger_object::get_field(slot, sfield::Amount).is_ok());
            assert!(ledger_object::get_field(slot, sfield::EmailHash).is_ok());
            assert!(ledger_object::get_field(slot, sfield::PreviousTxnID).is_ok());
            assert!(ledger_object::get_field(slot, sfield::PublicKey).is_ok());

            let result = ledger_object::get_field_optional(slot, sfield::Flags);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        // ========================================
        // Type inference and compilation tests
        // ========================================

        #[test]
        fn test_type_inference() {
            let slot = 0;
            // Verify type inference works with turbofish syntax
            let _balance = get_field(slot, sfield::Balance);
            let _account = get_field(slot, sfield::Account);

            // Verify type inference works with type annotations
            let _sequence: Result<u32> = get_field(slot, sfield::Sequence);
            let _flags: Result<u32> = get_field(slot, sfield::Flags);
        }

        // ========================================
        // Data size verification tests
        // ========================================

        #[test]
        fn test_type_sizes() {
            // Verify that returned types have the expected sizes
            let hash128 = Hash128::get_from_current_ledger_obj(262145i32).unwrap();
            assert_eq!(hash128.as_bytes().len(), HASH128_SIZE);

            let hash256 = Hash256::get_from_current_ledger_obj(262146i32).unwrap();
            assert_eq!(hash256.as_bytes().len(), HASH256_SIZE);

            let account = AccountID::get_from_current_ledger_obj(524289i32).unwrap();
            assert_eq!(account.0.len(), ACCOUNT_ID_SIZE);

            let blob: Blob<{ PUBLIC_KEY_BUFFER_SIZE }> =
                Blob::get_from_current_ledger_obj(sfield::PublicKey.into()).unwrap();
            // In test environment, host returns buffer size as result code
            assert_eq!(blob.len, PUBLIC_KEY_BUFFER_SIZE);
            assert_eq!(blob.data.len(), PUBLIC_KEY_BUFFER_SIZE);
        }
    }
}
