//! Vector256 - Array of 256-bit hashes
//!
//! This module provides support for XRPL's Vector256 field type, which represents
//! an array of 32-byte (256-bit) hashes. Common uses include CredentialIDs, NFTokenOffers,
//! and other fields that contain multiple hash values.
//!
//! ## Serialization Format
//!
//! Vector256 fields are serialized as:
//! - 1 byte: total byte length of the data (must be a multiple of 32)
//! - N × 32 bytes: the hash values
//!
//! The count of hashes is calculated as: data_length / 32
//!
//! ## Example
//!
//! ```no_run
//! use xrpl_wasm_stdlib::core::types::vector_256::Vector256;
//! use xrpl_wasm_stdlib::core::types::uint::Hash256;
//!
//! // Parse from raw bytes (64 = 2 hashes × 32 bytes each)
//! let raw_data = [64, /* 64 bytes of hash data */];
//! let vector = Vector256::from_bytes(&raw_data).unwrap();
//! assert_eq!(vector.len(), 2);
//! ```

use crate::core::types::uint::Hash256;
use crate::host::Error;

/// Maximum number of hashes that can be stored in a Vector256
pub const MAX_VECTOR_256_SIZE: usize = 8;

/// A collection of 256-bit hashes with a maximum size
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct Vector256 {
    /// The hash values
    pub hashes: [Hash256; MAX_VECTOR_256_SIZE],
    /// Number of valid hashes (0 to MAX_VECTOR256_SIZE)
    pub len: u8,
}

impl Vector256 {
    /// Creates an empty Vector256
    pub const fn new() -> Self {
        use crate::core::types::uint::UInt;
        const EMPTY_HASH: Hash256 = UInt([0u8; 32]);
        Self {
            hashes: [EMPTY_HASH; MAX_VECTOR_256_SIZE],
            len: 0,
        }
    }

    /// Returns the number of hashes in the vector
    pub fn len(&self) -> usize {
        self.len as usize
    }

    /// Returns true if the vector is empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns a slice of the valid hashes
    pub fn as_slice(&self) -> &[Hash256] {
        &self.hashes[..self.len as usize]
    }

    /// Gets a hash at the specified index, if it exists
    pub fn get(&self, index: usize) -> Option<&Hash256> {
        if index < self.len as usize {
            Some(&self.hashes[index])
        } else {
            None
        }
    }

    /// Parses a Vector256 from raw bytes
    ///
    /// The byte array should be in the format:
    /// - First byte: total byte length of the data (not count of hashes)
    /// - Remaining bytes: 32-byte hashes
    ///
    /// Returns an error if:
    /// - The buffer is too small
    /// - The data length is not a multiple of 32
    /// - The count exceeds MAX_VECTOR256_SIZE
    /// - The buffer doesn't contain enough bytes for all hashes
    pub fn from_bytes(bytes: &[u8]) -> crate::host::Result<Self> {
        use crate::host::trace::{trace, trace_num};

        let _ = trace("Vector256::from_bytes called");
        let _ = trace_num("  bytes.len():", bytes.len() as i64);

        if bytes.is_empty() {
            let _ = trace("  ERROR: Buffer is empty");
            return crate::host::Result::Err(Error::BufferTooSmall);
        }

        // Read VL-encoded length: first byte is total data length in bytes
        let data_len = bytes[0] as usize;
        let _ = trace_num("  data_len (first byte):", data_len as i64);

        // Validate data length is a multiple of 32 (each hash is 32 bytes)
        if data_len % 32 != 0 {
            let _ = trace("  ERROR: data_len is not a multiple of 32");
            let _ = trace_num("  data_len % 32 =", (data_len % 32) as i64);
            return crate::host::Result::Err(Error::InvalidParams);
        }

        // Calculate number of hashes from total data length
        let count = data_len / 32;
        let _ = trace_num("  count (data_len / 32):", count as i64);

        // Enforce maximum size limit
        if count > MAX_VECTOR_256_SIZE {
            let _ = trace("  ERROR: count exceeds MAX_VECTOR_256_SIZE");
            let _ = trace_num("  count:", count as i64);
            let _ = trace_num("  MAX_VECTOR_256_SIZE:", MAX_VECTOR_256_SIZE as i64);
            return crate::host::Result::Err(Error::InvalidParams);
        }

        // Verify buffer contains length byte + all hash data
        let expected_len = 1 + data_len;
        let _ = trace_num("  expected_len (1 + data_len):", expected_len as i64);
        if bytes.len() < expected_len {
            let _ = trace("  ERROR: Buffer too small for expected data");
            let _ = trace_num("  bytes.len():", bytes.len() as i64);
            let _ = trace_num("  expected_len:", expected_len as i64);
            return crate::host::Result::Err(Error::BufferTooSmall);
        }

        // Parse each 32-byte hash from the buffer
        let _ = trace("  Parsing hashes...");
        let mut vector = Self::new();
        vector.len = count as u8;

        for i in 0..count {
            let start = 1 + (i * 32);
            let end = start + 32;
            let mut hash_bytes = [0u8; 32];
            hash_bytes.copy_from_slice(&bytes[start..end]);
            vector.hashes[i] = Hash256::from(hash_bytes);
        }

        let _ = trace("  Vector256::from_bytes SUCCESS");
        crate::host::Result::Ok(vector)
    }

