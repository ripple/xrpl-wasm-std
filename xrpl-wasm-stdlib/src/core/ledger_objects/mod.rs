pub mod account_root;
pub mod current_escrow;
pub mod escrow;
pub mod nft;
pub mod traits;

use crate::core::types::account_id::{ACCOUNT_ID_SIZE, AccountID};
use crate::core::types::amount::Amount;
use crate::core::types::blob::Blob;
use crate::core::types::uint::{HASH128_SIZE, HASH256_SIZE, Hash128, Hash256};
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
/// - `Blob` - Variable-length binary data (up to 1024 bytes)
///
/// ## Usage Patterns
///
/// ```rust,no_run
/// use xrpl_wasm_stdlib::core::ledger_objects::{ledger_object, current_ledger_object};
/// use xrpl_wasm_stdlib::core::types::account_id::AccountID;
/// use xrpl_wasm_stdlib::sfield;
/// # fn example() {
/// # let slot = 0;
/// // Get a required field from a specific ledger object
/// let balance: u64 = ledger_object::get_field(slot, sfield::Balance).unwrap();
/// let account: AccountID = ledger_object::get_field(slot, sfield::Account).unwrap();
///
/// // Get an optional field from the current ledger object
/// let flags: Option<u32> = current_ledger_object::get_field_optional(sfield::Flags).unwrap();
/// # }
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
        let mut value: u16 = 0;
        let value_ptr: *mut u8 = (&mut value as *mut u16).cast::<u8>();
        let result_code = unsafe { get_current_ledger_obj_field(field_code, value_ptr, 2) };
        match_result_code_with_expected_bytes(result_code, 2, || value)
    }

    #[inline]
    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        let mut value: u16 = 0;
        let value_ptr: *mut u8 = (&mut value as *mut u16).cast::<u8>();
        let result_code = unsafe { get_current_ledger_obj_field(field_code, value_ptr, 2) };
        match_result_code_with_expected_bytes_optional(result_code, 2, || Some(value))
    }

    #[inline]
    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        let mut value: u16 = 0;
        let value_ptr: *mut u8 = (&mut value as *mut u16).cast::<u8>();
        let result_code = unsafe { get_ledger_obj_field(register_num, field_code, value_ptr, 2) };
        match_result_code_with_expected_bytes(result_code, 2, || value)
    }

    #[inline]
    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        let mut value: u16 = 0;
        let value_ptr: *mut u8 = (&mut value as *mut u16).cast::<u8>();
        let result_code = unsafe { get_ledger_obj_field(register_num, field_code, value_ptr, 2) };
        match_result_code_with_expected_bytes_optional(result_code, 2, || Some(value))
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
        let mut value: u32 = 0;
        let value_ptr: *mut u8 = (&mut value as *mut u32).cast::<u8>();
        let result_code = unsafe { get_current_ledger_obj_field(field_code, value_ptr, 4) };
        match_result_code_with_expected_bytes(result_code, 4, || value)
    }

    #[inline]
    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        let mut value: u32 = 0;
        let value_ptr: *mut u8 = (&mut value as *mut u32).cast::<u8>();
        let result_code = unsafe { get_current_ledger_obj_field(field_code, value_ptr, 4) };
        match_result_code_with_expected_bytes_optional(result_code, 4, || Some(value))
    }

    #[inline]
    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        let mut value: u32 = 0;
        let value_ptr: *mut u8 = (&mut value as *mut u32).cast::<u8>();
        let result_code = unsafe { get_ledger_obj_field(register_num, field_code, value_ptr, 4) };
        match_result_code_with_expected_bytes(result_code, 4, || value)
    }

    #[inline]
    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        let mut value: u32 = 0;
        let value_ptr: *mut u8 = (&mut value as *mut u32).cast::<u8>();
        let result_code = unsafe { get_ledger_obj_field(register_num, field_code, value_ptr, 4) };
        match_result_code_with_expected_bytes_optional(result_code, 4, || Some(value))
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
        let mut value: u64 = 0;
        let value_ptr: *mut u8 = (&mut value as *mut u64).cast::<u8>();
        let result_code = unsafe { get_current_ledger_obj_field(field_code, value_ptr, 8) };
        match_result_code_with_expected_bytes(result_code, 8, || value)
    }

    #[inline]
    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        let mut value: u64 = 0;
        let value_ptr: *mut u8 = (&mut value as *mut u64).cast::<u8>();
        let result_code = unsafe { get_current_ledger_obj_field(field_code, value_ptr, 8) };
        match_result_code_with_expected_bytes_optional(result_code, 8, || Some(value))
    }

    #[inline]
    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        let mut value: u64 = 0;
        let value_ptr: *mut u8 = (&mut value as *mut u64).cast::<u8>();
        let result_code = unsafe { get_ledger_obj_field(register_num, field_code, value_ptr, 8) };
        match_result_code_with_expected_bytes(result_code, 8, || value)
    }

    #[inline]
    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        let mut value: u64 = 0;
        let value_ptr: *mut u8 = (&mut value as *mut u64).cast::<u8>();
        let result_code = unsafe { get_ledger_obj_field(register_num, field_code, value_ptr, 8) };
        match_result_code_with_expected_bytes_optional(result_code, 8, || Some(value))
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
        let mut buffer = [0x00; ACCOUNT_ID_SIZE];
        let result_code = unsafe {
            get_current_ledger_obj_field(field_code, buffer.as_mut_ptr(), ACCOUNT_ID_SIZE)
        };
        match_result_code_with_expected_bytes(result_code, ACCOUNT_ID_SIZE, || buffer.into())
    }

    #[inline]
    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        let mut buffer = [0x00; ACCOUNT_ID_SIZE];
        let result_code = unsafe {
            get_current_ledger_obj_field(field_code, buffer.as_mut_ptr(), ACCOUNT_ID_SIZE)
        };
        match_result_code_with_expected_bytes_optional(result_code, ACCOUNT_ID_SIZE, || {
            Some(buffer.into())
        })
    }

    #[inline]
    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        let mut buffer = [0x00; ACCOUNT_ID_SIZE];
        let result_code = unsafe {
            get_ledger_obj_field(
                register_num,
                field_code,
                buffer.as_mut_ptr(),
                ACCOUNT_ID_SIZE,
            )
        };
        match_result_code_with_expected_bytes(result_code, ACCOUNT_ID_SIZE, || buffer.into())
    }

    #[inline]
    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        let mut buffer = [0x00; ACCOUNT_ID_SIZE];
        let result_code = unsafe {
            get_ledger_obj_field(
                register_num,
                field_code,
                buffer.as_mut_ptr(),
                ACCOUNT_ID_SIZE,
            )
        };
        match_result_code_with_expected_bytes_optional(result_code, ACCOUNT_ID_SIZE, || {
            Some(buffer.into())
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
        let mut buffer = [0u8; HASH128_SIZE];
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, buffer.as_mut_ptr(), HASH128_SIZE) };
        match_result_code_with_expected_bytes(result_code, HASH128_SIZE, || Hash128::from(buffer))
    }

    #[inline]
    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        let mut buffer = [0u8; HASH128_SIZE];
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, buffer.as_mut_ptr(), HASH128_SIZE) };
        match_result_code_with_expected_bytes_optional(result_code, HASH128_SIZE, || {
            Some(Hash128::from(buffer))
        })
    }

    #[inline]
    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        let mut buffer = [0u8; HASH128_SIZE];
        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, buffer.as_mut_ptr(), HASH128_SIZE)
        };
        match_result_code_with_expected_bytes(result_code, HASH128_SIZE, || Hash128::from(buffer))
    }

    #[inline]
    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        let mut buffer = [0u8; HASH128_SIZE];
        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, buffer.as_mut_ptr(), HASH128_SIZE)
        };
        match_result_code_with_expected_bytes_optional(result_code, HASH128_SIZE, || {
            Some(Hash128::from(buffer))
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
        let mut buffer = [0u8; HASH256_SIZE];
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, buffer.as_mut_ptr(), HASH256_SIZE) };
        match_result_code_with_expected_bytes(result_code, HASH256_SIZE, || Hash256::from(buffer))
    }

    #[inline]
    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        let mut buffer = [0u8; HASH256_SIZE];
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, buffer.as_mut_ptr(), HASH256_SIZE) };
        match_result_code_with_expected_bytes_optional(result_code, HASH256_SIZE, || {
            Some(Hash256::from(buffer))
        })
    }

    #[inline]
    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        let mut buffer = [0u8; HASH256_SIZE];
        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, buffer.as_mut_ptr(), HASH256_SIZE)
        };
        match_result_code_with_expected_bytes(result_code, HASH256_SIZE, || Hash256::from(buffer))
    }

    #[inline]
    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        let mut buffer = [0u8; HASH256_SIZE];
        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, buffer.as_mut_ptr(), HASH256_SIZE)
        };
        match_result_code_with_expected_bytes_optional(result_code, HASH256_SIZE, || {
            Some(Hash256::from(buffer))
        })
    }
}

