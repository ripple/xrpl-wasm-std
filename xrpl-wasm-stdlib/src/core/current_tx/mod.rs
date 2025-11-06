//! # Current Transaction Retrieval Module
//!
//! This module provides utilities for retrieving typed fields from the current XRPL transaction
//! within the context of XRPL Programmability. It offers a safe, type-safe
//! interface over the low-level host functions for accessing transaction data, such as from an
//! `EscrowFinish` transaction.
//!
//! ## Overview
//!
//! When processing XRPL transactions in a permissionless programmability environment, you often
//! need to extract specific fields like account IDs, hashes, public keys, and other data. This
//! module provides convenient wrapper functions that handle the low-level buffer management
//! and error handling required to safely retrieve these fields.
//!
//! ## Field Types Supported
//!
//! - **AccountID**: 20-byte account identifiers
//! - **u32**: 32-bit unsigned integers
//! - **Hash256**: 256-bit cryptographic hashes
//! - **PublicKey**: 33-byte public keys
//! - **Blob**: Variable-length binary data
//!
//! ## Optional vs Required Fields
//!
//! The module provides both optional and required variants for field retrieval:
//!
//! - **Required variants** (e.g., `get_u32_field`): Return an error if the field is missing
//! - **Optional variants** (e.g., `get_optional_u32_field`): Return `None` if the field is missing
//!
//! ## Error Handling
//!
//! All functions return `Result<T>` or `Result<Option<T>>` types that encapsulate
//! the custom error handling required for the XRPL Programmability environment.
//!
//! ## Safety Considerations
//!
//! - All functions use fixed-size buffers appropriate for their data types
//! - Buffer sizes are validated against expected field sizes
//! - Unsafe operations are contained within the low-level host function calls
//! - Memory safety is ensured through proper buffer management
//! - Field codes are validated by the underlying host functions
//!
//! ## Performance Notes
//!
//! - All functions are marked `#[inline]` to minimize call overhead
//! - Buffer allocations are stack-based and have minimal cost
//! - Host function calls are the primary performance bottleneck
//!
//! ## Example
//!
//! Get sender Account and optional flags:
//!
//! ```no_run
//! use xrpl_wasm_stdlib::core::current_tx::escrow_finish::EscrowFinish;
//! use xrpl_wasm_stdlib::core::current_tx::traits::TransactionCommonFields;
//! let tx = EscrowFinish;
//! let account = tx.get_account().unwrap_or_panic();
//! let _flags = tx.get_flags().unwrap_or_panic();
//! ```

use crate::core::types::account_id::{ACCOUNT_ID_SIZE, AccountID};
use crate::core::types::amount::{AMOUNT_SIZE, Amount};
use crate::core::types::blob::Blob;
use crate::core::types::public_key::PublicKey;
use crate::core::types::transaction_type::TransactionType;
use crate::core::types::uint::{HASH256_SIZE, Hash256};
use crate::host::field_helpers::{
    get_fixed_size_field_with_expected_bytes, get_fixed_size_field_with_expected_bytes_optional,
    get_variable_size_field, get_variable_size_field_optional,
};
use crate::host::{Result, get_tx_field};

