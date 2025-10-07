pub const CURRENCY_CODE_SIZE: usize = 20;
pub const STANDARD_CURRENCY_CODE_SIZE: usize = 3; // For standard currencies like USD, EUR, etc.

/// Represents a currency code in the XRPL, which is a 20-byte identifier.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct CurrencyCode(pub [u8; CURRENCY_CODE_SIZE]);

impl CurrencyCode {
    /// Creates a new CurrencyCode from a 20-byte array.
    pub fn new(code: [u8; CURRENCY_CODE_SIZE]) -> Self {
        CurrencyCode(code)
    }

    /// Gets the raw bytes of the CurrencyCode.
    pub fn as_bytes(&self) -> &[u8; CURRENCY_CODE_SIZE] {
        &self.0
    }
}

impl From<[u8; CURRENCY_CODE_SIZE]> for CurrencyCode {
    fn from(value: [u8; CURRENCY_CODE_SIZE]) -> Self {
        CurrencyCode(value)
    }
}

// Implement From<[u8; 3]> to create CurrencyCode from the standard currency array type
impl From<[u8; STANDARD_CURRENCY_CODE_SIZE]> for CurrencyCode {
    fn from(bytes: [u8; STANDARD_CURRENCY_CODE_SIZE]) -> Self {
        let mut arr = [0u8; CURRENCY_CODE_SIZE];
        arr[12..15].copy_from_slice(&bytes);
        Self(arr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_code_creation() {
        // Create a test currency code
        let code_bytes = [1u8; CURRENCY_CODE_SIZE];
        let currency_code = CurrencyCode::new(code_bytes);

        // Verify the bytes
        assert_eq!(currency_code.as_bytes(), &code_bytes);
    }

    #[test]
    fn test_currency_code_from_bytes() {
        // Create a test byte array
        let bytes = [2u8; CURRENCY_CODE_SIZE];

        // Create a CurrencyCode from bytes
        let currency_code = CurrencyCode::from(bytes);

        // Verify the bytes
        assert_eq!(currency_code.as_bytes(), &bytes);
    }

    #[test]
    fn test_currency_code_equality() {
        // Create two identical currency codes
        let code1 = CurrencyCode::new([3u8; CURRENCY_CODE_SIZE]);
        let code2 = CurrencyCode::new([3u8; CURRENCY_CODE_SIZE]);

        // Create a different currency code
        let code3 = CurrencyCode::new([4u8; CURRENCY_CODE_SIZE]);

        // Test equality
        assert_eq!(code1, code2);
        assert_ne!(code1, code3);
    }

    #[test]
    fn test_currency_code_from_standard_bytes() {
        // Create a 3-byte array representing "USD"
        let standard_bytes = *b"USD";

        // Convert to CurrencyCode
        let currency_code = CurrencyCode::from(standard_bytes);

        // Create the expected 20-byte array (zeros with "USD" at positions 12-14)
        let mut expected = [0u8; CURRENCY_CODE_SIZE];
        expected[12..15].copy_from_slice(&standard_bytes);

        // Verify the bytes
        assert_eq!(currency_code.as_bytes(), &expected);
    }
}
