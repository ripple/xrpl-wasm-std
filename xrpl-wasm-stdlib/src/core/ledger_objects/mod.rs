pub mod account_root;
pub mod current_escrow;
pub mod escrow;
pub mod traits;

use crate::core::types::account_id::{ACCOUNT_ID_SIZE, AccountID};
use crate::core::types::amount::Amount;
use crate::core::types::blob::Blob;
use crate::core::types::currency::Currency;
use crate::core::types::issue::{IouIssue, Issue, MptIssue, XrpIssue};
use crate::core::types::mpt_id::MptId;
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

/// Trait for types that can be retrieved as fixed-size fields from ledger objects.
///
/// This trait enables a generic implementation of `FieldGetter` for all fixed-size
/// unsigned integer types (u8, u16, u32, u64). Types implementing this trait must
/// have a known, constant size in bytes.
///
/// # Implementing Types
///
/// - `u8` - 1 byte
/// - `u16` - 2 bytes
/// - `u32` - 4 bytes
/// - `u64` - 8 bytes
trait FixedSizeFieldType: Sized {
    /// The size of this type in bytes
    const SIZE: usize;
}

impl FixedSizeFieldType for u8 {
    const SIZE: usize = 1;
}

impl FixedSizeFieldType for u16 {
    const SIZE: usize = 2;
}

impl FixedSizeFieldType for u32 {
    const SIZE: usize = 4;
}

impl FixedSizeFieldType for u64 {
    const SIZE: usize = 8;
}