/// Trait for types that can be retrieved from current transaction fields.
///
/// This trait provides a unified interface for retrieving typed data from the current
/// XRPL transaction being processed, replacing the previous collection of type-specific
/// functions with a generic, type-safe approach.
///
/// ## Supported Types
///
/// The following types implement this trait:
/// - `u32` - 32-bit unsigned integers for sequence numbers, flags, timestamps
/// - `AccountID` - 20-byte account identifiers for transaction participants
/// - `Amount` - XRP amounts and token amounts for transaction values
/// - `Hash256` - 256-bit hashes for transaction IDs and references
/// - `PublicKey` - 33-byte compressed public keys for cryptographic operations
/// - `Blob` - Variable-length binary data for signatures, memos, and other content
///
/// ## Usage Patterns
///
/// ```rust,no_run
/// use xrpl_wasm_stdlib::core::current_tx::{get_field, get_field_optional};
/// use xrpl_wasm_stdlib::core::types::account_id::AccountID;
/// use xrpl_wasm_stdlib::core::types::amount::Amount;
/// use xrpl_wasm_stdlib::core::types::blob::Blob;
/// use xrpl_wasm_stdlib::sfield;
/// # fn example() {
/// // Get required fields from the current transaction
/// let account: AccountID = get_field(sfield::Account).unwrap();
/// let sequence: u32 = get_field(sfield::Sequence).unwrap();
/// let fee: Amount = get_field(sfield::Fee).unwrap();
///
/// // Get optional fields from the current transaction
/// let flags: Option<u32> = get_field_optional(sfield::Flags).unwrap();
/// let memo: Option<Blob> = get_field_optional(sfield::Memo).unwrap();
/// # }
/// ```
///
/// ## Error Handling
///
/// - Required field methods return `Result<T>` and error if the field is missing
/// - Optional field methods return `Result<Option<T>>` and return `None` if the field is missing
/// - All methods return appropriate errors for buffer size mismatches or other retrieval failures
///
/// ## Transaction Context
///
/// This trait operates on the "current transaction" - the transaction currently being
/// processed in the XRPL Programmability environment. The transaction context is
/// established by the XRPL host environment before calling into WASM code.
///
/// ## Safety Considerations
///
/// - All implementations use appropriately sized buffers for their data types
/// - Buffer sizes are validated against expected field sizes where applicable
/// - Unsafe operations are contained within the host function calls
/// - Transaction field access is validated by the host environment
pub trait CurrentTxFieldGetter: Sized {
    /// Get a required field from the current transaction.
    ///
    /// This method retrieves a field that must be present in the transaction.
    /// If the field is missing, an error is returned.
    ///
    /// # Arguments
    ///
    /// * `field_code` - The field code identifying which field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<Self>` where:
    /// * `Ok(Self)` - The field value for the specified field
    /// * `Err(Error::FieldNotFound)` - If the field is not present in the transaction
    /// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
    fn get_from_current_tx(field_code: i32) -> Result<Self>;

    /// Get an optional field from the current transaction.
    ///
    /// This method retrieves a field that may or may not be present in the transaction.
    /// If the field is missing, `None` is returned rather than an error.
    ///
    /// # Arguments
    ///
    /// * `field_code` - The field code identifying which field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<Self>>` where:
    /// * `Ok(Some(Self))` - The field value for the specified field
    /// * `Ok(None)` - If the field is not present in the transaction
    /// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
    fn get_from_current_tx_optional(field_code: i32) -> Result<Option<Self>>;
}

/// Implementation of `CurrentTxFieldGetter` for 32-bit unsigned integers.
///
/// This implementation handles 4-byte integer fields in XRPL transactions.
/// Common use cases include sequence numbers, flags, timestamps, ledger sequence
/// numbers, and various counters and identifiers.
///
/// # Buffer Management
///
/// Uses a 4-byte buffer and validates that exactly 4 bytes are returned
/// from the host function. The bytes are interpreted as little-endian.
impl CurrentTxFieldGetter for u32 {
    #[inline]
    fn get_from_current_tx(field_code: i32) -> Result<Self> {
        match get_fixed_size_field_with_expected_bytes::<4, _>(field_code, |fc, buf, size| unsafe {
            get_tx_field(fc, buf, size)
        }) {
            Result::Ok(buffer) => Result::Ok(u32::from_le_bytes(buffer)),
            Result::Err(e) => Result::Err(e),
        }
    }

    #[inline]
    fn get_from_current_tx_optional(field_code: i32) -> Result<Option<Self>> {
        match get_fixed_size_field_with_expected_bytes_optional::<4, _>(
            field_code,
            |fc, buf, size| unsafe { get_tx_field(fc, buf, size) },
        ) {
            Result::Ok(buffer) => Result::Ok(buffer.map(u32::from_le_bytes)),
            Result::Err(e) => Result::Err(e),
        }
    }
}

/// Implementation of `CurrentTxFieldGetter` for XRPL account identifiers.
///
/// This implementation handles 20-byte account ID fields in XRPL transactions.
/// Account IDs identify transaction participants such as the sending account,
/// destination account, and various other account references throughout the transaction.
///
/// # Buffer Management
///
/// Uses a 20-byte buffer (ACCOUNT_ID_SIZE) and validates that exactly 20 bytes
/// are returned from the host function. The buffer is converted to an AccountID
/// using the `From<[u8; 20]>` implementation.
impl CurrentTxFieldGetter for AccountID {
    #[inline]
    fn get_from_current_tx(field_code: i32) -> Result<Self> {
        match get_fixed_size_field_with_expected_bytes::<ACCOUNT_ID_SIZE, _>(
            field_code,
            |fc, buf, size| unsafe { get_tx_field(fc, buf, size) },
        ) {
            Result::Ok(buffer) => Result::Ok(buffer.into()),
            Result::Err(e) => Result::Err(e),
        }
    }

