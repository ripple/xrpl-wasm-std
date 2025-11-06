/// A crypto-condition Condition. The maximum size is based on the crypto-condition format.
///
/// Byte-encoding For PREIMAGE-SHA-256:
/// 2 bytes (type) + 2 bytes (length tag) + 32 bytes (hash) + 2 bytes (cost tag) + 1 byte (cost) = 39 bytes (generally)

/// The maximum number of bytes in a Condition. Xrpld currently caps this value at 256 bytes
/// (see `maxSerializedFulfillment` in xrpld source code), so we do the same here.
pub const MAX_CONDITION_SIZE: usize = 128;
pub struct Condition {
    /// The full raw condition data in crypto-condition format
    pub data: [u8; MAX_CONDITION_SIZE],

    /// The actual length of this Condition, if less than data.len()
    pub len: usize,
}

impl From<[u8; MAX_CONDITION_SIZE]> for Condition {
    fn from(bytes: [u8; MAX_CONDITION_SIZE]) -> Self {
        Self {
            data: bytes,
            len: MAX_CONDITION_SIZE,
        }
    }
}

/// A crypto-condition Fulfillment.
///
/// ## Fulfillment Format (PREIMAGE-SHA-256)
///
/// A PREIMAGE-SHA-256 fulfillment follows this structure:
///
/// ```text
/// [Type Tag] [Length Tag] [Preimage Data]
/// ```
///
/// ### Example: `A0058003736868`
///
/// Breaking down the bytes:
/// - `A0` = PREIMAGE-SHA-256 fulfillment type tag (1 byte)
/// - `05` = Total length of remaining data (1 byte) = 5 bytes
/// - `80` = Preimage data tag (1 byte)
/// - `03` = Length of preimage (1 byte) = 3 bytes
/// - `736868` = Preimage data (3 bytes) = "shh" in ASCII
///
/// Total: 7 bytes
///
/// ### Another Example: `A0028000` (empty preimage)
///
/// Breaking down the bytes:
/// - `A0` = PREIMAGE-SHA-256 fulfillment type tag (1 byte)
/// - `02` = Total length of remaining data (1 byte) = 2 bytes
/// - `80` = Preimage data tag (1 byte)
/// - `00` = Length of preimage (1 byte) = 0 bytes
/// - (no preimage data)
///
/// Total: 4 bytes

/// The maximum number of bytes in a Fulfillment. Theoretically, the crypto-condition format allows for much larger
/// fulfillments, but xrpld currently caps this value at 256 bytes (see `maxSerializedFulfillment` in xrpld source
/// code), so we do the same here.
pub const MAX_FULFILLMENT_SIZE: usize = 256;
///
pub struct Fulfillment {
    /// The full raw fulfillment data in crypto-condition format
    pub data: [u8; 256],

    /// The actual length of this Fulfillment, if less than data.len()
    pub len: usize,
}

impl From<[u8; MAX_FULFILLMENT_SIZE]> for Fulfillment {
    fn from(bytes: [u8; MAX_FULFILLMENT_SIZE]) -> Self {
        Self {
            data: bytes,
            len: MAX_FULFILLMENT_SIZE,
        }
    }
}

#[cfg(test)]
mod test_condition {
    use super::*;

    // PREIMAGE-SHA-256 crypto-condition (39 bytes)
    // Format: A0258020<32-byte-hash>810100
    const TEST_CONDITION_DATA: [u8; 39] = [
        0xA0, 0x25, 0x80, 0x20, // Type and length tags
        0xE3, 0xB0, 0xC4, 0x42, 0x98, 0xFC, 0x1C, 0x14, 0x9A, 0xFB, 0xF4, 0xC8, 0x99, 0x6F, 0xB9,
        0x24, 0x27, 0xAE, 0x41, 0xE4, 0x64, 0x9B, 0x93, 0x4C, 0xA4, 0x95, 0x99, 0x1B, 0x78, 0x52,
        0xB8, 0x55, // SHA-256 hash (32 bytes)
        0x81, 0x01, 0x00, // Cost tags
    ];

    #[test]
    fn test_condition() {
        let mut condition = Condition {
            data: [0u8; MAX_CONDITION_SIZE],
            len: TEST_CONDITION_DATA.len(),
        };
        condition.data[..TEST_CONDITION_DATA.len()].copy_from_slice(&TEST_CONDITION_DATA);

        assert_eq!(condition.len, 39);
        assert_eq!(&condition.data[..condition.len], &TEST_CONDITION_DATA);
    }
}

#[cfg(test)]
mod test_fulfillment {
    use super::*;

    #[test]
    fn test_fulfillment_empty_preimage() {
        // PREIMAGE-SHA-256 fulfillment with empty preimage (4 bytes)
        // Format: A0 02 80 00
        // A0 = PREIMAGE-SHA-256 fulfillment type tag
        // 02 = Total length of remaining data (2 bytes)
        // 80 = Preimage data tag
        // 00 = Length of preimage (0 bytes)
        const TEST_FULFILLMENT_EMPTY: [u8; 4] = [0xA0, 0x02, 0x80, 0x00];

        let mut fulfillment = Fulfillment {
            data: [0u8; 256],
            len: TEST_FULFILLMENT_EMPTY.len(),
        };
        fulfillment.data[..TEST_FULFILLMENT_EMPTY.len()].copy_from_slice(&TEST_FULFILLMENT_EMPTY);

        assert_eq!(fulfillment.len, 4);
        assert_eq!(
            &fulfillment.data[..fulfillment.len],
            &TEST_FULFILLMENT_EMPTY
        );

        // Verify structure
        assert_eq!(fulfillment.data[0], 0xA0); // Type tag
        assert_eq!(fulfillment.data[1], 0x02); // Length of remaining data
        assert_eq!(fulfillment.data[2], 0x80); // Preimage data tag
        assert_eq!(fulfillment.data[3], 0x00); // Preimage length (0)
    }

    #[test]
    fn test_fulfillment_with_preimage() {
        // PREIMAGE-SHA-256 fulfillment with preimage "shh" (7 bytes)
        // Format: A0 05 80 03 73 68 68
        // A0 = PREIMAGE-SHA-256 fulfillment type tag
        // 05 = Total length of remaining data (5 bytes)
        // 80 = Preimage data tag
        // 03 = Length of preimage (3 bytes)
        // 736868 = "shh" in ASCII
        const TEST_FULFILLMENT_SHH: [u8; 7] = [0xA0, 0x05, 0x80, 0x03, 0x73, 0x68, 0x68];

        let mut fulfillment = Fulfillment {
            data: [0u8; 256],
            len: TEST_FULFILLMENT_SHH.len(),
        };
        fulfillment.data[..TEST_FULFILLMENT_SHH.len()].copy_from_slice(&TEST_FULFILLMENT_SHH);

        assert_eq!(fulfillment.len, 7);
        assert_eq!(&fulfillment.data[..fulfillment.len], &TEST_FULFILLMENT_SHH);

        // Verify structure
        assert_eq!(fulfillment.data[0], 0xA0); // Type tag
        assert_eq!(fulfillment.data[1], 0x05); // Length of remaining data
        assert_eq!(fulfillment.data[2], 0x80); // Preimage data tag
        assert_eq!(fulfillment.data[3], 0x03); // Preimage length (3)
        assert_eq!(&fulfillment.data[4..7], b"shh"); // Preimage data
    }
}