/// Generic implementation of `FieldGetter` for all fixed-size unsigned integer types.
///
/// This single implementation handles u8, u16, u32, and u64 by leveraging the
/// `FixedSizeFieldType` trait. The implementation:
/// - Allocates a buffer of the appropriate size
/// - Calls the host function to retrieve the field
/// - Validates that the returned byte count matches the expected size
/// - Converts the buffer to the target type
///
/// # Buffer Management
///
/// Uses `MaybeUninit` for efficient stack allocation without initialization overhead.
/// The buffer size is determined at compile-time via the `SIZE` constant.
impl<T: FixedSizeFieldType> FieldGetter for T {
    #[inline]
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        let mut value = core::mem::MaybeUninit::<T>::uninit();
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, value.as_mut_ptr().cast(), T::SIZE) };
        match_result_code_with_expected_bytes(result_code, T::SIZE, || unsafe {
            value.assume_init()
        })
    }

    #[inline]
    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        let mut value = core::mem::MaybeUninit::<T>::uninit();
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, value.as_mut_ptr().cast(), T::SIZE) };
        match_result_code_with_expected_bytes_optional(result_code, T::SIZE, || {
            Some(unsafe { value.assume_init() })
        })
    }

    #[inline]
    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        let mut value = core::mem::MaybeUninit::<T>::uninit();
        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, value.as_mut_ptr().cast(), T::SIZE)
        };
        match_result_code_with_expected_bytes(result_code, T::SIZE, || unsafe {
            value.assume_init()
        })
    }

    #[inline]
    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        let mut value = core::mem::MaybeUninit::<T>::uninit();
        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, value.as_mut_ptr().cast(), T::SIZE)
        };
        match_result_code_with_expected_bytes_optional(result_code, T::SIZE, || {
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

/// Implementation of `FieldGetter` for XRPL issues.
///
/// This implementation handles issue fields in XRPL ledger objects.
/// Supports all three Issue variants: XRP, IOU, and MPT.
///
/// # Buffer Management
///
/// Uses a 40-byte buffer to accommodate all Issue types:
/// - XRP: 20 bytes (all zeros)
/// - IOU: 40 bytes (20 bytes currency + 20 bytes issuer)
/// - MPT: 24 bytes (4 bytes sequence + 20 bytes issuer)
///
/// The implementation detects the Issue type based on the number of bytes returned
/// from the host function.
impl FieldGetter for Issue {
    #[inline]
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        let mut buffer = core::mem::MaybeUninit::<[u8; 40]>::uninit();
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, buffer.as_mut_ptr().cast(), 40) };

        // Detect Issue type based on returned byte count
        match result_code {
            20 => {
                // XRP Issue (20 bytes)
                Result::Ok(Issue::XRP(XrpIssue {}))
            }
            24 => {
                // MPT Issue (24 bytes: 4-byte sequence + 20-byte issuer)
                let buffer = unsafe { buffer.assume_init() };
                let mpt_bytes: [u8; 24] = buffer[..24].try_into().unwrap_or([0u8; 24]);
                let mpt_id = MptId::from(mpt_bytes);
                Result::Ok(Issue::MPT(MptIssue::new(mpt_id)))
            }
            40 => {
                // IOU Issue (40 bytes: 20-byte currency + 20-byte issuer)
                let buffer = unsafe { buffer.assume_init() };
                let currency_bytes: [u8; 20] = buffer[..20].try_into().unwrap_or([0u8; 20]);
                let issuer_bytes: [u8; 20] = buffer[20..40].try_into().unwrap_or([0u8; 20]);
                let currency = Currency::from(currency_bytes);
                let issuer = AccountID::from(issuer_bytes);
                Result::Ok(Issue::IOU(IouIssue::new(issuer, currency)))
            }
            _ => {
                // Unexpected size, treat as error
                Result::Err(crate::host::Error::from_code(result_code))
            }
        }
    }

    #[inline]
    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        let mut buffer = core::mem::MaybeUninit::<[u8; 40]>::uninit();
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, buffer.as_mut_ptr().cast(), 40) };

        // Detect Issue type based on returned byte count
        match result_code {
            20 => {
                // XRP Issue (20 bytes)
                Result::Ok(Some(Issue::XRP(XrpIssue {})))
            }
            24 => {
                // MPT Issue (24 bytes: 4-byte sequence + 20-byte issuer)
                let buffer = unsafe { buffer.assume_init() };
                let mpt_bytes: [u8; 24] = buffer[..24].try_into().unwrap_or([0u8; 24]);
                let mpt_id = MptId::from(mpt_bytes);
                Result::Ok(Some(Issue::MPT(MptIssue::new(mpt_id))))
            }
            40 => {
                // IOU Issue (40 bytes: 20-byte currency + 20-byte issuer)
                let buffer = unsafe { buffer.assume_init() };
                let currency_bytes: [u8; 20] = buffer[..20].try_into().unwrap_or([0u8; 20]);
                let issuer_bytes: [u8; 20] = buffer[20..40].try_into().unwrap_or([0u8; 20]);
                let currency = Currency::from(currency_bytes);
                let issuer = AccountID::from(issuer_bytes);
                Result::Ok(Some(Issue::IOU(IouIssue::new(issuer, currency))))
            }
            code if code < 0 => {
                // Negative code indicates field not found or error
                Result::Ok(None)
            }
            _ => {
                // Unexpected size, treat as error
                Result::Err(crate::host::Error::from_code(result_code))
            }
        }
    }

    #[inline]
    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        let mut buffer = core::mem::MaybeUninit::<[u8; 40]>::uninit();
        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, buffer.as_mut_ptr().cast(), 40)
        };

        // Detect Issue type based on returned byte count
        match result_code {
            20 => {
                // XRP Issue (20 bytes)
                Result::Ok(Issue::XRP(XrpIssue {}))
            }
            24 => {
                // MPT Issue (24 bytes: 4-byte sequence + 20-byte issuer)
                let buffer = unsafe { buffer.assume_init() };
                let mpt_bytes: [u8; 24] = buffer[..24].try_into().unwrap_or([0u8; 24]);
                let mpt_id = MptId::from(mpt_bytes);
                Result::Ok(Issue::MPT(MptIssue::new(mpt_id)))
            }
            40 => {
                // IOU Issue (40 bytes: 20-byte currency + 20-byte issuer)
                let buffer = unsafe { buffer.assume_init() };
                let currency_bytes: [u8; 20] = buffer[..20].try_into().unwrap_or([0u8; 20]);
                let issuer_bytes: [u8; 20] = buffer[20..40].try_into().unwrap_or([0u8; 20]);
                let currency = Currency::from(currency_bytes);
                let issuer = AccountID::from(issuer_bytes);
                Result::Ok(Issue::IOU(IouIssue::new(issuer, currency)))
            }
            _ => {
                // Unexpected size, treat as error
                Result::Err(crate::host::Error::from_code(result_code))
            }
        }
    }

    #[inline]
    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        let mut buffer = core::mem::MaybeUninit::<[u8; 40]>::uninit();
        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, buffer.as_mut_ptr().cast(), 40)
        };

        // Detect Issue type based on returned byte count
        match result_code {
            20 => {
                // XRP Issue (20 bytes)
                Result::Ok(Some(Issue::XRP(XrpIssue {})))
            }
            24 => {
                // MPT Issue (24 bytes: 4-byte sequence + 20-byte issuer)
                let buffer = unsafe { buffer.assume_init() };
                let mpt_bytes: [u8; 24] = buffer[..24].try_into().unwrap_or([0u8; 24]);
                let mpt_id = MptId::from(mpt_bytes);
                Result::Ok(Some(Issue::MPT(MptIssue::new(mpt_id))))
            }
            40 => {
                // IOU Issue (40 bytes: 20-byte currency + 20-byte issuer)
                let buffer = unsafe { buffer.assume_init() };
                let currency_bytes: [u8; 20] = buffer[..20].try_into().unwrap_or([0u8; 20]);
                let issuer_bytes: [u8; 20] = buffer[20..40].try_into().unwrap_or([0u8; 20]);
                let currency = Currency::from(currency_bytes);
                let issuer = AccountID::from(issuer_bytes);
                Result::Ok(Some(Issue::IOU(IouIssue::new(issuer, currency))))
            }
            code if code < 0 => {
                // Negative code indicates field not found or error
                Result::Ok(None)
            }
            _ => {
                // Unexpected size, treat as error
                Result::Err(crate::host::Error::from_code(result_code))
            }
        }
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
        // Basic smoke tests for FieldGetter implementations
        // These tests verify that the trait implementations compile and work with the test host.
        // Note: The test host returns buffer_len as success, so these only verify basic functionality.
        // ========================================

        #[test]
        fn test_field_getter_basic_types() {
            // Test that all basic integer types work
            assert!(u16::get_from_current_ledger_obj(sfield::LedgerEntryType).is_ok());
            assert!(u32::get_from_current_ledger_obj(sfield::Flags).is_ok());
            assert!(u64::get_from_current_ledger_obj(sfield::Balance).is_ok());
        }

        #[test]
        fn test_field_getter_xrpl_types() {
            // Test that XRPL-specific types work
            assert!(AccountID::get_from_current_ledger_obj(sfield::Account).is_ok());
            assert!(Amount::get_from_current_ledger_obj(sfield::Amount).is_ok());
            assert!(Hash128::get_from_current_ledger_obj(sfield::EmailHash).is_ok());
            assert!(Hash256::get_from_current_ledger_obj(sfield::PreviousTxnID).is_ok());

            let blob = Blob::get_from_current_ledger_obj(sfield::PublicKey).unwrap();
            // The test host returns buffer length as the result
            assert_eq!(blob.len, BLOB_BUFFER_SIZE);
        }

        #[test]
        fn test_field_getter_optional_variants() {
            // Test optional field retrieval
            let result = u32::get_from_current_ledger_obj_optional(sfield::Flags);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());

            let result = AccountID::get_from_current_ledger_obj_optional(sfield::Account);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        #[test]
        fn test_field_getter_with_slot() {
            // Test ledger object field retrieval with slot numbers
            let slot = 0;
            assert!(u32::get_from_ledger_obj(slot, sfield::Flags).is_ok());
            assert!(u64::get_from_ledger_obj(slot, sfield::Balance).is_ok());
            assert!(AccountID::get_from_ledger_obj(slot, sfield::Account).is_ok());
        }

        #[test]
        fn test_field_getter_optional_with_slot() {
            // Test optional field retrieval with slot numbers
            let slot = 0;
            let result = u32::get_from_ledger_obj_optional(slot, sfield::Flags);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        // ========================================
        // Tests for module-level convenience functions
        // ========================================

        #[test]
        fn test_current_ledger_object_module() {
            // Test the current_ledger_object module's convenience functions
            assert!(current_ledger_object::get_field::<u32>(sfield::Flags).is_ok());
            assert!(current_ledger_object::get_field::<AccountID>(sfield::Account).is_ok());

            let result = current_ledger_object::get_field_optional::<u32>(sfield::Flags);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }

        #[test]
        fn test_ledger_object_module() {
            // Test the ledger_object module's convenience functions
            let slot = 0;
            assert!(ledger_object::get_field::<u16>(slot, sfield::LedgerEntryType).is_ok());
            assert!(ledger_object::get_field::<u32>(slot, sfield::Flags).is_ok());
            assert!(ledger_object::get_field::<u64>(slot, sfield::Balance).is_ok());
            assert!(ledger_object::get_field::<AccountID>(slot, sfield::Account).is_ok());
            assert!(ledger_object::get_field::<Amount>(slot, sfield::Amount).is_ok());
            assert!(ledger_object::get_field::<Hash128>(slot, sfield::EmailHash).is_ok());
            assert!(ledger_object::get_field::<Hash256>(slot, sfield::PreviousTxnID).is_ok());
            assert!(ledger_object::get_field::<Blob>(slot, sfield::PublicKey).is_ok());

            let result = ledger_object::get_field_optional::<u32>(slot, sfield::Flags);
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
            let _balance = ledger_object::get_field::<u64>(slot, sfield::Balance);
            let _account = ledger_object::get_field::<AccountID>(slot, sfield::Account);

            // Verify type inference works with type annotations
            let _sequence: Result<u32> = ledger_object::get_field(slot, sfield::Sequence);
            let _flags: Result<u32> = ledger_object::get_field(slot, sfield::Flags);
        }

        // ========================================
        // Data size verification tests
        // ========================================

        #[test]
        fn test_type_sizes() {
            // Verify that returned types have the expected sizes
            let hash128 = Hash128::get_from_current_ledger_obj(sfield::EmailHash).unwrap();
            assert_eq!(hash128.as_bytes().len(), HASH128_SIZE);

            let hash256 = Hash256::get_from_current_ledger_obj(sfield::PreviousTxnID).unwrap();
            assert_eq!(hash256.as_bytes().len(), HASH256_SIZE);

            let account = AccountID::get_from_current_ledger_obj(sfield::Account).unwrap();
            assert_eq!(account.0.len(), ACCOUNT_ID_SIZE);

            let blob = Blob::get_from_current_ledger_obj(sfield::PublicKey).unwrap();
            // In test environment, host returns buffer size as result code
            assert_eq!(blob.len, BLOB_BUFFER_SIZE);
            assert_eq!(blob.data.len(), BLOB_BUFFER_SIZE);
        }
    }
}
