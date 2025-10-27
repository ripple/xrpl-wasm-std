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
//! use xrpl_wasm_std::core::current_tx::escrow_finish::EscrowFinish;
//! use xrpl_wasm_std::core::current_tx::traits::TransactionCommonFields;
//! let tx = EscrowFinish;
//! let account = tx.get_account().unwrap_or_panic();
//! let _flags = tx.get_flags().unwrap_or_panic();
//! ```

use crate::core::types::account_id::{ACCOUNT_ID_SIZE, AccountID};
use crate::core::types::amount::{AMOUNT_SIZE, Amount};
use crate::core::types::blob::Blob;
use crate::core::types::public_key::PublicKey;
use crate::core::types::uint::{HASH256_SIZE, Hash256};
use crate::host::error_codes::{
    match_result_code, match_result_code_with_expected_bytes,
    match_result_code_with_expected_bytes_optional,
};
use crate::host::{Result, get_tx_field, to_non_optional};

pub mod escrow_finish;
pub mod traits;

/// Retrieves an AccountID field from the current transaction.
///
/// This function extracts a 20-byte account identifier from the current XRPL transaction.
/// Account IDs are used to identify XRPL accounts in various transaction fields such as
/// the transaction sender (`Account`), destination (`Destination`), or other account references.
///
/// # Arguments
///
/// * `field_code` - The field code identifying which AccountID field to retrieve. This corresponds
///   to the XRPL transaction field identifier.
///
/// # Returns
///
/// Returns a `Result<AccountID>` where:
/// * `Ok(AccountID)` - The 20-byte account identifier for the specified field
/// * `Err(Error)` - If the field cannot be retrieved, is missing (for required fields), or has an
///   unexpected size
///
/// # Errors
///
/// This function returns an error if:
/// - The specified field is not present in the transaction
/// - The field data is not exactly 20 bytes (ACCOUNT_ID_SIZE)
/// - The underlying host function call fails
#[inline]
fn get_account_id_field(field_code: i32) -> Result<AccountID> {
    let mut buffer = [0x00; ACCOUNT_ID_SIZE];

    let result_code = unsafe { get_tx_field(field_code, buffer.as_mut_ptr(), buffer.len()) };

    match_result_code_with_expected_bytes(result_code, ACCOUNT_ID_SIZE, || buffer.into())
}

/// Retrieves a `Amount` field from the current ledger object.
///
/// # Arguments
///
/// * `field_code` - The field code identifying which field to retrieve
///
/// # Returns
///
/// Returns a `Result<Amount>` where:
/// * `Ok(AccountID)` - The account identifier for the specified field
/// * `Err(Error)` - If the field cannot be retrieved or has an unexpected size.
#[inline]
fn get_amount_field(field_code: i32) -> Result<Amount> {
    let mut buffer = [0u8; AMOUNT_SIZE]; // Enough to hold an Amount

    let result_code = unsafe { get_tx_field(field_code, buffer.as_mut_ptr(), buffer.len()) };

    match_result_code(result_code, || Amount::from(buffer))
}

/// Retrieves a `u32` field from the current transaction.
///
/// This function extracts a 32-bit unsigned integer from the current XRPL transaction.
/// u32 fields are commonly used for sequence numbers, flags, timestamps, and other
/// numeric values in XRPL transactions.
///
/// # Arguments
///
/// * `field_code` - The field code identifying which u32 field to retrieve. This corresponds to
///   the XRPL transaction field identifier.
///
/// # Returns
///
/// Returns a `Result<u32>` where:
/// * `Ok(u32)` - The 32-bit unsigned integer value for the specified field
/// * `Err(Error)` - If the field cannot be retrieved, is missing, or has an unexpected size
///
/// # Errors
///
/// This function returns an error if:
/// - The specified field is not present in the transaction
/// - The field data is not exactly 4 bytes
/// - The underlying host function call fails
///
/// # See Also
///
/// * [`get_u32_field_optional`] - For optional u32 fields that may not be present
#[inline]
fn get_u32_field(field_code: i32) -> Result<u32> {
    to_non_optional(get_u32_field_optional(field_code))
}