    #[inline]
    fn get_from_current_tx_optional(field_code: i32) -> Result<Option<Self>> {
        match get_fixed_size_field_with_expected_bytes_optional::<ACCOUNT_ID_SIZE, _>(
            field_code,
            |fc, buf, size| unsafe { get_tx_field(fc, buf, size) },
        ) {
            Result::Ok(buffer) => Result::Ok(buffer.map(|b| b.into())),
            Result::Err(e) => Result::Err(e),
        }
    }
}

/// Implementation of `CurrentTxFieldGetter` for XRPL amount values.
///
/// This implementation handles amount fields in XRPL transactions, which can represent
/// either XRP amounts (8 bytes) or token amounts (up to 48 bytes including currency code
/// and issuer information). Common uses include transaction fees, payment amounts,
/// offer amounts, and escrow amounts.
///
/// # Buffer Management
///
/// Uses a 48-byte buffer (AMOUNT_SIZE) to accommodate the largest possible amount
/// representation. The Amount type handles the parsing of different amount formats
/// internally. No strict byte count validation is performed since amounts can vary in size.
impl CurrentTxFieldGetter for Amount {
    #[inline]
    fn get_from_current_tx(field_code: i32) -> Result<Self> {
        match get_variable_size_field::<AMOUNT_SIZE, _>(field_code, |fc, buf, size| unsafe {
            get_tx_field(fc, buf, size)
        }) {
            Result::Ok((buffer, _len)) => Result::Ok(Amount::from(buffer)),
            Result::Err(e) => Result::Err(e),
        }
    }

    #[inline]
    fn get_from_current_tx_optional(field_code: i32) -> Result<Option<Self>> {
        match get_variable_size_field_optional::<AMOUNT_SIZE, _>(
            field_code,
            |fc, buf, size| unsafe { get_tx_field(fc, buf, size) },
        ) {
            Result::Ok(opt) => Result::Ok(opt.map(|(buffer, _len)| Amount::from(buffer))),
            Result::Err(e) => Result::Err(e),
        }
    }
}

/// Implementation of `CurrentTxFieldGetter` for 256-bit cryptographic hashes.
///
/// This implementation handles 32-byte hash fields in XRPL transactions.
/// Hash256 values are used for transaction IDs, account transaction IDs,
/// references to other transactions, and various cryptographic identifiers.
///
/// # Buffer Management
///
/// Uses a 32-byte buffer (HASH256_SIZE) and validates that exactly 32 bytes
/// are returned from the host function to ensure data integrity.
impl CurrentTxFieldGetter for Hash256 {
    #[inline]
    fn get_from_current_tx(field_code: i32) -> Result<Self> {
        match get_fixed_size_field_with_expected_bytes::<HASH256_SIZE, _>(
            field_code,
            |fc, buf, size| unsafe { get_tx_field(fc, buf, size) },
        ) {
            Result::Ok(buffer) => Result::Ok(buffer.into()),
            Result::Err(e) => Result::Err(e),
        }
    }

    #[inline]
    fn get_from_current_tx_optional(field_code: i32) -> Result<Option<Self>> {
        match get_fixed_size_field_with_expected_bytes_optional::<HASH256_SIZE, _>(
            field_code,
            |fc, buf, size| unsafe { get_tx_field(fc, buf, size) },
        ) {
            Result::Ok(buffer) => Result::Ok(buffer.map(|b| b.into())),
            Result::Err(e) => Result::Err(e),
        }
    }
}

