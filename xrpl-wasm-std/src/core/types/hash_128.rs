//! Represents a 128-bit number

pub const HASH128_SIZE: usize = 16;

#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Hash128(pub [u8; HASH128_SIZE]);

// Implement From<[u8; 16]> to create Hash128 from the array type
impl From<[u8; HASH128_SIZE]> for Hash128 {
    fn from(bytes: [u8; HASH128_SIZE]) -> Self {
        Self(bytes) // Access private field legally here
    }
}