/// Retrieves an optional `u32` field from the current transaction.
///
/// This function extracts a 32-bit unsigned integer from the current XRPL transaction,
/// returning `None` if the field is not present. This is useful for optional transaction
/// fields that may or may not be included in the transaction data.
///
/// # Arguments
///
/// * `field_code` - The field code identifying which u32 field to retrieve. This corresponds to
///   the XRPL transaction field identifier.
///
/// # Returns
///
/// Returns a `Result<Option<u32>>` where:
/// * `Ok(Some(u32))` - The 32-bit unsigned integer value if the field is present
/// * `Ok(None)` - If the field is not present in the transaction (this is not an error)
/// * `Err(Error)` - If an error occurred during field retrieval or the field has unexpected size
///
/// # Errors
///
/// This function returns an error if:
/// - The field data is present but not exactly 4 bytes
/// - The underlying host function call fails for reasons other than missing field
///
/// # See Also
///
/// * [`get_u32_field`] - For required u32 fields that must be present
#[inline]
fn get_u32_field_optional(field_code: i32) -> Result<Option<u32>> {
    let mut buffer = [0u8; 4]; // Enough to hold an u32

    let result_code = unsafe { get_tx_field(field_code, buffer.as_mut_ptr(), buffer.len()) };

    match_result_code_with_expected_bytes_optional(result_code, 4, || {
        Some(u32::from_le_bytes(buffer)) // <-- Move the buffer into a u32
    })
}

/// Retrieves a `Hash256` field from the current transaction.
///
/// This function extracts a 256-bit cryptographic hash from the current XRPL transaction.
/// Hash256 fields are used for transaction hashes, previous transaction references,
/// ledger hashes, and other cryptographic identifiers in XRPL.
///
/// # Arguments
///
/// * `field_code` - The field code identifying which Hash256 field to retrieve. This corresponds
///   to the XRPL transaction field identifier.
///
/// # Returns
///
/// Returns a `Result<Hash256>` where:
/// * `Ok(Hash256)` - The 256-bit hash for the specified field
/// * `Err(Error)` - If the field cannot be retrieved, is missing, or has an unexpected size
///
/// # Errors
///
/// This function returns an error if:
/// - The specified field is not present in the transaction
/// - The field data is not exactly 32 bytes (HASH256_SIZE)
/// - The underlying host function call fails
///
/// # See Also
///
/// * [`get_hash_256_field_optional`] - For optional Hash256 fields that may not be present
#[inline]
fn get_hash_256_field(field_code: i32) -> Result<Hash256> {
    to_non_optional(get_hash_256_field_optional(field_code))
}

/// Retrieves an optional `Hash256` field from the current transaction.
///
/// This function extracts a 256-bit cryptographic hash from the current XRPL transaction,
/// returning `None` if the field is not present. This is useful for optional hash fields
/// that may or may not be included in the transaction data.
///
/// # Arguments
///
/// * `field_code` - The field code identifying which Hash256 field to retrieve. This corresponds
///   to the XRPL transaction field identifier.
///
/// # Returns
///
/// Returns a `Result<Option<Hash256>>` where:
/// * `Ok(Some(Hash256))` - The 256-bit hash if the field is present
/// * `Ok(None)` - If the field is not present in the transaction (this is not an error)
/// * `Err(Error)` - If an error occurred during field retrieval or the field has unexpected size
///
/// # Errors
///
/// This function returns an error if:
/// - The field data is present but not exactly 32 bytes (HASH256_SIZE)
/// - The underlying host function call fails for reasons other than missing field
///
/// # See Also
///
/// * [`get_hash_256_field`] - For required Hash256 fields that must be present
#[inline]
fn get_hash_256_field_optional(field_code: i32) -> Result<Option<Hash256>> {
    let mut buffer = [0u8; HASH256_SIZE]; // Enough to hold 256 bits (32 bytes)

    let result_code = unsafe { get_tx_field(field_code, buffer.as_mut_ptr(), buffer.len()) };

    match_result_code_with_expected_bytes(result_code, HASH256_SIZE, || {
        Some(Hash256::from(buffer)) // <-- Move the buffer into an Hash256
    })
}

/// Retrieves a `PublicKey` field from the current transaction.
///
/// This function extracts a 33-byte compressed public key from the current XRPL transaction.
/// Public key fields are used for cryptographic operations, signature verification,
/// and account authentication in XRPL transactions.
///
/// # Arguments
///
/// * `field_code` - The field code identifying which PublicKey field to retrieve. This corresponds
///   to the XRPL transaction field identifier.
///
/// # Returns
///
/// Returns a `Result<PublicKey>` where:
/// * `Ok(PublicKey)` - The 33-byte compressed public key for the specified field
/// * `Err(Error)` - If the field cannot be retrieved, is missing, or has an unexpected size
///
/// # Errors
///
/// This function returns an error if:
/// - The specified field is not present in the transaction
/// - The field data is not exactly 33 bytes (compressed public key size)
/// - The underlying host function call fails
///
/// # See Also
///
/// * [`get_optional_public_key_field`] - For optional PublicKey fields that may not be present
#[inline]
fn get_public_key_field(field_code: i32) -> Result<PublicKey> {
    to_non_optional(get_optional_public_key_field(field_code))
}

