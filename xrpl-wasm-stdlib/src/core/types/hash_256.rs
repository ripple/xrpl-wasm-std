//! Represents a 256-bit hash (like transaction ID)

pub const HASH256_SIZE: usize = 32;

#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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
