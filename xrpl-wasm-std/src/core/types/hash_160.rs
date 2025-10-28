//! Represents a 160-bit number

pub const HASH160_SIZE: usize = 20;

#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Hash160(pub [u8; HASH160_SIZE]);

// Implement From<[u8; 20]> to create Hash160 from the array type
impl From<[u8; HASH160_SIZE]> for Hash160 {
    fn from(bytes: [u8; HASH160_SIZE]) -> Self {
        Self(bytes) // Access private field legally here
    }
}

impl Hash160 {
    /// Returns the inner 20 bytes as a reference to the inner array.
    pub fn as_bytes(&self) -> &[u8; HASH160_SIZE] {
        &self.0
    }
}