    /// Serializes the Vector256 to bytes
    ///
    /// Returns a buffer containing:
    /// - First byte: total byte length of the data (count × 32)
    /// - Remaining bytes: 32-byte hashes
    pub fn to_bytes(&self) -> ([u8; 1 + (MAX_VECTOR_256_SIZE * 32)], usize) {
        let mut bytes = [0u8; 1 + (MAX_VECTOR_256_SIZE * 32)];
        let data_len = self.len as usize * 32;
        bytes[0] = data_len as u8;

        for i in 0..self.len as usize {
            let start = 1 + (i * 32);
            let end = start + 32;
            bytes[start..end].copy_from_slice(self.hashes[i].as_bytes());
        }

        let total_len = 1 + data_len;
        (bytes, total_len)
    }
}

impl Default for Vector256 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_vector256() {
        let vec = Vector256::new();
        assert_eq!(vec.len(), 0);
        assert!(vec.is_empty());
    }

    #[test]
    fn test_from_bytes_empty() {
        let bytes = [0u8]; // count = 0
        let vec = Vector256::from_bytes(&bytes).unwrap();
        assert_eq!(vec.len(), 0);
        assert!(vec.is_empty());
    }

    #[test]
    fn test_from_bytes_single_hash() {
        let mut bytes = [0u8; 33]; // 1 byte length + 32 bytes hash
        bytes[0] = 32; // data length = 32 (1 hash × 32 bytes)
        // Fill with test data
        for i in 0..32 {
            bytes[1 + i] = i as u8;
        }

        let vec = Vector256::from_bytes(&bytes).unwrap();
        assert_eq!(vec.len(), 1);
        assert!(!vec.is_empty());

        let hash = vec.get(0).unwrap();
        for i in 0..32 {
            assert_eq!(hash.as_bytes()[i], i as u8);
        }
    }

    #[test]
    fn test_from_bytes_multiple_hashes() {
        let mut bytes = [0u8; 65]; // 1 byte length + 64 bytes (2 hashes)
        bytes[0] = 64; // data length = 64 (2 hashes × 32 bytes)

        // First hash: all 0xAA
        for i in 0..32 {
            bytes[1 + i] = 0xAA;
        }

        // Second hash: all 0xBB
        for i in 0..32 {
            bytes[33 + i] = 0xBB;
        }

        let vec = Vector256::from_bytes(&bytes).unwrap();
        assert_eq!(vec.len(), 2);

        let hash0 = vec.get(0).unwrap();
        assert_eq!(hash0.as_bytes()[0], 0xAA);

        let hash1 = vec.get(1).unwrap();
        assert_eq!(hash1.as_bytes()[0], 0xBB);
    }

    #[test]
    fn test_from_bytes_buffer_too_small() {
        let bytes = [64u8, 0, 0]; // Says 64 bytes but only has 2 bytes
        let result = Vector256::from_bytes(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_bytes_too_many_hashes() {
        let mut bytes = [0u8; 1];
        bytes[0] = ((MAX_VECTOR_256_SIZE + 1) * 32) as u8; // data length exceeds max
        let result = Vector256::from_bytes(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_bytes() {
        let mut vec = Vector256::new();
        vec.len = 2;
        vec.hashes[0] = Hash256::from([0xAA; 32]);
        vec.hashes[1] = Hash256::from([0xBB; 32]);

        let (bytes, len) = vec.to_bytes();
        assert_eq!(len, 65); // 1 + 2*32
        assert_eq!(bytes[0], 64); // data length = 64 (2 × 32)
        assert_eq!(bytes[1], 0xAA); // first hash
        assert_eq!(bytes[33], 0xBB); // second hash
    }

    #[test]
    fn test_get() {
        let mut vec = Vector256::new();
        vec.len = 1;
        vec.hashes[0] = Hash256::from([0x42; 32]);

        assert!(vec.get(0).is_some());
        assert!(vec.get(1).is_none());
    }

    #[test]
    fn test_as_slice() {
        let mut vec = Vector256::new();
        vec.len = 2;
        vec.hashes[0] = Hash256::from([0xAA; 32]);
        vec.hashes[1] = Hash256::from([0xBB; 32]);

        let slice = vec.as_slice();
        assert_eq!(slice.len(), 2);
        assert_eq!(slice[0].as_bytes()[0], 0xAA);
        assert_eq!(slice[1].as_bytes()[0], 0xBB);
    }
}