/// Implementation of `CurrentTxFieldGetter` for XRPL public keys.
///
/// This implementation handles 33-byte compressed public key fields in XRPL transactions.
/// Public keys are used for cryptographic signature verification and are commonly found
/// in the SigningPubKey field and various other cryptographic contexts.
///
/// # Buffer Management
///
/// Uses a 33-byte buffer and validates that exactly 33 bytes are returned
/// from the host function. The buffer is converted to a PublicKey using
/// the `From<[u8; 33]>` implementation.
impl CurrentTxFieldGetter for PublicKey {
    #[inline]
    fn get_from_current_tx(field_code: i32) -> Result<Self> {
        match get_fixed_size_field_with_expected_bytes::<33, _>(
            field_code,
            |fc, buf, size| unsafe { get_tx_field(fc, buf, size) },
        ) {
            Result::Ok(buffer) => Result::Ok(buffer.into()),
            Result::Err(e) => Result::Err(e),
        }
    }

    #[inline]
    fn get_from_current_tx_optional(field_code: i32) -> Result<Option<Self>> {
        match get_fixed_size_field_with_expected_bytes_optional::<33, _>(
            field_code,
            |fc, buf, size| unsafe { get_tx_field(fc, buf, size) },
        ) {
            Result::Ok(buffer) => Result::Ok(buffer.map(|b| b.into())),
            Result::Err(e) => Result::Err(e),
        }
    }
}

/// Implementation of `CurrentTxFieldGetter` for variable-length binary data.
///
/// This implementation handles blob fields in XRPL transactions, which can contain
/// arbitrary binary data such as transaction signatures, memos, fulfillment data,
/// and other variable-length content that doesn't fit into fixed-size types.
///
/// # Buffer Management
///
/// Uses a 1024-byte buffer to accommodate most blob field sizes. The actual
/// length of the data is determined by the return value from the host function
/// and stored in the Blob's `len` field. No strict byte count validation is
/// performed since blobs can vary significantly in size.
impl CurrentTxFieldGetter for Blob {
    #[inline]
    fn get_from_current_tx(field_code: i32) -> Result<Self> {
        match get_variable_size_field::<1024, _>(field_code, |fc, buf, size| unsafe {
            get_tx_field(fc, buf, size)
        }) {
            Result::Ok((data, len)) => Result::Ok(Blob { data, len }),
            Result::Err(e) => Result::Err(e),
        }
    }

    #[inline]
    fn get_from_current_tx_optional(field_code: i32) -> Result<Option<Self>> {
        match get_variable_size_field_optional::<1024, _>(field_code, |fc, buf, size| unsafe {
            get_tx_field(fc, buf, size)
        }) {
            Result::Ok(opt) => Result::Ok(opt.map(|(data, len)| Blob { data, len })),
            Result::Err(e) => Result::Err(e),
        }
    }
}

/// Implementation of `CurrentTxFieldGetter` for XRPL TransactionType enums.
///
/// This implementation handles 2byte transaction type fields in XRPL transactions.
///
/// # Buffer Management
///
/// Uses a 2-byte buffer and validates that exactly 2 bytes are returned from the host function.
impl CurrentTxFieldGetter for TransactionType {
    #[inline]
    fn get_from_current_tx(field_code: i32) -> Result<Self> {
        match get_fixed_size_field_with_expected_bytes::<2, _>(field_code, |fc, buf, size| unsafe {
            get_tx_field(fc, buf, size)
        }) {
            Result::Ok(buffer) => Result::Ok(i16::from_le_bytes(buffer).into()),
            Result::Err(e) => Result::Err(e),
        }
    }

    #[inline]
    fn get_from_current_tx_optional(field_code: i32) -> Result<Option<Self>> {
        match get_fixed_size_field_with_expected_bytes_optional::<2, _>(
            field_code,
            |fc, buf, size| unsafe { get_tx_field(fc, buf, size) },
        ) {
            Result::Ok(buffer) => Result::Ok(buffer.map(|b| i16::from_le_bytes(b).into())),
            Result::Err(e) => Result::Err(e),
        }
    }
}

/// Retrieves a field from the current transaction.
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
pub fn get_field<T: CurrentTxFieldGetter>(field_code: i32) -> Result<T> {
    T::get_from_current_tx(field_code)
}

/// Retrieves an optionally present field from the current transaction.
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
pub fn get_field_optional<T: CurrentTxFieldGetter>(field_code: i32) -> Result<Option<T>> {
    T::get_from_current_tx_optional(field_code)
}

pub mod escrow_finish;
pub mod traits;
