//! Represents a 192-bit number

pub const HASH192_SIZE: usize = 24;

#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Hash192(pub [u8; HASH192_SIZE]);

// Implement From<[u8; 24]> to create Hash192 from the array type
impl From<[u8; HASH192_SIZE]> for Hash192 {
    fn from(bytes: [u8; HASH192_SIZE]) -> Self {
        Self(bytes) // Access private field legally here
    }
}