/// Retrieves an optional `PublicKey` field from the current transaction.
///
/// This function extracts a 33-byte compressed public key from the current XRPL transaction,
/// returning `None` if the field is not present. This is useful for optional public key fields
/// that may or may not be included in the transaction data.
///
/// # Arguments
///
/// * `field_code` - The field code identifying which PublicKey field to retrieve. This corresponds
///   to the XRPL transaction field identifier.
///
/// # Returns
///
/// Returns a `Result<Option<PublicKey>>` where:
/// * `Ok(Some(PublicKey))` - The 33-byte compressed public key if the field is present
/// * `Ok(None)` - If the field is not present in the transaction (this is not an error)
/// * `Err(Error)` - If an error occurred during field retrieval or the field has unexpected size
///
/// # Errors
///
/// This function returns an error if:
/// - The field data is present but not exactly 33 bytes
/// - The underlying host function call fails for reasons other than missing field
///
/// # See Also
///
/// * [`get_public_key_field`] - For required PublicKey fields that must be present
#[inline]
fn get_optional_public_key_field(field_code: i32) -> Result<Option<PublicKey>> {
    let mut buffer = [0u8; 33];

    let result_code = unsafe { get_tx_field(field_code, buffer.as_mut_ptr(), buffer.len()) };

    match_result_code_with_expected_bytes_optional(result_code, 33, || Some(buffer.into()))
}

/// Retrieves a variable-length `Blob` field from the current transaction.
///
/// This function extracts variable-length binary data from the current XRPL transaction.
/// Blob fields are used for memos, arbitrary data, encoded objects, and other variable-length
/// information that doesn't fit into fixed-size field types.
///
/// # Arguments
///
/// * `field_code` - The field code identifying which blob field to retrieve. This corresponds to
///   the XRPL transaction field identifier.
///
/// # Returns
///
/// Returns a `Result<Blob>` where:
/// * `Ok(Blob)` - The blob data with its actual length encoded in the structure
/// * `Err(Error)` - If the field cannot be retrieved or is missing
///
/// # Buffer Management
///
/// This function uses a 1024-byte buffer to accommodate the largest possible field,
/// which is typically a memo field. The actual length of the retrieved data is
/// encoded in the returned `Blob` structure, allowing for efficient handling of
/// variable-length data without unnecessary allocations.
///
/// # Errors
///
/// This function returns an error if:
/// - The specified field is not present in the transaction
/// - The underlying host function call fails
/// - The field data exceeds 1024 bytes (buffer overflow protection)
///
/// # See Also
///
/// * [`get_blob_field_optional`] - For optional blob fields that may not be present
#[inline]
fn get_blob_field(field_code: i32) -> Result<Blob> {
    to_non_optional(get_blob_field_optional(field_code))
}

/// Retrieves an optional variable-length `Blob` field from the current transaction.
///
/// This function extracts variable-length binary data from the current XRPL transaction,
/// returning `None` if the field is not present. This is useful for optional blob fields
/// such as memos or custom data that may or may not be included in the transaction.
///
/// # Arguments
///
/// * `field_code` - The field code identifying which blob field to retrieve. This corresponds to
///   the XRPL transaction field identifier.
///
/// # Returns
///
/// Returns a `Result<Option<Blob>>` where:
/// * `Ok(Some(Blob))` - The blob data with its actual length if the field is present
/// * `Ok(None)` - If the field is not present in the transaction (this is not an error)
/// * `Err(Error)` - If an error occurred during field retrieval
///
/// # Buffer Management
///
/// This function uses a 1024-byte buffer to accommodate the largest possible field,
/// which is typically a memo field. The actual length of the retrieved data is
/// encoded in the returned `Blob` structure. Only the bytes up to the actual length
/// contain valid data; the rest of the buffer should be ignored.
///
/// # Errors
///
/// This function will return an error if:
/// - The underlying host function call fails for reasons other than missing field
/// - The field data would exceed 1024 bytes (buffer overflow protection)
///
/// # Performance Notes
///
/// - The 1024-byte buffer is allocated on the stack for performance
/// - Only the actual data length (stored in `result_code`) is meaningful
/// - The buffer size is chosen to handle the largest expected XRPL field
///
/// # See Also
///
/// * [`get_blob_field`] - For required blob fields that must be present
/// * [`Blob`] - The structure used to represent variable-length binary data
#[inline]
fn get_blob_field_optional(field_code: i32) -> Result<Option<Blob>> {
    let mut buffer = [0u8; 1024]; // Enough to hold the largest field, which is a memo.

    let result_code = unsafe { get_tx_field(field_code, buffer.as_mut_ptr(), buffer.len()) };

    match_result_code(result_code, || {
        Some(Blob {
            data: buffer,
            len: result_code as usize,
        })
    })
}
