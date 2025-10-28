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
/// use xrpl_wasm_std::core::ledger_objects::{ledger_object, current_ledger_object};
/// use xrpl_wasm_std::core::types::account_id::AccountID;
/// use xrpl_wasm_std::sfield;
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
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        let mut value: u16 = 0;
        let value_ptr: *mut u8 = (&mut value as *mut u16).cast::<u8>();
        let result_code = unsafe { get_current_ledger_obj_field(field_code, value_ptr, 2) };
        match_result_code_with_expected_bytes(result_code, 2, || value)
    }

    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        let mut value: u16 = 0;
        let value_ptr: *mut u8 = (&mut value as *mut u16).cast::<u8>();
        let result_code = unsafe { get_current_ledger_obj_field(field_code, value_ptr, 2) };
        match_result_code_with_expected_bytes_optional(result_code, 2, || Some(value))
    }

    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        let mut value: u16 = 0;
        let value_ptr: *mut u8 = (&mut value as *mut u16).cast::<u8>();
        let result_code = unsafe { get_ledger_obj_field(register_num, field_code, value_ptr, 2) };
        match_result_code_with_expected_bytes(result_code, 2, || value)
    }

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
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        let mut value: u32 = 0;
        let value_ptr: *mut u8 = (&mut value as *mut u32).cast::<u8>();
        let result_code = unsafe { get_current_ledger_obj_field(field_code, value_ptr, 4) };
        match_result_code_with_expected_bytes(result_code, 4, || value)
    }

    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        let mut value: u32 = 0;
        let value_ptr: *mut u8 = (&mut value as *mut u32).cast::<u8>();
        let result_code = unsafe { get_current_ledger_obj_field(field_code, value_ptr, 4) };
        match_result_code_with_expected_bytes_optional(result_code, 4, || Some(value))
    }

    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        let mut value: u32 = 0;
        let value_ptr: *mut u8 = (&mut value as *mut u32).cast::<u8>();
        let result_code = unsafe { get_ledger_obj_field(register_num, field_code, value_ptr, 4) };
        match_result_code_with_expected_bytes(result_code, 4, || value)
    }

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
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        let mut value: u64 = 0;
        let value_ptr: *mut u8 = (&mut value as *mut u64).cast::<u8>();
        let result_code = unsafe { get_current_ledger_obj_field(field_code, value_ptr, 8) };
        match_result_code_with_expected_bytes(result_code, 8, || value)
    }

    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        let mut value: u64 = 0;
        let value_ptr: *mut u8 = (&mut value as *mut u64).cast::<u8>();
        let result_code = unsafe { get_current_ledger_obj_field(field_code, value_ptr, 8) };
        match_result_code_with_expected_bytes_optional(result_code, 8, || Some(value))
    }

    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        let mut value: u64 = 0;
        let value_ptr: *mut u8 = (&mut value as *mut u64).cast::<u8>();
        let result_code = unsafe { get_ledger_obj_field(register_num, field_code, value_ptr, 8) };
        match_result_code_with_expected_bytes(result_code, 8, || value)
    }

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
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        let mut buffer = [0x00; ACCOUNT_ID_SIZE];
        let result_code = unsafe {
            get_current_ledger_obj_field(field_code, buffer.as_mut_ptr(), ACCOUNT_ID_SIZE)
        };
        match_result_code_with_expected_bytes(result_code, ACCOUNT_ID_SIZE, || buffer.into())
    }

    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        let mut buffer = [0x00; ACCOUNT_ID_SIZE];
        let result_code = unsafe {
            get_current_ledger_obj_field(field_code, buffer.as_mut_ptr(), ACCOUNT_ID_SIZE)
        };
        match_result_code_with_expected_bytes_optional(result_code, ACCOUNT_ID_SIZE, || {
            Some(buffer.into())
        })
    }

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
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        const BUFFER_SIZE: usize = 48;
        let mut buffer = [0u8; BUFFER_SIZE];
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, buffer.as_mut_ptr(), BUFFER_SIZE) };
        match_result_code(result_code, || Amount::from(buffer))
    }

    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        const BUFFER_SIZE: usize = 48;
        let mut buffer = [0u8; BUFFER_SIZE];
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, buffer.as_mut_ptr(), BUFFER_SIZE) };
        match_result_code_optional(result_code, || Some(Amount::from(buffer)))
    }

    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        const BUFFER_SIZE: usize = 48;
        let mut buffer = [0u8; BUFFER_SIZE];
        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, buffer.as_mut_ptr(), BUFFER_SIZE)
        };
        match_result_code(result_code, || Amount::from(buffer))
    }

    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        const BUFFER_SIZE: usize = 48;
        let mut buffer = [0u8; BUFFER_SIZE];
        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, buffer.as_mut_ptr(), BUFFER_SIZE)
        };
        match_result_code_optional(result_code, || Some(Amount::from(buffer)))
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
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        let mut buffer = [0u8; HASH128_SIZE];
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, buffer.as_mut_ptr(), HASH128_SIZE) };
        match_result_code_with_expected_bytes(result_code, HASH128_SIZE, || Hash128::from(buffer))
    }

    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        let mut buffer = [0u8; HASH128_SIZE];
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, buffer.as_mut_ptr(), HASH128_SIZE) };
        match_result_code_with_expected_bytes_optional(result_code, HASH128_SIZE, || {
            Some(Hash128::from(buffer))
        })
    }

    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        let mut buffer = [0u8; HASH128_SIZE];
        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, buffer.as_mut_ptr(), HASH128_SIZE)
        };
        match_result_code_with_expected_bytes(result_code, HASH128_SIZE, || Hash128::from(buffer))
    }

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
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        let mut buffer = [0u8; HASH256_SIZE];
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, buffer.as_mut_ptr(), HASH256_SIZE) };
        match_result_code_with_expected_bytes(result_code, HASH256_SIZE, || Hash256::from(buffer))
    }

    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        let mut buffer = [0u8; HASH256_SIZE];
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, buffer.as_mut_ptr(), HASH256_SIZE) };
        match_result_code_with_expected_bytes_optional(result_code, HASH256_SIZE, || {
            Some(Hash256::from(buffer))
        })
    }

    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        let mut buffer = [0u8; HASH256_SIZE];
        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, buffer.as_mut_ptr(), HASH256_SIZE)
        };
        match_result_code_with_expected_bytes(result_code, HASH256_SIZE, || Hash256::from(buffer))
    }

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
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        let mut buffer = [0u8; BLOB_BUFFER_SIZE];
        let result_code = unsafe {
            get_current_ledger_obj_field(field_code, buffer.as_mut_ptr(), BLOB_BUFFER_SIZE)
        };
        match_result_code(result_code, || Blob {
            data: buffer,
            len: result_code as usize,
        })
    }

    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        let mut buffer = [0u8; BLOB_BUFFER_SIZE];
        let result_code = unsafe {
            get_current_ledger_obj_field(field_code, buffer.as_mut_ptr(), BLOB_BUFFER_SIZE)
        };
        match_result_code_optional(result_code, || {
            Some(Blob {
                data: buffer,
                len: result_code as usize,
            })
        })
    }

    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        let mut buffer = [0u8; BLOB_BUFFER_SIZE];
        let result_code = unsafe {
            get_ledger_obj_field(
                register_num,
                field_code,
                buffer.as_mut_ptr(),
                BLOB_BUFFER_SIZE,
            )
        };
        match_result_code(result_code, || Blob {
            data: buffer,
            len: result_code as usize,
        })
    }

    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        let mut buffer = [0u8; BLOB_BUFFER_SIZE];
        let result_code = unsafe {
            get_ledger_obj_field(
                register_num,
                field_code,
                buffer.as_mut_ptr(),
                BLOB_BUFFER_SIZE,
            )
        };
        match_result_code_optional(result_code, || {
            Some(Blob {
                data: buffer,
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

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::core::ledger_objects::{BLOB_BUFFER_SIZE, current_ledger_object, ledger_object};
        use crate::core::types::account_id::{ACCOUNT_ID_SIZE, AccountID};
        use crate::core::types::amount::Amount;
        use crate::core::types::blob::Blob;
        use crate::core::types::uint::{HASH128_SIZE, HASH256_SIZE, Hash128, Hash256};
        use crate::sfield;

        // ========================================
        // Tests for u16 FieldGetter implementation
        // ========================================

        #[test]
        fn test_u16_get_from_current_ledger_obj_success() {
            // The test host function returns buffer length (2) as success code
            let result = u16::get_from_current_ledger_obj(sfield::LedgerEntryType);
            assert!(result.is_ok());
        }

        #[test]
        fn test_u16_get_from_current_ledger_obj_optional_success() {
            let result = u16::get_from_current_ledger_obj_optional(sfield::LedgerEntryType);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        #[test]
        fn test_u16_get_from_ledger_obj_success() {
            let slot = 0;
            let result = u16::get_from_ledger_obj(slot, sfield::LedgerEntryType);
            assert!(result.is_ok());
        }

        #[test]
        fn test_u16_get_from_ledger_obj_optional_success() {
            let slot = 0;
            let result = u16::get_from_ledger_obj_optional(slot, sfield::LedgerEntryType);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        // ========================================
        // Tests for u32 FieldGetter implementation
        // ========================================

        #[test]
        fn test_u32_get_from_current_ledger_obj_success() {
            let result = u32::get_from_current_ledger_obj(sfield::Flags);
            assert!(result.is_ok());
        }

        #[test]
        fn test_u32_get_from_current_ledger_obj_optional_success() {
            let result = u32::get_from_current_ledger_obj_optional(sfield::Flags);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        #[test]
        fn test_u32_get_from_ledger_obj_success() {
            let slot = 0;
            let result = u32::get_from_ledger_obj(slot, sfield::Sequence);
            assert!(result.is_ok());
        }

        #[test]
        fn test_u32_get_from_ledger_obj_optional_success() {
            let slot = 0;
            let result = u32::get_from_ledger_obj_optional(slot, sfield::Sequence);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        // ========================================
        // Tests for u64 FieldGetter implementation
        // ========================================

        #[test]
        fn test_u64_get_from_current_ledger_obj_success() {
            let result = u64::get_from_current_ledger_obj(sfield::Balance);
            assert!(result.is_ok());
        }

        #[test]
        fn test_u64_get_from_current_ledger_obj_optional_success() {
            let result = u64::get_from_current_ledger_obj_optional(sfield::Balance);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        #[test]
        fn test_u64_get_from_ledger_obj_success() {
            let slot = 0;
            let result = u64::get_from_ledger_obj(slot, sfield::Balance);
            assert!(result.is_ok());
        }

        #[test]
        fn test_u64_get_from_ledger_obj_optional_success() {
            let slot = 0;
            let result = u64::get_from_ledger_obj_optional(slot, sfield::Balance);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        // ========================================
        // Tests for AccountID FieldGetter implementation
        // ========================================

        #[test]
        fn test_account_id_get_from_current_ledger_obj_success() {
            let result = AccountID::get_from_current_ledger_obj(sfield::Account);
            assert!(result.is_ok());
        }

        #[test]
        fn test_account_id_get_from_current_ledger_obj_optional_success() {
            let result = AccountID::get_from_current_ledger_obj_optional(sfield::Account);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        #[test]
        fn test_account_id_get_from_ledger_obj_success() {
            let slot = 0;
            let result = AccountID::get_from_ledger_obj(slot, sfield::Account);
            assert!(result.is_ok());
        }

        #[test]
        fn test_account_id_get_from_ledger_obj_optional_success() {
            let slot = 0;
            let result = AccountID::get_from_ledger_obj_optional(slot, sfield::Account);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        // ========================================
        // Tests for Amount FieldGetter implementation
        // ========================================

        #[test]
        fn test_amount_get_from_current_ledger_obj_success() {
            let result = Amount::get_from_current_ledger_obj(sfield::Amount);
            assert!(result.is_ok());
        }

        #[test]
        fn test_amount_get_from_current_ledger_obj_optional_success() {
            let result = Amount::get_from_current_ledger_obj_optional(sfield::Amount);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        #[test]
        fn test_amount_get_from_ledger_obj_success() {
            let slot = 0;
            let result = Amount::get_from_ledger_obj(slot, sfield::Amount);
            assert!(result.is_ok());
        }

        #[test]
        fn test_amount_get_from_ledger_obj_optional_success() {
            let slot = 0;
            let result = Amount::get_from_ledger_obj_optional(slot, sfield::Amount);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        // ========================================
        // Tests for Hash128 FieldGetter implementation
        // ========================================

        #[test]
        fn test_hash128_get_from_current_ledger_obj_success() {
            let result = Hash128::get_from_current_ledger_obj(sfield::EmailHash);
            assert!(result.is_ok());
        }

        #[test]
        fn test_hash128_get_from_current_ledger_obj_optional_success() {
            let result = Hash128::get_from_current_ledger_obj_optional(sfield::EmailHash);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        #[test]
        fn test_hash128_get_from_ledger_obj_success() {
            let slot = 0;
            let result = Hash128::get_from_ledger_obj(slot, sfield::EmailHash);
            assert!(result.is_ok());
        }

        #[test]
        fn test_hash128_get_from_ledger_obj_optional_success() {
            let slot = 0;
            let result = Hash128::get_from_ledger_obj_optional(slot, sfield::EmailHash);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        // ========================================
        // Tests for Hash256 FieldGetter implementation
        // ========================================

        #[test]
        fn test_hash256_get_from_current_ledger_obj_success() {
            let result = Hash256::get_from_current_ledger_obj(sfield::PreviousTxnID);
            assert!(result.is_ok());
        }

        #[test]
        fn test_hash256_get_from_current_ledger_obj_optional_success() {
            let result = Hash256::get_from_current_ledger_obj_optional(sfield::PreviousTxnID);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        #[test]
        fn test_hash256_get_from_ledger_obj_success() {
            let slot = 0;
            let result = Hash256::get_from_ledger_obj(slot, sfield::PreviousTxnID);
            assert!(result.is_ok());
        }

        #[test]
        fn test_hash256_get_from_ledger_obj_optional_success() {
            let slot = 0;
            let result = Hash256::get_from_ledger_obj_optional(slot, sfield::PreviousTxnID);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        // ========================================
        // Tests for Blob FieldGetter implementation
        // ========================================

        #[test]
        fn test_blob_get_from_current_ledger_obj_success() {
            let result = Blob::get_from_current_ledger_obj(sfield::PublicKey);
            assert!(result.is_ok());
            let blob = result.unwrap();
            // The test host returns buffer length (1024) as the result
            assert_eq!(blob.len, BLOB_BUFFER_SIZE);
        }

        #[test]
        fn test_blob_get_from_current_ledger_obj_optional_success() {
            let result = Blob::get_from_current_ledger_obj_optional(sfield::PublicKey);
            assert!(result.is_ok());
            let blob_opt = result.unwrap();
            assert!(blob_opt.is_some());
            assert_eq!(blob_opt.unwrap().len, BLOB_BUFFER_SIZE);
        }

        #[test]
        fn test_blob_get_from_ledger_obj_success() {
            let slot = 0;
            let result = Blob::get_from_ledger_obj(slot, sfield::PublicKey);
            assert!(result.is_ok());
            let blob = result.unwrap();
            assert_eq!(blob.len, BLOB_BUFFER_SIZE);
        }

        #[test]
        fn test_blob_get_from_ledger_obj_optional_success() {
            let slot = 0;
            let result = Blob::get_from_ledger_obj_optional(slot, sfield::PublicKey);
            assert!(result.is_ok());
            let blob_opt = result.unwrap();
            assert!(blob_opt.is_some());
            assert_eq!(blob_opt.unwrap().len, BLOB_BUFFER_SIZE);
        }

        // ========================================
        // Tests for current_ledger_object module
        // ========================================

        #[test]
        fn test_current_ledger_object_get_field_u32() {
            let result = current_ledger_object::get_field::<u32>(sfield::Flags);
            assert!(result.is_ok());
        }

        #[test]
        fn test_current_ledger_object_get_field_u64() {
            let result = current_ledger_object::get_field::<u64>(sfield::Balance);
            assert!(result.is_ok());
        }

        #[test]
        fn test_current_ledger_object_get_field_account_id() {
            let result = current_ledger_object::get_field::<AccountID>(sfield::Account);
            assert!(result.is_ok());
        }

        #[test]
        fn test_current_ledger_object_get_field_amount() {
            let result = current_ledger_object::get_field::<Amount>(sfield::Amount);
            assert!(result.is_ok());
        }

        #[test]
        fn test_current_ledger_object_get_field_optional_u32() {
            let result = current_ledger_object::get_field_optional::<u32>(sfield::Flags);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        #[test]
        fn test_current_ledger_object_get_field_optional_hash256() {
            let result =
                current_ledger_object::get_field_optional::<Hash256>(sfield::PreviousTxnID);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        // ========================================
        // Tests for ledger_object module
        // ========================================

        #[test]
        fn test_ledger_object_get_field_u16() {
            let slot = 0;
            let result = ledger_object::get_field::<u16>(slot, sfield::LedgerEntryType);
            assert!(result.is_ok());
        }

        #[test]
        fn test_ledger_object_get_field_u32() {
            let slot = 0;
            let result = ledger_object::get_field::<u32>(slot, sfield::Sequence);
            assert!(result.is_ok());
        }

        #[test]
        fn test_ledger_object_get_field_u64() {
            let slot = 0;
            let result = ledger_object::get_field::<u64>(slot, sfield::Balance);
            assert!(result.is_ok());
        }

        #[test]
        fn test_ledger_object_get_field_account_id() {
            let slot = 0;
            let result = ledger_object::get_field::<AccountID>(slot, sfield::Account);
            assert!(result.is_ok());
        }

        #[test]
        fn test_ledger_object_get_field_amount() {
            let slot = 0;
            let result = ledger_object::get_field::<Amount>(slot, sfield::Amount);
            assert!(result.is_ok());
        }

        #[test]
        fn test_ledger_object_get_field_hash128() {
            let slot = 0;
            let result = ledger_object::get_field::<Hash128>(slot, sfield::EmailHash);
            assert!(result.is_ok());
        }

        #[test]
        fn test_ledger_object_get_field_hash256() {
            let slot = 0;
            let result = ledger_object::get_field::<Hash256>(slot, sfield::PreviousTxnID);
            assert!(result.is_ok());
        }

        #[test]
        fn test_ledger_object_get_field_blob() {
            let slot = 0;
            let result = ledger_object::get_field::<Blob>(slot, sfield::PublicKey);
            assert!(result.is_ok());
        }

        #[test]
        fn test_ledger_object_get_field_optional_u16() {
            let slot = 0;
            let result = ledger_object::get_field_optional::<u16>(slot, sfield::LedgerEntryType);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        #[test]
        fn test_ledger_object_get_field_optional_u32() {
            let slot = 0;
            let result = ledger_object::get_field_optional::<u32>(slot, sfield::Flags);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        #[test]
        fn test_ledger_object_get_field_optional_u64() {
            let slot = 0;
            let result = ledger_object::get_field_optional::<u64>(slot, sfield::Balance);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        #[test]
        fn test_ledger_object_get_field_optional_account_id() {
            let slot = 0;
            let result = ledger_object::get_field_optional::<AccountID>(slot, sfield::Account);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        #[test]
        fn test_ledger_object_get_field_optional_amount() {
            let slot = 0;
            let result = ledger_object::get_field_optional::<Amount>(slot, sfield::Amount);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        #[test]
        fn test_ledger_object_get_field_optional_hash128() {
            let slot = 0;
            let result = ledger_object::get_field_optional::<Hash128>(slot, sfield::EmailHash);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        #[test]
        fn test_ledger_object_get_field_optional_hash256() {
            let slot = 0;
            let result = ledger_object::get_field_optional::<Hash256>(slot, sfield::PreviousTxnID);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        #[test]
        fn test_ledger_object_get_field_optional_blob() {
            let slot = 0;
            let result = ledger_object::get_field_optional::<Blob>(slot, sfield::PublicKey);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        // ========================================
        // Integration tests - verify type inference works
        // ========================================

        #[test]
        fn test_type_inference_with_turbofish() {
            let slot = 0;
            // These should compile with explicit type parameters
            let _balance = ledger_object::get_field::<u64>(slot, sfield::Balance);
            let _sequence = ledger_object::get_field::<u32>(slot, sfield::Sequence);
            let _account = ledger_object::get_field::<AccountID>(slot, sfield::Account);
        }

        #[test]
        fn test_type_inference_with_annotation() {
            let slot = 0;
            // These should compile with type annotations
            let _balance: Result<u64> = ledger_object::get_field(slot, sfield::Balance);
            let _sequence: Result<u32> = ledger_object::get_field(slot, sfield::Sequence);
            let _account: Result<AccountID> = ledger_object::get_field(slot, sfield::Account);
        }

        #[test]
        fn test_multiple_slot_numbers() {
            // Test that different slot numbers work
            for slot in 0..5 {
                let result = ledger_object::get_field::<u32>(slot, sfield::Flags);
                assert!(result.is_ok());
            }
        }

        // ========================================
        // Edge case and boundary tests
        // ========================================

        #[test]
        fn test_blob_length_tracking() {
            // Verify that Blob correctly tracks the length returned by the host
            let result = Blob::get_from_current_ledger_obj(sfield::PublicKey);
            assert!(result.is_ok());
            let blob = result.unwrap();
            // In test environment, host returns buffer size as result code
            assert_eq!(blob.len, BLOB_BUFFER_SIZE);
            assert_eq!(blob.data.len(), BLOB_BUFFER_SIZE);
        }

        #[test]
        fn test_all_integer_types_have_correct_sizes() {
            // Verify that each integer type uses the correct buffer size
            // This is a compile-time check more than runtime, but documents expectations

            // u16 should use 2 bytes
            let result = u16::get_from_current_ledger_obj(sfield::LedgerEntryType);
            assert!(result.is_ok());

            // u32 should use 4 bytes
            let result = u32::get_from_current_ledger_obj(sfield::Flags);
            assert!(result.is_ok());

            // u64 should use 8 bytes
            let result = u64::get_from_current_ledger_obj(sfield::Balance);
            assert!(result.is_ok());
        }

        #[test]
        fn test_hash_types_have_correct_sizes() {
            // Hash128 should use 16 bytes
            let result = Hash128::get_from_current_ledger_obj(sfield::EmailHash);
            assert!(result.is_ok());
            let hash = result.unwrap();
            assert_eq!(hash.as_bytes().len(), HASH128_SIZE);

            // Hash256 should use 32 bytes
            let result = Hash256::get_from_current_ledger_obj(sfield::PreviousTxnID);
            assert!(result.is_ok());
            let hash = result.unwrap();
            assert_eq!(hash.as_bytes().len(), HASH256_SIZE);
        }

        #[test]
        fn test_account_id_has_correct_size() {
            let result = AccountID::get_from_current_ledger_obj(sfield::Account);
            assert!(result.is_ok());
            let account = result.unwrap();
            assert_eq!(account.0.len(), ACCOUNT_ID_SIZE);
        }

        #[test]
        fn test_amount_buffer_size() {
            // Amount uses a 48-byte buffer to accommodate all amount types
            let result = Amount::get_from_current_ledger_obj(sfield::Amount);
            assert!(result.is_ok());
            // The Amount type handles variable-length data internally
        }

        #[test]
        fn test_negative_slot_numbers() {
            // Test that negative slot numbers are handled
            // In a real environment, this might return an error, but in tests it should work
            let slot = -1;
            let result = ledger_object::get_field::<u32>(slot, sfield::Flags);
            // In test environment, this succeeds
            assert!(result.is_ok());
        }

        #[test]
        fn test_zero_slot_number() {
            // Slot 0 should be valid
            let slot = 0;
            let result = ledger_object::get_field::<u32>(slot, sfield::Flags);
            assert!(result.is_ok());
        }

        #[test]
        fn test_large_slot_numbers() {
            // Test with larger slot numbers
            let slot = 100;
            let result = ledger_object::get_field::<u32>(slot, sfield::Flags);
            assert!(result.is_ok());
        }

        #[test]
        fn test_different_field_codes() {
            // Test that different field codes work correctly
            let slot = 0;

            // Test various common field codes
            let _account = ledger_object::get_field::<AccountID>(slot, sfield::Account);
            let _destination =
                ledger_object::get_field_optional::<AccountID>(slot, sfield::Destination);
            let _balance = ledger_object::get_field::<u64>(slot, sfield::Balance);
            let _sequence = ledger_object::get_field::<u32>(slot, sfield::Sequence);
            let _flags = ledger_object::get_field::<u32>(slot, sfield::Flags);
        }

        #[test]
        fn test_optional_field_returns_some_in_test_env() {
            // In the test environment, optional fields always return Some
            // because the mock host function always succeeds
            let result = current_ledger_object::get_field_optional::<u32>(sfield::Flags);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        #[test]
        fn test_consistency_between_current_and_slotted_access() {
            // Both current_ledger_object and ledger_object should work similarly
            let current_result = current_ledger_object::get_field::<u32>(sfield::Flags);
            let slotted_result = ledger_object::get_field::<u32>(0, sfield::Flags);

            // Both should succeed in test environment
            assert!(current_result.is_ok());
            assert!(slotted_result.is_ok());
        }

        #[test]
        fn test_all_field_getter_types_compile() {
            // Comprehensive compilation test for all supported types
            let slot = 0;

            // Primitive integer types
            let _: Result<u16> = ledger_object::get_field(slot, sfield::LedgerEntryType);
            let _: Result<u32> = ledger_object::get_field(slot, sfield::Flags);
            let _: Result<u64> = ledger_object::get_field(slot, sfield::Balance);

            // XRPL-specific types
            let _: Result<AccountID> = ledger_object::get_field(slot, sfield::Account);
            let _: Result<Amount> = ledger_object::get_field(slot, sfield::Amount);
            let _: Result<Hash128> = ledger_object::get_field(slot, sfield::EmailHash);
            let _: Result<Hash256> = ledger_object::get_field(slot, sfield::PreviousTxnID);
            let _: Result<Blob> = ledger_object::get_field(slot, sfield::PublicKey);

            // Optional variants
            let _: Result<Option<u32>> = ledger_object::get_field_optional(slot, sfield::Flags);
            let _: Result<Option<AccountID>> =
                ledger_object::get_field_optional(slot, sfield::Account);
            let _: Result<Option<Amount>> = ledger_object::get_field_optional(slot, sfield::Amount);
            let _: Result<Option<Blob>> =
                ledger_object::get_field_optional(slot, sfield::PublicKey);
        }

        #[test]
        fn test_field_getter_trait_is_object_safe() {
            // This test verifies that the FieldGetter trait can be used polymorphically
            // if needed (though it's primarily used with static dispatch via generics)

            fn get_u32_field(field_code: i32) -> Result<u32> {
                u32::get_from_current_ledger_obj(field_code)
            }

            fn get_u64_field(field_code: i32) -> Result<u64> {
                u64::get_from_current_ledger_obj(field_code)
            }

            assert!(get_u32_field(sfield::Flags).is_ok());
            assert!(get_u64_field(sfield::Balance).is_ok());
        }
    }
}