const BLOB_BUFFER_SIZE: usize = 1024;

/// Implementation of `FieldGetter` for variable-length binary data.
///
/// This implementation handles blob fields in XRPL ledger objects, which can contain
/// arbitrary binary data such as memos, signatures, public keys, and other
/// variable-length content.
///
/// # Buffer Management
///
/// Uses a 1024-byte buffer to accommodate most blob field sizes. The actual
/// length of the data is determined by the return value from the host function
/// and stored in the Blob's `len` field. No strict byte count validation is
/// performed since blobs can vary significantly in size.
impl FieldGetter for Blob {
    #[inline]
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        let mut buffer = core::mem::MaybeUninit::<[u8; BLOB_BUFFER_SIZE]>::uninit();
        let result_code = unsafe {
            get_current_ledger_obj_field(field_code, buffer.as_mut_ptr().cast(), BLOB_BUFFER_SIZE)
        };
        match_result_code(result_code, || Blob {
            data: unsafe { buffer.assume_init() },
            len: result_code as usize,
        })
    }

    #[inline]
    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        let mut buffer = core::mem::MaybeUninit::<[u8; BLOB_BUFFER_SIZE]>::uninit();
        let result_code = unsafe {
            get_current_ledger_obj_field(field_code, buffer.as_mut_ptr().cast(), BLOB_BUFFER_SIZE)
        };
        match_result_code_optional(result_code, || {
            Some(Blob {
                data: unsafe { buffer.assume_init() },
                len: result_code as usize,
            })
        })
    }

    #[inline]
    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        let mut buffer = core::mem::MaybeUninit::<[u8; BLOB_BUFFER_SIZE]>::uninit();
        let result_code = unsafe {
            get_ledger_obj_field(
                register_num,
                field_code,
                buffer.as_mut_ptr().cast(),
                BLOB_BUFFER_SIZE,
            )
        };
        match_result_code(result_code, || Blob {
            data: unsafe { buffer.assume_init() },
            len: result_code as usize,
        })
    }

    #[inline]
    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        let mut buffer = core::mem::MaybeUninit::<[u8; BLOB_BUFFER_SIZE]>::uninit();
        let result_code = unsafe {
            get_ledger_obj_field(
                register_num,
                field_code,
                buffer.as_mut_ptr().cast(),
                BLOB_BUFFER_SIZE,
            )
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

    /// Retrieves a field from the current ledger object.
    ///
    /// # Arguments
    ///
    /// * `field_code` - The field code identifying which field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<T>` where:
    /// * `Ok(T)` - The field value for the specified field
    /// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
    #[inline]
    pub fn get_field<T: FieldGetter>(field_code: i32) -> Result<T> {
        T::get_from_current_ledger_obj(field_code)
    }

    /// Retrieves an optionally present field from the current ledger object.
    ///
    /// # Arguments
    ///
    /// * `field_code` - The field code identifying which field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<T>>` where:
    /// * `Ok(Some(T))` - The field value for the specified field
    /// * `Ok(None)` - If the field is not present
    /// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
    #[inline]
    pub fn get_field_optional<T: FieldGetter>(field_code: i32) -> Result<Option<T>> {
        T::get_from_current_ledger_obj_optional(field_code)
    }
}

