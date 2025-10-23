pub mod account_root;
pub mod current_escrow;
pub mod escrow;
pub mod nft;
pub mod traits;

pub mod current_ledger_object {
    use crate::core::types::account_id::{ACCOUNT_ID_SIZE, AccountID};
    use crate::core::types::amount::{AMOUNT_SIZE, Amount};
    use crate::core::types::blob::Blob;
    use crate::core::types::hash_256::{HASH256_SIZE, Hash256};
    use crate::host::error_codes::{
        match_result_code, match_result_code_with_expected_bytes,
        match_result_code_with_expected_bytes_optional,
    };
    use crate::host::{Result, get_current_ledger_obj_field, to_non_optional};

    /// Retrieves an `AccountID` field from the current ledger object.
    ///
    /// # Arguments
    ///
    /// * `field_code` - The field code identifying which field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<AccountID>` where:
    /// * `Ok(AccountID)` - The account identifier for the specified field
    /// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
    #[inline(always)]
    pub fn get_account_id_field(field_code: i32) -> Result<AccountID> {
        let mut buffer = [0x00; ACCOUNT_ID_SIZE];

        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, buffer.as_mut_ptr(), buffer.len()) };

        match_result_code_with_expected_bytes(result_code, buffer.len(), || buffer.into())
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
    pub fn get_amount_field(field_code: i32) -> Result<Amount> {
        let mut buffer = [0u8; AMOUNT_SIZE]; // Enough to hold an Amount

        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, buffer.as_mut_ptr(), AMOUNT_SIZE) };

        match_result_code(result_code, || Amount::from(buffer))
    }

    /// Retrieves a `u16` field from the current ledger object.
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
    pub fn get_u16_field(field_code: i32) -> Result<u16> {
        to_non_optional(get_u16_field_optional(field_code))
    }

    /// Retrieves an optionally present `u16` field from the current ledger object.
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
    pub fn get_u16_field_optional(field_code: i32) -> Result<Option<u16>> {
        let mut value: u16 = 0; // <-- Initialize 8 bytes (only works due to little endian encoding in WASM and WAMR-host
        let value_ptr: *mut u8 = (&mut value as *mut u16).cast::<u8>();

        let result_code = unsafe { get_current_ledger_obj_field(field_code, value_ptr, 2) };

        match_result_code_with_expected_bytes_optional(result_code, 2, || Some(value))
    }

    /// Retrieves a `u32` field from the current ledger object.
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
    pub fn get_u32_field(field_code: i32) -> Result<u32> {
        to_non_optional(get_u32_field_optional(field_code))
    }

    /// Retrieves an optionally present `u32` field from the current ledger object.
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
    pub fn get_u32_field_optional(field_code: i32) -> Result<Option<u32>> {
        let mut value: u32 = 0; // <-- Initialize 8 bytes (only works due to little endian encoding in WASM and WAMR-host
        let value_ptr: *mut u8 = (&mut value as *mut u32).cast::<u8>();

        let result_code = unsafe { get_current_ledger_obj_field(field_code, value_ptr, 4) };

        match_result_code_with_expected_bytes_optional(result_code, 4, || Some(value))
    }

    /// Retrieves a `u64` field from the current ledger object.
    ///
    /// # Arguments
    ///
    /// * `field_code` - The field code identifying which AccountID field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<Amount>` where:
    /// * `Ok(AccountID)` - The account identifier for the specified field
    /// * `Err(Error)` - If the field cannot be retrieved or has an unexpected size.
    #[inline]
    pub fn get_u64_field(field_code: i32) -> Result<u64> {
        to_non_optional(get_u64_field_optional(field_code))
    }

    /// Retrieves an optionally present `u64` field from the current ledger object.
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
    pub fn get_u64_field_optional(field_code: i32) -> Result<Option<u64>> {
        let mut value: u64 = 0u64; // <-- Initialize 8 bytes (only works due to little endian encoding in WASM and WAMR-host
        let value_ptr: *mut u8 = (&mut value as *mut u64).cast::<u8>();

        let result_code = unsafe { get_current_ledger_obj_field(field_code, value_ptr, 8) };

        match_result_code_with_expected_bytes_optional(result_code, 8, || Some(value))
    }

    /// Retrieves a `Hash256` field from the current ledger object.
    ///
    /// # Arguments
    ///
    /// * `field_code` - The field code identifying which AccountID field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<Amount>` where:
    /// * `Ok(AccountID)` - The account identifier for the specified field
    /// * `Err(Error)` - If the field cannot be retrieved or has an unexpected size.
    #[inline]
    pub fn get_hash_256_field(field_code: i32) -> Result<Hash256> {
        to_non_optional(get_hash_256_field_optional(field_code))
    }

    /// Retrieves an optionally present `Hash256` field from the current ledger object.
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
    pub fn get_hash_256_field_optional(field_code: i32) -> Result<Option<Hash256>> {
        let mut buffer = [0u8; HASH256_SIZE]; // Enough to hold 256 bits (32 bytes)

        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, buffer.as_mut_ptr(), buffer.len()) };

        match_result_code_with_expected_bytes(result_code, HASH256_SIZE, || {
            Some(Hash256(buffer)) // <-- Move the buffer into a Hash256
        })
    }

    /// Retrieves a `Blob` field from the current ledger object.
    ///
    /// # Arguments
    ///
    /// * `field_code` - The field code identifying which AccountID field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<Amount>` where:
    /// * `Ok(AccountID)` - The account identifier for the specified field
    /// * `Err(Error)` - If the field cannot be retrieved or has an unexpected size.
    #[inline]
    pub fn get_blob_field(field_code: i32) -> Result<Blob> {
        to_non_optional(get_blob_field_optional(field_code))
    }

    /// Retrieves an optionally present `Blob` field from the current ledger object.
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
    pub fn get_blob_field_optional(field_code: i32) -> Result<Option<Blob>> {
        let mut buffer = [0u8; 1024]; // Enough to hold the largest field, which is a memo.

        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, buffer.as_mut_ptr(), buffer.len()) };

        match_result_code(result_code, || {
            Some(Blob {
                data: buffer,
                len: result_code as usize,
            })
        })
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::core::ledger_objects::ledger_object::get_amount_field_optional;

        // Note: These tests verify the logic of the wrapper functions.
        // The actual host function calls cannot be tested in unit tests as they require the WASM runtime environment.
        // These tests verify constants, buffer sizes, and function signatures.

        #[test]
        fn test_get_account_id_field_signature() {
            // Verify the function signature compiles correctly
            // This function directly calls the host function, not an optional variant
            let _: fn(i32) -> Result<AccountID> = get_account_id_field;
        }

        #[test]
        fn test_get_amount_field_signature() {
            // Verify the function signature compiles correctly
            // This function directly calls the host function, not an optional variant
            let _: fn(i32) -> Result<Amount> = get_amount_field;
        }

        #[test]
        fn test_get_u16_field_signature() {
            // Verify the function signatures compile correctly
            let _: fn(i32) -> Result<u16> = get_u16_field;
            let _: fn(i32) -> Result<Option<u16>> = get_u16_field_optional;
        }

        #[test]
        fn test_get_u32_field_signature() {
            // Verify the function signatures compile correctly
            let _: fn(i32) -> Result<u32> = get_u32_field;
            let _: fn(i32) -> Result<Option<u32>> = get_u32_field_optional;
        }

        #[test]
        fn test_get_u64_field_signature() {
            // Verify the function signatures compile correctly
            let _: fn(i32) -> Result<u64> = get_u64_field;
            let _: fn(i32) -> Result<Option<u64>> = get_u64_field_optional;
        }

        #[test]
        fn test_get_hash_256_field_signature() {
            // Verify the function signatures compile correctly
            let _: fn(i32) -> Result<Hash256> = get_hash_256_field;
            let _: fn(i32) -> Result<Option<Hash256>> = get_hash_256_field_optional;
        }

        #[test]
        fn test_get_blob_field_signature() {
            // Verify the function signatures compile correctly
            let _: fn(i32) -> Result<Blob> = get_blob_field;
            let _: fn(i32) -> Result<Option<Blob>> = get_blob_field_optional;
        }

        // Tests that actually call the functions with test host bindings
        #[test]
        fn test_get_account_id_field_call() {
            // Call the function - test bindings will return buffer length (20)
            let result = get_account_id_field(1);
            assert!(result.is_ok());
            let account_id = result.unwrap();
            assert_eq!(account_id.0.len(), ACCOUNT_ID_SIZE);
        }

        #[test]
        fn test_get_amount_field_optional_call() {
            // Call the optional variant - test bindings will return buffer length (48)
            let result = get_amount_field_optional(1, 1);
            assert!(result.is_ok());
            assert_eq!(result.unwrap().unwrap(), Amount::XRP { num_drops: 0 });
        }

        #[test]
        fn test_get_amount_field_call() {
            // Call the function - test bindings will return buffer length (48)
            let result = get_amount_field(1);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), Amount::XRP { num_drops: 0 });
        }

        #[test]
        fn test_get_u16_field_optional_call() {
            // Call the optional variant - test bindings will return buffer length (2)
            let result = get_u16_field_optional(1);
            assert!(result.is_ok());
            assert!(result.unwrap().unwrap() == 0u16);
        }

        #[test]
        fn test_get_u16_field_call() {
            // Call the non-optional variant
            let result = get_u16_field(1);
            assert!(result.is_ok());
            assert!(result.unwrap() == 0u16);
        }

        #[test]
        fn test_get_u32_field_optional_call() {
            // Call the optional variant - test bindings will return buffer length (4)
            let result = get_u32_field_optional(1);
            assert!(result.is_ok());
            assert!(result.unwrap().unwrap() == 0u32);
        }

        #[test]
        fn test_get_u32_field_call() {
            // Call the non-optional variant
            let result = get_u32_field(1);
            assert!(result.is_ok());
            assert!(result.unwrap() == 0u32);
        }

        #[test]
        fn test_get_u64_field_optional_call() {
            // Call the optional variant - test bindings will return buffer length (8)
            let result = get_u64_field_optional(1);
            assert!(result.is_ok());
            assert_eq!(result.unwrap().unwrap(), 0u64);
        }

        #[test]
        fn test_get_u64_field_call() {
            // Call the non-optional variant
            let result = get_u64_field(1);
            assert!(result.is_ok());
            assert!(result.unwrap() == 0u64);
        }

        #[test]
        fn test_get_hash_256_field_optional_call() {
            // Call the optional variant - test bindings will return buffer length (32)
            let result = get_hash_256_field_optional(1);
            assert!(result.is_ok());
            assert_eq!(result.unwrap().unwrap(), Hash256([0u8; 32]));
        }

        #[test]
        fn test_get_hash_256_field_call() {
            // Call the non-optional variant
            let result = get_hash_256_field(1);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), Hash256([0u8; 32]));
        }

        #[test]
        fn test_get_blob_field_optional_call() {
            // Call the optional variant - test bindings will return buffer length (1024)
            let result = get_blob_field_optional(1);
            assert!(result.is_ok());
            let blob_opt = result.unwrap();
            assert!(blob_opt.is_some());
            let blob = blob_opt.unwrap();
            assert_eq!(blob.len, 1024);
        }

        #[test]
        fn test_get_blob_field_call() {
            // Call the non-optional variant
            let result = get_blob_field(1);
            assert!(result.is_ok());
            let blob = result.unwrap();
            assert_eq!(blob.len, 1024);
        }
    }
}

