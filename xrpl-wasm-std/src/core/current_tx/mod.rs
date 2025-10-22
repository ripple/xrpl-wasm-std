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
use crate::core::types::hash_256::{HASH256_SIZE, Hash256};
use crate::core::types::public_key::PublicKey;
use crate::host::error_codes::{
    match_result_code, match_result_code_with_expected_bytes,
    match_result_code_with_expected_bytes_optional,
};
use crate::host::{Result, get_tx_field};

/// Trait for types that can be retrieved from current transaction fields
pub trait CurrentTxFieldGetter: Sized {
    /// Get a required field from the current transaction
    fn get_from_current_tx(field_code: i32) -> Result<Self>;

    /// Get an optional field from the current transaction
    fn get_from_current_tx_optional(field_code: i32) -> Result<Option<Self>>;
}

// Implementation for u32
impl CurrentTxFieldGetter for u32 {
    fn get_from_current_tx(field_code: i32) -> Result<Self> {
        let mut buffer = [0u8; 4];
        let result_code = unsafe { get_tx_field(field_code, buffer.as_mut_ptr(), buffer.len()) };
        match_result_code_with_expected_bytes(result_code, 4, || u32::from_le_bytes(buffer))
    }

    fn get_from_current_tx_optional(field_code: i32) -> Result<Option<Self>> {
        let mut buffer = [0u8; 4];
        let result_code = unsafe { get_tx_field(field_code, buffer.as_mut_ptr(), buffer.len()) };
        match_result_code_with_expected_bytes_optional(result_code, 4, || {
            Some(u32::from_le_bytes(buffer))
        })
    }
}

// Implementation for AccountID
impl CurrentTxFieldGetter for AccountID {
    fn get_from_current_tx(field_code: i32) -> Result<Self> {
        let mut buffer = [0x00; ACCOUNT_ID_SIZE];
        let result_code = unsafe { get_tx_field(field_code, buffer.as_mut_ptr(), buffer.len()) };
        match_result_code_with_expected_bytes(result_code, ACCOUNT_ID_SIZE, || buffer.into())
    }

    fn get_from_current_tx_optional(field_code: i32) -> Result<Option<Self>> {
        let mut buffer = [0x00; ACCOUNT_ID_SIZE];
        let result_code = unsafe { get_tx_field(field_code, buffer.as_mut_ptr(), buffer.len()) };
        match_result_code_with_expected_bytes_optional(result_code, ACCOUNT_ID_SIZE, || {
            Some(buffer.into())
        })
    }
}

// Implementation for Amount
impl CurrentTxFieldGetter for Amount {
    fn get_from_current_tx(field_code: i32) -> Result<Self> {
        let mut buffer = [0u8; AMOUNT_SIZE];
        let result_code = unsafe { get_tx_field(field_code, buffer.as_mut_ptr(), buffer.len()) };
        match_result_code(result_code, || Amount::from(buffer))
    }

    fn get_from_current_tx_optional(field_code: i32) -> Result<Option<Self>> {
        let mut buffer = [0u8; AMOUNT_SIZE];
        let result_code = unsafe { get_tx_field(field_code, buffer.as_mut_ptr(), buffer.len()) };
        match_result_code(result_code, || Some(Amount::from(buffer)))
    }
}

// Implementation for Hash256
impl CurrentTxFieldGetter for Hash256 {
    fn get_from_current_tx(field_code: i32) -> Result<Self> {
        let mut buffer = [0u8; HASH256_SIZE];
        let result_code = unsafe { get_tx_field(field_code, buffer.as_mut_ptr(), buffer.len()) };
        match_result_code_with_expected_bytes(result_code, HASH256_SIZE, || Hash256(buffer))
    }

    fn get_from_current_tx_optional(field_code: i32) -> Result<Option<Self>> {
        let mut buffer = [0u8; HASH256_SIZE];
        let result_code = unsafe { get_tx_field(field_code, buffer.as_mut_ptr(), buffer.len()) };
        match_result_code_with_expected_bytes_optional(result_code, HASH256_SIZE, || {
            Some(Hash256(buffer))
        })
    }
}

// Implementation for PublicKey
impl CurrentTxFieldGetter for PublicKey {
    fn get_from_current_tx(field_code: i32) -> Result<Self> {
        let mut buffer = [0u8; 33];
        let result_code = unsafe { get_tx_field(field_code, buffer.as_mut_ptr(), buffer.len()) };
        match_result_code_with_expected_bytes(result_code, 33, || buffer.into())
    }

    fn get_from_current_tx_optional(field_code: i32) -> Result<Option<Self>> {
        let mut buffer = [0u8; 33];
        let result_code = unsafe { get_tx_field(field_code, buffer.as_mut_ptr(), buffer.len()) };
        match_result_code_with_expected_bytes_optional(result_code, 33, || Some(buffer.into()))
    }
}

// Implementation for Blob
impl CurrentTxFieldGetter for Blob {
    fn get_from_current_tx(field_code: i32) -> Result<Self> {
        let mut buffer = [0u8; 1024];
        let result_code = unsafe { get_tx_field(field_code, buffer.as_mut_ptr(), buffer.len()) };
        match_result_code(result_code, || Blob {
            data: buffer,
            len: result_code as usize,
        })
    }

    fn get_from_current_tx_optional(field_code: i32) -> Result<Option<Self>> {
        let mut buffer = [0u8; 1024];
        let result_code = unsafe { get_tx_field(field_code, buffer.as_mut_ptr(), buffer.len()) };
        match_result_code(result_code, || {
            Some(Blob {
                data: buffer,
                len: result_code as usize,
            })
        })
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
#[inline(always)]
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