pub mod ledger_object {
    use super::FieldGetter;
    use crate::host::Result;

    /// Retrieves a field from a specified ledger object.
    ///
    /// # Arguments
    ///
    /// * `register_num` - The register number holding the ledger object to look for data in
    /// * `field_code` - The field code identifying which field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<T>` where:
    /// * `Ok(T)` - The field value for the specified field
    /// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
    #[inline]
    pub fn get_field<T: FieldGetter>(register_num: i32, field_code: i32) -> Result<T> {
        T::get_from_ledger_obj(register_num, field_code)
    }

    /// Retrieves an optionally present field from a specified ledger object.
    ///
    /// # Arguments
    ///
    /// * `register_num` - The register number holding the ledger object to look for data in
    /// * `field_code` - The field code identifying which field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<T>>` where:
    /// * `Ok(Some(T))` - The field value for the specified field
    /// * `Ok(None)` - If the field is not present in the ledger object
    /// * `Err(Error)` - If the field retrieval operation failed
    #[inline]
    pub fn get_field_optional<T: FieldGetter>(
        register_num: i32,
        field_code: i32,
    ) -> Result<Option<T>> {
        T::get_from_ledger_obj_optional(register_num, field_code)
    }
}

#[cfg(test)]
mod tests {
    use super::ledger_object;
    use crate::sfield;

    #[test]
    fn test_get_field_compilation() {
        // This test verifies that the get_field function compiles with the expected types
        // We can't actually run these functions without a proper host environment,
        // but we can verify they compile correctly

        let slot = 0;

        // Test the user's requested usage patterns
        let _balance_call = || -> crate::host::Result<u64> {
            ledger_object::get_field::<u64>(slot, sfield::Balance)
        };

        let _sequence_call = || -> crate::host::Result<u32> {
            ledger_object::get_field::<u32>(slot, sfield::Sequence)
        };

        // Test with other types to ensure the trait implementations work
        let _account_call = || -> crate::host::Result<crate::core::types::account_id::AccountID> {
            ledger_object::get_field(slot, sfield::Account)
        };

        let _amount_call = || -> crate::host::Result<crate::core::types::amount::Amount> {
            ledger_object::get_field(slot, sfield::Amount)
        };

        // Test optional variants
        let _optional_balance_call = || -> crate::host::Result<Option<u64>> {
            ledger_object::get_field_optional::<u64>(slot, sfield::Balance)
        };

        let _optional_sequence_call = || -> crate::host::Result<Option<u32>> {
            ledger_object::get_field_optional::<u32>(slot, sfield::Sequence)
        };
    }

    #[test]
    fn test_exact_user_pattern() {
        // Test the exact pattern the user requested
        let slot = 0;

        // These should compile exactly as the user specified
        let _balance = ledger_object::get_field::<u64>(slot, sfield::Balance);
        let _sequence = ledger_object::get_field::<u32>(slot, sfield::Sequence);

        // Also test that Balance should work with Amount type (which is more correct)
        let _balance_amount =
            ledger_object::get_field::<crate::core::types::amount::Amount>(slot, sfield::Balance);
    }
}
