pub const BLOB_SIZE: usize = 1024;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct Blob {
    pub data: [u8; BLOB_SIZE],

    /// The actual length of this blob, if less than data.len()
    pub len: usize,
}

pub const EMPTY_BLOB: Blob = Blob {
    data: [0u8; BLOB_SIZE], // TODO: Consider an optional?
    len: 0usize,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blob_size_constant() {
        // Verify the BLOB_SIZE constant is 1024
        assert_eq!(BLOB_SIZE, 1024);
    }

    #[test]
    fn test_empty_blob_constant() {
        // Verify EMPTY_BLOB has zero length
        assert_eq!(EMPTY_BLOB.len, 0);

        // Verify EMPTY_BLOB data is all zeros
        assert_eq!(EMPTY_BLOB.data, [0u8; BLOB_SIZE]);
    }

    #[test]
    fn test_blob_creation() {
        // Create a test blob with some data
        let mut data = [0u8; BLOB_SIZE];
        data[0] = 0xAB;
        data[1] = 0xCD;
        data[2] = 0xEF;

        let blob = Blob { data, len: 3 };

        // Verify the blob properties
        assert_eq!(blob.len, 3);
        assert_eq!(blob.data[0], 0xAB);
        assert_eq!(blob.data[1], 0xCD);
        assert_eq!(blob.data[2], 0xEF);
    }

    #[test]
    fn test_blob_full_size() {
        // Create a blob that uses the full buffer
        let data = [0xFF; BLOB_SIZE];
        let blob = Blob {
            data,
            len: BLOB_SIZE,
        };

        // Verify the blob uses full capacity
        assert_eq!(blob.len, BLOB_SIZE);
        assert_eq!(blob.data.len(), BLOB_SIZE);
        assert!(blob.data.iter().all(|&b| b == 0xFF));
    }

    #[test]
    fn test_blob_partial_length() {
        // Create a blob where actual length is less than buffer size
        let mut data = [0u8; BLOB_SIZE];
        let test_data = b"Hello, XRPL!";
        data[..test_data.len()].copy_from_slice(test_data);

        let blob = Blob {
            data,
            len: test_data.len(),
        };

        // Verify the actual length is tracked correctly
        assert_eq!(blob.len, test_data.len());
        assert_eq!(&blob.data[..blob.len], test_data);

        // Verify remaining bytes are zero
        assert!(blob.data[blob.len..].iter().all(|&b| b == 0));
    }

    #[test]
    #[allow(clippy::clone_on_copy)]
    fn test_blob_clone() {
        // Create a test blob
        let mut data = [0u8; BLOB_SIZE];
        data[0] = 0x12;
        data[1] = 0x34;

        let blob1 = Blob { data, len: 2 };

        // Clone the blob
        let blob2 = blob1.clone();

        // Verify the clone is identical
        assert_eq!(blob1, blob2);
        assert_eq!(blob1.len, blob2.len);
        assert_eq!(blob1.data, blob2.data);
    }

    #[test]
    fn test_blob_copy() {
        // Create a test blob
        let mut data = [0u8; BLOB_SIZE];
        data[0] = 0xAA;

        let blob1 = Blob { data, len: 1 };

        // Copy the blob
        let blob2 = blob1;

        // Verify the copy is identical
        assert_eq!(blob1, blob2);

        // Verify we can still use blob1 (proving it's Copy, not just Clone)
        assert_eq!(blob1.len, 1);
        assert_eq!(blob2.len, 1);
    }

    #[test]
    fn test_blob_equality() {
        // Create two identical blobs
        let mut data1 = [0u8; BLOB_SIZE];
        data1[0] = 0x42;

        let mut data2 = [0u8; BLOB_SIZE];
        data2[0] = 0x42;

        let blob1 = Blob {
            data: data1,
            len: 1,
        };

        let blob2 = Blob {
            data: data2,
            len: 1,
        };

        // Verify equality
        assert_eq!(blob1, blob2);
    }

    #[test]
    fn test_blob_inequality_different_length() {
        // Create two blobs with same data but different lengths
        let data = [0u8; BLOB_SIZE];

        let blob1 = Blob { data, len: 10 };

        let blob2 = Blob { data, len: 20 };

        // Verify inequality
        assert_ne!(blob1, blob2);
    }

    #[test]
    fn test_blob_inequality_different_data() {
        // Create two blobs with different data
        let mut data1 = [0u8; BLOB_SIZE];
        data1[0] = 0x01;

        let mut data2 = [0u8; BLOB_SIZE];
        data2[0] = 0x02;

        let blob1 = Blob {
            data: data1,
            len: 1,
        };

        let blob2 = Blob {
            data: data2,
            len: 1,
        };

        // Verify inequality
        assert_ne!(blob1, blob2);
    }

    #[test]
    fn test_blob_debug_format() {
        // Create a test blob
        let blob = Blob {
            data: [0u8; BLOB_SIZE],
            len: 0,
        };

        // Verify Debug trait is implemented (should not panic)
        // We can't use format! in no_std, but we can verify the trait exists
        let _ = &blob as &dyn core::fmt::Debug;
    }

    #[test]
    fn test_empty_blob_equals_new_empty() {
        // Create a new empty blob manually
        let new_empty = Blob {
            data: [0u8; BLOB_SIZE],
            len: 0,
        };

        // Verify it equals EMPTY_BLOB constant
        assert_eq!(EMPTY_BLOB, new_empty);
    }

    #[test]
    #[allow(clippy::needless_range_loop)]
    fn test_blob_with_binary_data() {
        // Test with various binary patterns
        let mut data = [0u8; BLOB_SIZE];

        // Fill with a pattern
        for i in 0..256 {
            data[i] = i as u8;
        }

        let blob = Blob { data, len: 256 };

        // Verify the pattern
        assert_eq!(blob.len, 256);
        for i in 0..256 {
            assert_eq!(blob.data[i], i as u8);
        }
    }

    #[test]
    fn test_blob_zero_length_with_data() {
        // Create a blob with data but zero length
        // This represents a blob where no actual data is valid
        let data = [0xFF; BLOB_SIZE];

        let blob = Blob { data, len: 0 };

        // Verify length is zero even though buffer has data
        assert_eq!(blob.len, 0);
        // The data field still contains 0xFF, but len indicates it's not valid
        assert_eq!(blob.data[0], 0xFF);
    }
}
