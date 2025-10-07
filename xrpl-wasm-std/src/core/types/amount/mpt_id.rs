use crate::core::types::account_id::AccountID;

pub const MPT_ID_SIZE: usize = 24;
pub const MPT_SEQUENCE_NUM_SIZE: usize = 4;

/// Holds an MPT Identifier, which consists of a 4-byte sequence number and a 20-byte account id.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct MptId([u8; MPT_ID_SIZE]);

impl MptId {
    /// Creates a new MptId from a sequence number and an issuer account ID.
    pub fn new(sequence_num: u32, issuer: AccountID) -> Self {
        let mut bytes = [0u8; MPT_ID_SIZE];

        // Set the sequence number (first 4 bytes)
        bytes[0..4].copy_from_slice(&sequence_num.to_be_bytes());

        // Set the issuer account ID (last 20 bytes)
        bytes[4..MPT_ID_SIZE].copy_from_slice(&issuer.0);

        MptId(bytes)
    }

    /// Gets the sequence number part of the MptId.
    pub fn get_sequence_num(&self) -> u32 {
        // Transform the first 4 bytes of self.0 into a u32.
        u32::from_be_bytes([self.0[0], self.0[1], self.0[2], self.0[3]])
    }

    /// Gets the issuer account ID part of the MptId.
    pub fn get_issuer(&self) -> AccountID {
        // Transform the last 20 bytes of self.0 into an AccountID.
        let mut account_bytes = [0u8; 20]; // AccountID is 20 bytes
        account_bytes.copy_from_slice(&self.0[4..24]); // Extract bytes 4-23 (20 bytes total)
        AccountID::from(account_bytes)
    }

    /// Gets the raw bytes of the MptId.
    pub fn as_bytes(&self) -> &[u8; 24] {
        &self.0
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.iter().all(|&byte| byte == 0)
    }
}

impl From<[u8; 24]> for MptId {
    fn from(value: [u8; 24]) -> Self {
        MptId(value)
    }
}

impl From<(u32, AccountID)> for MptId {
    fn from(value: (u32, AccountID)) -> Self {
        MptId::new(value.0, value.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mpt_id_creation() {
        // Create a test account ID
        let account_bytes = [1u8; 20];
        let account_id = AccountID::from(account_bytes);

        // Create an MptId using the constructor
        let sequence_num = 12345u32;
        let mpt_id = MptId::new(sequence_num, account_id);

        // Verify the sequence number and issuer
        assert_eq!(mpt_id.get_sequence_num(), sequence_num);
        assert_eq!(mpt_id.get_issuer(), account_id);
    }

    #[test]
    fn test_mpt_id_from_bytes() {
        // Create a test byte array
        let mut bytes = [0u8; 24];
        // Set sequence number bytes (first 4 bytes)
        bytes[0..4].copy_from_slice(&67890u32.to_be_bytes());
        // Set account ID bytes (last 20 bytes)
        // 1. **`bytes.iter_mut().skip(4)`** - Creates a mutable iterator over the bytes array and skips the first 4 elements
        // 2. **`.take(20)`** - Takes only the next 20 elements (since we want indices 4 through 23, which is 20 elements)
        // 3. **`*byte = 2`** - Dereferences the mutable reference to set the value to the integer 2 (for testing purposes)
        for byte in bytes.iter_mut().skip(4).take(20) {
            *byte = 2;
        }

        // Create an MptId from bytes
        let mpt_id = MptId::from(bytes);

        // Verify the sequence number and issuer
        assert_eq!(mpt_id.get_sequence_num(), 67890u32);
        assert_eq!(mpt_id.get_issuer(), AccountID::from([2u8; 20]));
    }

    #[test]
    fn test_mpt_id_from_tuple() {
        // Create a test account ID
        let account_bytes = [3u8; 20];
        let account_id = AccountID::from(account_bytes);

        // Create an MptId from a tuple
        let sequence_num = 54321u32;
        let mpt_id = MptId::from((sequence_num, account_id));

        // Verify the sequence number and issuer
        assert_eq!(mpt_id.get_sequence_num(), sequence_num);
        assert_eq!(mpt_id.get_issuer(), account_id);
    }
}
