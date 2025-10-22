pub mod account_root;
pub mod current_escrow;
pub mod escrow;
pub mod nft;
pub mod traits;

use crate::core::types::account_id::{ACCOUNT_ID_SIZE, AccountID};
use crate::core::types::amount::Amount;
use crate::core::types::blob::Blob;
use crate::core::types::hash_128::{HASH128_SIZE, Hash128};
use crate::core::types::hash_256::{HASH256_SIZE, Hash256};
use crate::host::error_codes::{
    match_result_code, match_result_code_optional, match_result_code_with_expected_bytes,
    match_result_code_with_expected_bytes_optional,
};
use crate::host::{Result, get_current_ledger_obj_field, get_ledger_obj_field};

/// Trait for types that can be retrieved from ledger object fields
pub trait FieldGetter: Sized {
    /// Get a required field from the current ledger object
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self>;

    /// Get an optional field from the current ledger object
    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>>;

    /// Get a required field from a specific ledger object
    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self>;

    /// Get an optional field from a specific ledger object
    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>>;
}

// Implementation for u16
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

// Implementation for u32
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

// Implementation for u64
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

// Implementation for AccountID
impl FieldGetter for AccountID {
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        let mut buffer = [0x00; ACCOUNT_ID_SIZE];
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, buffer.as_mut_ptr(), buffer.len()) };
        match_result_code_with_expected_bytes(result_code, buffer.len(), || buffer.into())
    }

    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        let mut buffer = [0x00; ACCOUNT_ID_SIZE];
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, buffer.as_mut_ptr(), buffer.len()) };
        match_result_code_with_expected_bytes_optional(result_code, buffer.len(), || {
            Some(buffer.into())
        })
    }

    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        let mut buffer = [0x00; ACCOUNT_ID_SIZE];
        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, buffer.as_mut_ptr(), buffer.len())
        };
        match_result_code_with_expected_bytes(result_code, buffer.len(), || buffer.into())
    }

    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        let mut buffer = [0x00; ACCOUNT_ID_SIZE];
        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, buffer.as_mut_ptr(), buffer.len())
        };
        match_result_code_with_expected_bytes_optional(result_code, buffer.len(), || {
            Some(buffer.into())
        })
    }
}

// Implementation for Amount
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

// Implementation for Hash128
impl FieldGetter for Hash128 {
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        let mut buffer = [0u8; HASH128_SIZE];
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, buffer.as_mut_ptr(), buffer.len()) };
        match_result_code_with_expected_bytes(result_code, HASH128_SIZE, || Hash128(buffer))
    }

    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        let mut buffer = [0u8; HASH128_SIZE];
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, buffer.as_mut_ptr(), buffer.len()) };
        match_result_code_with_expected_bytes_optional(result_code, HASH128_SIZE, || {
            Some(Hash128(buffer))
        })
    }

    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        let mut buffer = [0u8; HASH128_SIZE];
        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, buffer.as_mut_ptr(), buffer.len())
        };
        match_result_code_with_expected_bytes(result_code, HASH128_SIZE, || Hash128(buffer))
    }

    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        let mut buffer = [0u8; HASH128_SIZE];
        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, buffer.as_mut_ptr(), buffer.len())
        };
        match_result_code_with_expected_bytes_optional(result_code, HASH128_SIZE, || {
            Some(Hash128(buffer))
        })
    }
}

// Implementation for Hash256
impl FieldGetter for Hash256 {
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        let mut buffer = [0u8; HASH256_SIZE];
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, buffer.as_mut_ptr(), buffer.len()) };
        match_result_code_with_expected_bytes(result_code, HASH256_SIZE, || Hash256(buffer))
    }

    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        let mut buffer = [0u8; HASH256_SIZE];
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, buffer.as_mut_ptr(), buffer.len()) };
        match_result_code_with_expected_bytes_optional(result_code, HASH256_SIZE, || {
            Some(Hash256(buffer))
        })
    }

    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        let mut buffer = [0u8; HASH256_SIZE];
        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, buffer.as_mut_ptr(), buffer.len())
        };
        match_result_code_with_expected_bytes(result_code, HASH256_SIZE, || Hash256(buffer))
    }

    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        let mut buffer = [0u8; HASH256_SIZE];
        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, buffer.as_mut_ptr(), buffer.len())
        };
        match_result_code_with_expected_bytes_optional(result_code, HASH256_SIZE, || {
            Some(Hash256(buffer))
        })
    }
}

// Implementation for Blob
impl FieldGetter for Blob {
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        let mut buffer = [0u8; 1024];
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, buffer.as_mut_ptr(), buffer.len()) };
        match_result_code(result_code, || Blob {
            data: buffer,
            len: result_code as usize,
        })
    }

    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        let mut buffer = [0u8; 1024];
        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, buffer.as_mut_ptr(), buffer.len()) };
        match_result_code_optional(result_code, || {
            Some(Blob {
                data: buffer,
                len: result_code as usize,
            })
        })
    }

    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        let mut buffer = [0u8; 1024];
        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, buffer.as_mut_ptr(), buffer.len())
        };
        match_result_code(result_code, || Blob {
            data: buffer,
            len: result_code as usize,
        })
    }

    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        let mut buffer = [0u8; 1024];
        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, buffer.as_mut_ptr(), buffer.len())
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
    #[inline(always)]
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
