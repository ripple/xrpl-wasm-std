//! Represents a 256-bit hash (like transaction ID)

pub const HASH256_SIZE: usize = 32;

/// A 256-bit (32-byte) hash value used throughout the XRP Ledger.
///
/// Hash256 values are used for:
/// - Transaction IDs
/// - Ledger hashes
/// - Object IDs in the ledger state tree
/// - Various cryptographic operations
///
/// ## Derived Traits
///
/// - `Debug`: Useful for development and debugging
/// - `Clone`: Automatically derived with Copy for consistency
/// - `Copy`: Efficient for this 32-byte struct, enabling implicit copying
/// - `PartialEq, Eq`: Enable hash comparisons and use in hash-based collections
///
/// The `Copy` trait is appropriate here because:
/// - The struct is only 32 bytes, making copies reasonably cheap
/// - Hashes are frequently passed around and compared in ledger operations
/// - Implicit copying improves ergonomics without significant performance concerns
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Hash256(pub [u8; HASH256_SIZE]);

// Implement From<[u8; 32]> to create Hash256 from the array type
impl From<[u8; HASH256_SIZE]> for Hash256 {
    fn from(bytes: [u8; HASH256_SIZE]) -> Self {
        Self(bytes) // Access private field legally here
    }
}

impl Hash256 {
    /// Returns the inner 32 bytes as a reference to the inner array.
    pub fn as_bytes(&self) -> &[u8; HASH256_SIZE] {
        &self.0
    }
}
