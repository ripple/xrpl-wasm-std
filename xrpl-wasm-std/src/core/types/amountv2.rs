use crate::core::types::account_id::AccountID;
use crate::core::types::asset::Asset;
use crate::core::types::currency::Currency;
use crate::core::types::mpt_id::MptId;

/// Represents an amount of value with an associated asset.
/// This is the Rust equivalent of C++'s STAmount.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct Amount {
    /// The asset (currency type)
    asset: Asset,
    /// The mantissa (value)
    mantissa: i64,
    /// The exponent (for IOUs, indicates decimal place)
    exponent: i32,
    /// Whether this is negative
    negative: bool,
}

impl Amount {
    /// Create a new XRP amount (in drops)
    pub const fn xrp(drops: i64) -> Self {
        let (mantissa, negative) = if drops < 0 {
            (-drops, true)
        } else {
            (drops, false)
        };
        
        Self {
            asset: Asset::xrp(),
            mantissa,
            exponent: 0,
            negative,
        }
    }

    /// Create a new IOU amount
    pub const fn iou(mantissa: i64, exponent: i32, currency: Currency, account: AccountID) -> Self {
        let (abs_mantissa, negative) = if mantissa < 0 {
            (-mantissa, true)
        } else {
            (mantissa, false)
        };
        
        Self {
            asset: Asset::iou(currency, account),
            mantissa: abs_mantissa,
            exponent,
            negative,
        }
    }

    /// Create a new MPT amount
    pub const fn mpt(value: u64, mpt_id: MptId) -> Self {
        Self {
            asset: Asset::mpt(mpt_id),
            mantissa: value as i64,
            exponent: 0,
            negative: false, // MPT amounts are always non-negative
        }
    }

    /// Get the asset
    pub const fn asset(&self) -> Asset {
        self.asset
    }

    /// Get the mantissa (absolute value)
    pub const fn mantissa(&self) -> i64 {
        self.mantissa
    }

    /// Get the exponent
    pub const fn exponent(&self) -> i32 {
        self.exponent
    }

    /// Check if negative
    pub const fn is_negative(&self) -> bool {
        self.negative
    }

    /// Get the signed mantissa
    pub const fn signed_mantissa(&self) -> i64 {
        if self.negative {
            -self.mantissa
        } else {
            self.mantissa
        }
    }

    // /// Get the issue (for IOUs and XRP)
    // pub fn asset(&self) -> Option<Issue> {
    //     self.asset.as_issue()
    // }

    /// Get drops (for XRP only)
    pub const fn drops(&self) -> Option<i64> {
        if self.asset.is_xrp() {
            Some(self.signed_mantissa())
        } else {
            None
        }
    }

    /// Serialize to bytes.
    /// Returns the number of bytes written to buffer.
    /// Buffer must be at least 48 bytes for IOU, 8 bytes for XRP, or appropriate size for MPT.
    pub fn to_bytes(&self, buffer: &mut [u8]) -> usize {
        if self.asset.is_xrp() {
            // XRP: 8 bytes with bit flags
            let amount_bits = if self.negative {
                self.mantissa as u64
            } else {
                0x4000_0000_0000_0000u64 | (self.mantissa as u64)
            };
            
            // Write 8 bytes in big-endian
            let bytes = amount_bits.to_be_bytes();
            let mut i = 0;
            while i < 8 {
                buffer[i] = bytes[i];
                i += 1;
            }
            8
        } else if self.asset.is_iou() {
            // IOU: 8 bytes (amount) + 20 bytes (currency) + 20 bytes (issuer)
            let not_xrp_bit = 0x8000_0000_0000_0000u64;
            let sign_bit = if self.negative { 0x4000_0000_0000_0000u64 } else { 0 };
            let exponent_bits = ((self.exponent + 97) as u64) << 54;
            let mantissa_bits = (self.mantissa as u64) & 0x3F_FFFF_FFFF_FFFF;
            
            let amount_bits = not_xrp_bit | sign_bit | exponent_bits | mantissa_bits;
            let bytes = amount_bits.to_be_bytes();
            
            // Write amount (8 bytes)
            let mut i = 0;
            while i < 8 {
                buffer[i] = bytes[i];
                i += 1;
            }
            
            // Write asset bytes (currency + issuer)
            let asset_len = self.asset.to_bytes(&mut buffer[8..]);
            8 + asset_len
        } else {
            // MPT: 8 bytes (value) + asset bytes
            let bytes = (self.mantissa as u64).to_be_bytes();
            let mut i = 0;
            while i < 8 {
                buffer[i] = bytes[i];
                i += 1;
            }
            
            let asset_len = self.asset.to_bytes(&mut buffer[8..]);
            8 + asset_len
        }
    }

