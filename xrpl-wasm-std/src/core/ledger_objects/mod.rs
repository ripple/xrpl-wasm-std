pub mod account_root;
pub mod current_escrow;
pub mod escrow;
pub mod nft;
pub mod traits;

pub mod current_ledger_object {
    use crate::core::types::account_id::{ACCOUNT_ID_SIZE, AccountID};
    use crate::core::types::amount::token_amount::TokenAmount;
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

    /// Retrieves a `TokenAmount` field from the current ledger object.
    ///
    /// # Arguments
    ///
    /// * `field_code` - The field code identifying which field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<TokenAmount>` where:
    /// * `Ok(AccountID)` - The account identifier for the specified field
    /// * `Err(Error)` - If the field cannot be retrieved or has an unexpected size.
    #[inline]
    pub fn get_amount_field(field_code: i32) -> Result<TokenAmount> {
        const BUFFER_SIZE: usize = 48usize;

        let mut buffer = [0u8; BUFFER_SIZE]; // Enough to hold an Amount

        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, buffer.as_mut_ptr(), BUFFER_SIZE) };

        match_result_code(result_code, || TokenAmount::from(buffer))
    }

    /// Retrieves a `u16` field from the current ledger object.
    ///
    /// # Arguments
    ///
    /// * `field_code` - The field code identifying which field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<TokenAmount>` where:
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
    /// Returns a `Result<TokenAmount>` where:
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
    /// Returns a `Result<TokenAmount>` where:
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
    /// Returns a `Result<TokenAmount>` where:
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
    /// Returns a `Result<TokenAmount>` where:
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
    /// Returns a `Result<TokenAmount>` where:
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
    /// Returns a `Result<TokenAmount>` where:
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
    /// Returns a `Result<TokenAmount>` where:
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
    /// Returns a `Result<TokenAmount>` where:
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
    /// Returns a `Result<TokenAmount>` where:
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
}

pub mod ledger_object {
    use crate::core::types::account_id::{ACCOUNT_ID_SIZE, AccountID};
    use crate::core::types::amount::token_amount::TokenAmount;
    use crate::core::types::blob::Blob;
    use crate::core::types::hash_256::{HASH256_SIZE, Hash256};
    use crate::core::types::uint_128::{UINT128_SIZE, UInt128};
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

    /// Retrieves a `TokenAmount` field from a specified ledger object.
    ///
    /// This function retrieves a token amount field from a ledger object stored in a register.
    /// It wraps the optional variant and returns an error if the field is not present.
    ///
    /// # Arguments
    ///
    /// * `register_num` - The register number holding the ledger object to look for data in
    /// * `field_code` - The field code identifying which TokenAmount field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<TokenAmount>` where:
    /// * `Ok(TokenAmount)` - The token amount for the specified field, which can be XRP, IOU, or MPT
    /// * `Err(Error)` - If the field cannot be retrieved, is not present, or has an unexpected format
    #[inline]
    pub fn get_amount_field(register_num: i32, field_code: i32) -> Result<TokenAmount> {
        to_non_optional(get_amount_field_optional(register_num, field_code))
    }

    /// Retrieves an optionally present `TokenAmount` field from a specified ledger object.
    ///
    /// This function attempts to retrieve a token amount field from a ledger object stored in a register.
    /// Unlike the non-optional variant, this function returns `None` if the field is not present,
    /// rather than returning an error.
    ///
    /// # Arguments
    ///
    /// * `register_num` - The register number holding the ledger object to look for data in
    /// * `field_code` - The field code identifying which TokenAmount field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<TokenAmount>>` where:
    /// * `Ok(Some(TokenAmount))` - The token amount for the specified field, which can be XRP, IOU, or MPT
    /// * `Ok(None)` - If the field is not present in the ledger object
    /// * `Err(Error)` - If the field retrieval operation failed or the data has an unexpected format
    #[inline]
    pub fn get_amount_field_optional(
        register_num: i32,
        field_code: i32,
    ) -> Result<Option<TokenAmount>> {
        const BUFFER_SIZE: usize = 48usize;

        let mut buffer = [0u8; BUFFER_SIZE]; // Enough to hold an Amount

        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, buffer.as_mut_ptr(), BUFFER_SIZE)
        };

        match_result_code_optional(result_code, || Some(TokenAmount::from(buffer)))
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
    /// Returns a `Result<TokenAmount>` where:
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
    /// Returns a `Result<TokenAmount>` where:
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
    /// Returns a `Result<TokenAmount>` where:
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
    /// Returns a `Result<TokenAmount>` where:
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
    /// Returns a `Result<TokenAmount>` where:
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
    /// Returns a `Result<TokenAmount>` where:
    /// * `Ok(AccountID)` - The account identifier for the specified field
    /// * `Err(Error)` - If the field cannot be retrieved or has an unexpected size.
    #[inline]
    pub fn get_u64_field_optional(register_num: i32, field_code: i32) -> Result<Option<u64>> {
        let mut value: u64 = 0; // <-- Initialize 8 bytes (only works due to little endian encoding in WASM and WAMR-host
        let value_ptr: *mut u8 = (&mut value as *mut u64).cast::<u8>();

        let result_code = unsafe { get_ledger_obj_field(register_num, field_code, value_ptr, 8) };

        match_result_code_with_expected_bytes_optional(result_code, 4, || Some(value))
    }

    #[inline]
    pub fn get_uint_128_field(register_num: i32, field_code: i32) -> Result<UInt128> {
        to_non_optional(get_uint_128_field_optional(register_num, field_code))
    }

    #[inline]
    pub fn get_uint_128_field_optional(
        register_num: i32,
        field_code: i32,
    ) -> Result<Option<UInt128>> {
        let mut buffer = [0u8; UINT128_SIZE]; // Enough to hold 128 bits (16 bytes)

        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, buffer.as_mut_ptr(), buffer.len())
        };

        match_result_code_with_expected_bytes(result_code, UINT128_SIZE, || {
            Some(UInt128(buffer)) // <-- Move the buffer into a UInt128
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
}