pub mod ledger_object {
    use crate::core::types::account_id::{ACCOUNT_ID_SIZE, AccountID};
    use crate::core::types::amount::{AMOUNT_SIZE, Amount};
    use crate::core::types::blob::Blob;
    use crate::core::types::hash_128::{HASH128_SIZE, Hash128};
    use crate::core::types::hash_256::{HASH256_SIZE, Hash256};
    use crate::host::error_codes::{
        match_result_code, match_result_code_optional, match_result_code_with_expected_bytes,
        match_result_code_with_expected_bytes_optional,
    };
    use crate::host::{Result, get_ledger_obj_field, to_non_optional};

    /// Retrieves an AccountID field from the current ledger object.
    ///
    /// # Arguments
    ///
    /// * `field_code` - The field code identifying which AccountID field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<AccountID>` where:
    /// * `Ok(AccountID)` - The account identifier for the specified field
    /// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
    #[inline]
    pub fn get_account_id_field(register_num: i32, field_code: i32) -> Result<AccountID> {
        to_non_optional(get_account_id_field_optional(register_num, field_code))
    }

    pub fn get_account_id_field_optional(
        register_num: i32,
        field_code: i32,
    ) -> Result<Option<AccountID>> {
        let mut buffer = [0x00; ACCOUNT_ID_SIZE];

        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, buffer.as_mut_ptr(), buffer.len())
        };

        match_result_code_with_expected_bytes_optional(result_code, buffer.len(), || {
            let account_id: AccountID = buffer.into();
            Some(account_id)
        })
    }

    /// Retrieves a `Amount` field from a specified ledger object.
    ///
    /// This function retrieves a token amount field from a ledger object stored in a register.
    /// It wraps the optional variant and returns an error if the field is not present.
    ///
    /// # Arguments
    ///
    /// * `register_num` - The register number holding the ledger object to look for data in
    /// * `field_code` - The field code identifying which Amount field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<Amount>` where:
    /// * `Ok(Amount)` - The token amount for the specified field, which can be XRP, IOU, or MPT
    /// * `Err(Error)` - If the field cannot be retrieved, is not present, or has an unexpected format
    #[inline]
    pub fn get_amount_field(register_num: i32, field_code: i32) -> Result<Amount> {
        to_non_optional(get_amount_field_optional(register_num, field_code))
    }

    /// Retrieves an optionally present `Amount` field from a specified ledger object.
    ///
    /// This function attempts to retrieve a token amount field from a ledger object stored in a register.
    /// Unlike the non-optional variant, this function returns `None` if the field is not present,
    /// rather than returning an error.
    ///
    /// # Arguments
    ///
    /// * `register_num` - The register number holding the ledger object to look for data in
    /// * `field_code` - The field code identifying which Amount field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<Amount>>` where:
    /// * `Ok(Some(Amount))` - The token amount for the specified field, which can be XRP, IOU, or MPT
    /// * `Ok(None)` - If the field is not present in the ledger object
    /// * `Err(Error)` - If the field retrieval operation failed or the data has an unexpected format
    #[inline]
    pub fn get_amount_field_optional(register_num: i32, field_code: i32) -> Result<Option<Amount>> {
        let mut buffer = [0u8; AMOUNT_SIZE]; // Enough to hold an Amount

        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, buffer.as_mut_ptr(), AMOUNT_SIZE)
        };

        match_result_code_optional(result_code, || Some(Amount::from(buffer)))
    }

    /// Retrieves a `u16` field from the specified ledger object.
    ///
    /// # Arguments
    ///
    /// * `register_num` - The register number holding the ledger object to look for data in.
    /// * `field_code` - The field code identifying which field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<Amount>` where:
    /// * `Ok(AccountID)` - The account identifier for the specified field
    /// * `Err(Error)` - If the field cannot be retrieved or has an unexpected size.
    #[inline]
    pub fn get_u16_field(register_num: i32, field_code: i32) -> Result<u16> {
        to_non_optional(get_u16_field_optional(register_num, field_code))
    }

    /// Retrieves an optionally present `u16` field from the specified ledger object.
    ///
    /// # Arguments
    ///
    /// * `register_num` - The register number holding the ledger object to look for data in.
    /// * `field_code` - The field code identifying which field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<Amount>` where:
    /// * `Ok(AccountID)` - The account identifier for the specified field
    /// * `Err(Error)` - If the field cannot be retrieved or has an unexpected size.
    #[inline]
    pub fn get_u16_field_optional(register_num: i32, field_code: i32) -> Result<Option<u16>> {
        let mut value: u16 = 0; // <-- Initialize 8 bytes (only works due to little endian encoding in WASM and WAMR-host
        let value_ptr: *mut u8 = (&mut value as *mut u16).cast::<u8>();

        let result_code = unsafe { get_ledger_obj_field(register_num, field_code, value_ptr, 2) };

        match_result_code_with_expected_bytes_optional(result_code, 2, || Some(value))
    }

    /// Retrieves a `u32` field from the specified ledger object.
    ///
    /// # Arguments
    ///
    /// * `register_num` - The register number holding the ledger object to look for data in.
    /// * `field_code` - The field code identifying which AccountID field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<Amount>` where:
    /// * `Ok(AccountID)` - The account identifier for the specified field
    /// * `Err(Error)` - If the field cannot be retrieved or has an unexpected size.
    #[inline]
    pub fn get_u32_field(register_num: i32, field_code: i32) -> Result<u32> {
        to_non_optional(get_u32_field_optional(register_num, field_code))
    }

    /// Retrieves an optionally present `u32` field from the specified ledger object.
    ///
    /// # Arguments
    ///
    /// * `register_num` - The register number holding the ledger object to look for data in.
    /// * `field_code` - The field code identifying which field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<Amount>` where:
    /// * `Ok(AccountID)` - The account identifier for the specified field
    /// * `Err(Error)` - If the field cannot be retrieved or has an unexpected size.
    #[inline]
    pub fn get_u32_field_optional(register_num: i32, field_code: i32) -> Result<Option<u32>> {
        let mut value: u32 = 0; // <-- Initialize 8 bytes (only works due to little endian encoding in WASM and WAMR-host
        let value_ptr: *mut u8 = (&mut value as *mut u32).cast::<u8>();

        let result_code = unsafe { get_ledger_obj_field(register_num, field_code, value_ptr, 4) };

        match_result_code_with_expected_bytes_optional(result_code, 4, || Some(value))
    }

    /// Retrieves a `u64` field from the specified ledger object.
    ///
    /// # Arguments
    ///
    /// * `register_num` - The register number holding the ledger object to look for data in.
    /// * `field_code` - The field code identifying which AccountID field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<Amount>` where:
    /// * `Ok(AccountID)` - The account identifier for the specified field
    /// * `Err(Error)` - If the field cannot be retrieved or has an unexpected size.
    #[inline]
    pub fn get_u64_field(register_num: i32, field_code: i32) -> Result<u64> {
        to_non_optional(get_u64_field_optional(register_num, field_code))
    }

    /// Retrieves an optionally present `u64` field from the specified ledger object.
    ///
    /// # Arguments
    ///
    /// * `register_num` - The register number holding the ledger object to look for data in.
    /// * `field_code` - The field code identifying which field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<Amount>` where:
    /// * `Ok(AccountID)` - The account identifier for the specified field
    /// * `Err(Error)` - If the field cannot be retrieved or has an unexpected size.
    #[inline]
    pub fn get_u64_field_optional(register_num: i32, field_code: i32) -> Result<Option<u64>> {
        let mut value: u64 = 0; // <-- Initialize 8 bytes (only works due to little endian encoding in WASM and WAMR-host
        let value_ptr: *mut u8 = (&mut value as *mut u64).cast::<u8>();

        let result_code = unsafe { get_ledger_obj_field(register_num, field_code, value_ptr, 8) };

        match_result_code_with_expected_bytes_optional(result_code, 8, || Some(value))
    }

    #[inline]
    pub fn get_hash_128_field(register_num: i32, field_code: i32) -> Result<Hash128> {
        to_non_optional(get_hash_128_field_optional(register_num, field_code))
    }

    #[inline]
    pub fn get_hash_128_field_optional(
        register_num: i32,
        field_code: i32,
    ) -> Result<Option<Hash128>> {
        let mut buffer = [0u8; HASH128_SIZE]; // Enough to hold 128 bits (16 bytes)

        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, buffer.as_mut_ptr(), buffer.len())
        };

        match_result_code_with_expected_bytes(result_code, HASH128_SIZE, || {
            Some(Hash128(buffer)) // <-- Move the buffer into a Hash128
        })
    }

    #[inline]
    pub fn get_hash_256_field(register_num: i32, field_code: i32) -> Result<Hash256> {
        to_non_optional(get_hash_256_field_optional(register_num, field_code))
    }

    #[inline]
    pub fn get_hash_256_field_optional(
        register_num: i32,
        field_code: i32,
    ) -> Result<Option<Hash256>> {
        let mut buffer = [0u8; HASH256_SIZE]; // Enough to hold 256 bits (32 bytes)

        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, buffer.as_mut_ptr(), buffer.len())
        };

        match_result_code_with_expected_bytes(result_code, HASH256_SIZE, || {
            Some(Hash256(buffer)) // <-- Move the buffer into an Hash256
        })
    }

    #[inline]
    pub fn get_blob_field(register_num: i32, field_code: i32) -> Result<Blob> {
        to_non_optional(get_blob_field_optional(register_num, field_code))
    }

    #[inline]
    pub fn get_blob_field_optional(register_num: i32, field_code: i32) -> Result<Option<Blob>> {
        let mut buffer = [0u8; 1024]; // Enough to hold the largest field, which is a memo.

        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, buffer.as_mut_ptr(), buffer.len())
        };

        match_result_code(result_code, || {
            Some(Blob {
                data: buffer,
                len: result_code as usize,
            })
        })
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        // Note: These tests verify the logic of the wrapper functions.
        // The actual host function calls cannot be tested in unit tests as they require the WASM runtime environment
        // with cached ledger objects. These tests use mock parameters to verify function signatures and constants.

        #[test]
        fn test_get_account_id_field_signature() {
            // Verify the function signatures compile correctly
            let _: fn(i32, i32) -> Result<AccountID> = get_account_id_field;
            let _: fn(i32, i32) -> Result<Option<AccountID>> = get_account_id_field_optional;
        }

        #[test]
        fn test_get_amount_field_signature() {
            // Verify the function signatures compile correctly
            let _: fn(i32, i32) -> Result<Amount> = get_amount_field;
            let _: fn(i32, i32) -> Result<Option<Amount>> = get_amount_field_optional;
        }

        #[test]
        fn test_get_u16_field_signature() {
            // Verify the function signatures compile correctly
            let _: fn(i32, i32) -> Result<u16> = get_u16_field;
            let _: fn(i32, i32) -> Result<Option<u16>> = get_u16_field_optional;
        }

        #[test]
        fn test_get_u32_field_signature() {
            // Verify the function signatures compile correctly
            let _: fn(i32, i32) -> Result<u32> = get_u32_field;
            let _: fn(i32, i32) -> Result<Option<u32>> = get_u32_field_optional;
        }

        #[test]
        fn test_get_u64_field_signature() {
            // Verify the function signatures compile correctly
            let _: fn(i32, i32) -> Result<u64> = get_u64_field;
            let _: fn(i32, i32) -> Result<Option<u64>> = get_u64_field_optional;
        }

        #[test]
        fn test_get_uint_128_field_signature() {
            // Verify the function signatures compile correctly
            let _: fn(i32, i32) -> Result<Hash128> = get_hash_128_field;
            let _: fn(i32, i32) -> Result<Option<Hash128>> = get_hash_128_field_optional;
        }

        #[test]
        fn test_get_hash_256_field_signature() {
            // Verify the function signatures compile correctly
            let _: fn(i32, i32) -> Result<Hash256> = get_hash_256_field;
            let _: fn(i32, i32) -> Result<Option<Hash256>> = get_hash_256_field_optional;
        }

        #[test]
        fn test_get_blob_field_signature() {
            // Verify the function signatures compile correctly
            let _: fn(i32, i32) -> Result<Blob> = get_blob_field;
            let _: fn(i32, i32) -> Result<Option<Blob>> = get_blob_field_optional;
        }

        // Tests that actually call the functions with test host bindings
        #[test]
        fn test_get_account_id_field_optional_call() {
            // Call the optional variant - test bindings will return buffer length (20)
            let result = get_account_id_field_optional(1, 1);
            assert!(result.is_ok());
            assert_eq!(result.unwrap().unwrap().0.len(), ACCOUNT_ID_SIZE);
        }

        #[test]
        fn test_get_account_id_field_call() {
            // Call the non-optional variant
            let result = get_account_id_field(1, 1);
            assert!(result.is_ok());
            assert_eq!(result.unwrap().0.len(), ACCOUNT_ID_SIZE);
        }

        #[test]
        fn test_get_amount_field_optional_call() {
            // Call the optional variant - test bindings will return buffer length (48)
            let result = get_amount_field_optional(1, 1);
            assert!(result.is_ok());
            assert_eq!(result.unwrap().unwrap(), Amount::XRP { num_drops: 0 });
        }

        #[test]
        fn test_get_amount_field_call() {
            // Call the non-optional variant
            let result = get_amount_field(1, 1);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), Amount::XRP { num_drops: 0 });
        }

        #[test]
        fn test_get_u16_field_optional_call() {
            // Call the optional variant - test bindings will return buffer length (2)
            let result = get_u16_field_optional(1, 1);
            assert!(result.is_ok());
            assert_eq!(result.unwrap().unwrap(), 0u16);
        }

        #[test]
        fn test_get_u16_field_call() {
            // Call the non-optional variant
            let result = get_u16_field(1, 1);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 0u16);
        }

        #[test]
        fn test_get_u32_field_optional_call() {
            // Call the optional variant - test bindings will return buffer length (4)
            let result = get_u32_field_optional(1, 1);
            assert!(result.is_ok());
            assert_eq!(result.unwrap().unwrap(), 0u32);
        }

        #[test]
        fn test_get_u32_field_call() {
            // Call the non-optional variant
            let result = get_u32_field(1, 1);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 0u32);
        }

        #[test]
        fn test_get_u64_field_optional_call() {
            // Call the optional variant - test bindings will return buffer length (8)
            let result = get_u64_field_optional(1, 1);
            assert!(result.is_ok());
            assert_eq!(result.unwrap().unwrap(), 0u64);
        }

        #[test]
        fn test_get_u64_field_call() {
            // Call the non-optional variant
            let result = get_u64_field(1, 1);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 0u64);
        }

        #[test]
        fn test_get_uint_128_field_optional_call() {
            // Call the optional variant - test bindings will return buffer length (16)
            let result = get_hash_128_field_optional(1, 1);
            assert!(result.is_ok());
            assert_eq!(result.unwrap().unwrap(), Hash128([0u8; 16]));
        }

        #[test]
        fn test_get_uint_128_field_call() {
            // Call the non-optional variant
            let result = get_hash_128_field(1, 1);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), Hash128([0u8; 16]));
        }

        #[test]
        fn test_get_hash_256_field_optional_call() {
            // Call the optional variant - test bindings will return buffer length (32)
            let result = get_hash_256_field_optional(1, 1);
            assert!(result.is_ok());
            assert_eq!(result.unwrap().unwrap(), Hash256([0u8; 32]));
        }

        #[test]
        fn test_get_hash_256_field_call() {
            // Call the non-optional variant
            let result = get_hash_256_field(1, 1);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), Hash256([0u8; 32]));
        }

        #[test]
        fn test_get_blob_field_optional_call() {
            // Call the optional variant - test bindings will return buffer length (1024)
            let result = get_blob_field_optional(1, 1);
            assert!(result.is_ok());
            let blob_opt = result.unwrap();
            assert!(blob_opt.is_some());
            let blob = blob_opt.unwrap();
            assert_eq!(blob.len, 1024);
        }

        #[test]
        fn test_get_blob_field_call() {
            // Call the non-optional variant
            let result = get_blob_field(1, 1);
            assert!(result.is_ok());
            let blob = result.unwrap();
            assert_eq!(blob.len, 1024);
        }
    }
}