    /// Deserialize from bytes.
    /// Returns the Amount and the number of bytes consumed.
    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), &'static str> {
        if bytes.len() < 8 {
            return Err("Insufficient bytes for amount");
        }

        // Read first 8 bytes as u64 in big-endian
        let mut amount_bytes = [0u8; 8];
        let mut i = 0;
        while i < 8 {
            amount_bytes[i] = bytes[i];
            i += 1;
        }
        let amount_bits = u64::from_be_bytes(amount_bytes);
        
        // Check if XRP or IOU
        if (amount_bits & 0x8000_0000_0000_0000) == 0 {
            // XRP
            let positive = (amount_bits & 0x4000_0000_0000_0000) != 0;
            let drops = (amount_bits & 0x3FFF_FFFF_FFFF_FFFF) as i64;
            
            Ok((Amount::xrp(if positive { drops } else { -drops }), 8))
        } else {
            // IOU or MPT
            if bytes.len() < 48 {
                return Err("Insufficient bytes for IOU amount");
            }
            
            let negative = (amount_bits & 0x4000_0000_0000_0000) != 0;
            let exponent = (((amount_bits >> 54) & 0xFF) as i32) - 97;
            let mantissa = (amount_bits & 0x3F_FFFF_FFFF_FFFF) as i64;
            
            let mut currency_bytes = [0u8; 20];
            i = 0;
            while i < 20 {
                currency_bytes[i] = bytes[8 + i];
                i += 1;
            }
            let currency = Currency(currency_bytes);
            
            let mut account_bytes = [0u8; 20];
            i = 0;
            while i < 20 {
                account_bytes[i] = bytes[28 + i];
                i += 1;
            }
            let account = AccountID(account_bytes);
            
            let signed_mantissa = if negative { -mantissa } else { mantissa };
            Ok((Amount::iou(signed_mantissa, exponent, currency, account), 48))
        }
    }
}

impl Default for Amount {
    fn default() -> Self {
        Amount::xrp(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xrp_amount() {
        let amount = Amount::xrp(1_000_000);
        
        assert!(amount.asset.is_xrp());
        assert!(!amount.is_negative());
        assert_eq!(amount.drops(), Some(1_000_000));
        
        // Test accessor pattern
        assert!(amount.asset.currency().is_some());
    }

    #[test]
    fn test_negative_xrp() {
        let amount = Amount::xrp(-500_000);
        
        assert!(amount.is_negative());
        assert_eq!(amount.drops(), Some(-500_000));
        assert_eq!(amount.signed_mantissa(), -500_000);
    }

    #[test]
    fn test_iou_amount() {
        let currency = Currency::from(*b"USD");
        let issuer = AccountID::from([1u8; 20]);
        let amount = Amount::iou(100, -2, currency, issuer);
        
        assert!(amount.asset.is_iou());
        
        // Test accessors - both patterns should work
        assert_eq!(amount.asset.currency(), Some(currency));
        assert_eq!(amount.asset.account(), issuer);
        assert_eq!(amount.asset().currency(), Some(currency));
    }

    #[test]
    fn test_mpt_amount() {
        let mpt_id = MptId::from([2u8; 24]);
        let amount = Amount::mpt(500, mpt_id);
        
        assert!(amount.asset.is_mpt());
        assert!(!amount.is_negative()); // MPT amounts are never negative
        assert_eq!(amount.asset.mpt_id(), Some(mpt_id));
    }

    // #[test]
    // fn test_zero_amounts() {
    //     assert!(Amount::xrp(0).asset.is_zero());
        
    //     let currency = Currency::from(*b"EUR");
    //     let issuer = AccountID::from([3u8; 20]);
    //     assert!(Amount::iou(0, 0, currency, issuer).asset.is_zero());
    // }

    #[test]
    fn test_serialization() {
        let mut buffer = [0u8; 64];
        
        let amount = Amount::xrp(1_000_000);
        let len = amount.to_bytes(&mut buffer);
        
        assert_eq!(len, 8);
        
        let (deserialized, consumed) = Amount::from_bytes(&buffer).unwrap();
        assert_eq!(amount, deserialized);
        assert_eq!(consumed, 8);
    }

    #[test]
    fn test_accessor_patterns() {
        let currency = Currency::from(*b"GBP");
        let issuer = AccountID::from([4u8; 20]);
        let amount = Amount::iou(50, 0, currency, issuer);
        
        // Pattern 1: Direct on amount
        assert_eq!(amount.asset.currency(), Some(currency));
        
        // Pattern 2: Through asset (amount.asset().currency())
        assert_eq!(amount.asset().currency(), Some(currency));
        
        // Both should return the actual Currency, not wrapped in extra layers
        if let Some(curr) = amount.asset.currency() {
            assert_eq!(curr, currency);
        }
    }

    #[test]
    fn test_const_constructors() {
        // Test that constructors can be used in const contexts
        const XRP_AMT: Amount = Amount::xrp(1000);
        assert_eq!(XRP_AMT.drops(), Some(1000));
        
        const CURRENCY: Currency = Currency([0u8; 20]);
        const ACCOUNT: AccountID = AccountID([0u8; 20]);
        const IOU_AMT: Amount = Amount::iou(100, 0, CURRENCY, ACCOUNT);
        assert_eq!(IOU_AMT.mantissa(), 100);
    }
}