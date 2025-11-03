/// A 32-byte crypto-condition used in escrows and payment channels.
///
/// ## Derived Traits
///
/// - `Debug`: Useful for development and debugging
/// - `Clone`: Automatically derived with Copy for consistency
/// - `Copy`: Efficient for this 32-byte struct, enabling implicit copying
/// - `PartialEq, Eq`: Enable condition comparisons
///
/// The `Copy` trait is appropriate here because:
/// - The struct is only 32 bytes, making copies reasonably cheap
/// - Conditions are frequently checked and compared
/// - Implicit copying improves ergonomics without significant performance concerns
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Condition(pub [u8; 32]);

impl From<[u8; 32]> for Condition {
    fn from(bytes: [u8; 32]) -> Self {
        Self(bytes) // Access private field legally here
    }
}

/// A crypto-condition Fulfillment. Note that from rippled source, this value is currently capped
/// at 256 bytes, which allows us to treat it as such.
///
/// ## Derived Traits
///
/// - `Debug`: Useful for development and debugging
/// - `Clone`: Reasonable for this 256-byte struct when explicit copying is needed
/// - `PartialEq, Eq`: Enable fulfillment comparisons
///
/// Note: `Copy` is intentionally not derived due to the struct's size (256+ bytes).
/// Large `Copy` types can lead to accidental expensive copies and poor performance.
/// Use `.clone()` when you need to duplicate a fulfillment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fulfillment {
    pub data: [u8; 256],

    /// The actual length of this Fulfillment, if less than data.len()
    pub len: usize,
}

// impl From<[u8; 256]> for Fulfillment {
//     fn from(bytes: [u8; 256]) -> Self {
//         Self(bytes) // Access private field legally here
//     }
// }

#[cfg(test)]
mod test_public_key {
    // secp256k1
    const TEST_CONDITION: [u8; 32] = [
        0x02, 0xC7, 0x38, 0x7F, 0xFC, 0x25, 0xC1, 0x56, 0xCA, 0x7F, 0x8A, 0x6D, 0x76, 0x0C, 0x8D,
        0x01, 0xEF, 0x64, 0x2C, 0xEE, 0x9C, 0xE4, 0x68, 0x0C, 0x33, 0xFF, 0xB3, 0xFF, 0x39, 0xAF,
        0xEC, 0xFE,
    ];

    #[test]
    fn test_condition() {
        let condition_slice: &[u8] = TEST_CONDITION.as_slice();

        assert_eq!(condition_slice.len(), 32);
        assert_eq!(condition_slice, TEST_CONDITION);
    }
}
