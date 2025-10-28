
use crate::core::types::account_id::AccountID;
use crate::core::types::amountv1::currency_code::CurrencyCode;
use crate::core::types::issue::{Issue, MPTIssue, is_xrp_currency, xrp_issue};
use crate::core::types::mpt_id::MptId;

/// Represents an asset without a value.
/// This is the Rust equivalent of C++'s Asset type.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub enum Asset {
    /// Native XRP asset
    XRP,
    /// Issued currency (IOU)
    IOU(Issue),
    /// Multi-Purpose Token
    MPT(MPTIssue),
}

impl Asset {
    /// Create a new XRP asset
    pub const fn xrp() -> Self {
        Asset::XRP
    }

    /// Create a new IOU asset
    pub const fn iou(currency: CurrencyCode, account: AccountID) -> Self {
        Asset::IOU(Issue::new(currency, account))
    }

    /// Create a new MPT asset
    pub const fn mpt(mpt_id: MptId) -> Self {
        Asset::MPT(MPTIssue::new(mpt_id))
    }

    /// Check if this asset is XRP
    pub const fn is_xrp(&self) -> bool {
        matches!(self, Asset::XRP)
    }

    /// Check if this asset is an IOU
    pub const fn is_iou(&self) -> bool {
        matches!(self, Asset::IOU(_))
    }

    /// Check if this asset is an MPT
    pub const fn is_mpt(&self) -> bool {
        matches!(self, Asset::MPT(_))
    }

    /// Get the currency code.
    /// Returns the currency for IOUs, XRP_CURRENCY for XRP, None for MPT.
    pub fn currency(&self) -> Option<CurrencyCode> {
        match self {
            Asset::XRP => Some(CurrencyCode([0u8; 20])),
            Asset::IOU(issue) => Some(issue.currency),
            Asset::MPT(_) => None,
        }
    }

    /// Get the account/issuer.
    /// Returns the issuer for IOUs, XRP_ACCOUNT for XRP, issuer extracted from MPTID for MPT.
    pub fn account(&self) -> AccountID {
        match self {
            Asset::XRP => AccountID([0u8; 20]),
            Asset::IOU(issue) => issue.account,
            Asset::MPT(mpt) => mpt.issuer(),
        }
    }

    /// Get the MPT ID if this is an MPT
    pub fn mpt_id(&self) -> Option<MptId> {
        match self {
            Asset::MPT(mpt) => Some(mpt.mpt_id()),
            _ => None,
        }
    }

    /// Get the underlying Issue if this is an IOU
    pub fn as_issue(&self) -> Option<Issue> {
        match self {
            Asset::IOU(issue) => Some(*issue),
            Asset::XRP => Some(xrp_issue()),
            _ => None,
        }
    }

    /// Get the underlying MPTIssue if this is an MPT
    pub fn as_mpt_issue(&self) -> Option<MPTIssue> {
        match self {
            Asset::MPT(mpt) => Some(*mpt),
            _ => None,
        }
    }

    /// Serialize the asset to bytes.
    /// Returns (buffer, length) where buffer contains the serialized data.
    /// For XRP: 20 bytes
    /// For IOU: 40 bytes (currency + issuer)
    /// For MPT: 44 bytes (issuer + black hole + sequence)
    pub fn to_bytes(&self, buffer: &mut [u8]) -> usize {
        match self {
            Asset::XRP => {
                // Zero out first 20 bytes
                let mut i = 0;
                while i < 20 {
                    buffer[i] = 0;
                    i += 1;
                }
                20
            }
            Asset::IOU(issue) => {
                // Currency (20 bytes)
                let mut i = 0;
                while i < 20 {
                    buffer[i] = issue.currency.0[i];
                    i += 1;
                }
                
                // Issuer (20 bytes) - only if not XRP currency
                if !is_xrp_currency(issue.currency) {
                    let mut i = 0;
                    while i < 20 {
                        buffer[20 + i] = issue.account.0[i];
                        i += 1;
                    }
                    40
                } else {
                    20
                }
            }
            Asset::MPT(mpt) => {
                let mpt_id_bytes = mpt.mpt_id().0;
                
                // MPT serialization per C++ STIssue::add():
                // 1. Issuer (20 bytes) - bytes 4-23 of MPTID
                // 2. Black hole account (20 bytes) - all zeros  
                // 3. Sequence (4 bytes) - bytes 0-3 of MPTID
                
                // Write issuer (bytes 4-23 from MPTID)
                let mut i = 0;
                while i < 20 {
                    buffer[i] = mpt_id_bytes[4 + i];
                    i += 1;
                }
                
                // Write black hole account (all zeros, 20 bytes)
                i = 0;
                while i < 20 {
                    buffer[20 + i] = 0;
                    i += 1;
                }
                
                // Write sequence (first 4 bytes of MPTID)
                i = 0;
                while i < 4 {
                    buffer[40 + i] = mpt_id_bytes[i];
                    i += 1;
                }
                
                44
            }
        }
    }
}

impl Default for Asset {
    fn default() -> Self {
        Asset::xrp()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xrp_asset() {
        let asset = Asset::xrp();
        assert!(asset.is_xrp());
        assert!(!asset.is_iou());
        assert!(!asset.is_mpt());
        
        // XRP should return XRP currency
        assert_eq!(asset.currency(), Some(CurrencyCode([0u8; 20])));
    }

    #[test]
    fn test_iou_asset() {
        let currency = CurrencyCode::from(*b"USD");
        let issuer = AccountID::from([1u8; 20]);
        let asset = Asset::iou(currency, issuer);
        
        assert!(!asset.is_xrp());
        assert!(asset.is_iou());
        assert!(!asset.is_mpt());
        
        // Test accessor pattern: asset.currency() returns CurrencyCode
        assert_eq!(asset.currency(), Some(currency));
        assert_eq!(asset.account(), issuer);
    }

    #[test]
    fn test_mpt_asset() {
        let mpt_id = MptId::from([2u8; 24]);
        let asset = Asset::mpt(mpt_id);
        
        assert!(!asset.is_xrp());
        assert!(!asset.is_iou());
        assert!(asset.is_mpt());
        
        assert_eq!(asset.mpt_id(), Some(mpt_id));
        assert_eq!(asset.currency(), None);
    }

    #[test]
    fn test_accessor_chaining() {
        // Test the pattern: asset.currency() directly returns the type
        let currency = CurrencyCode::from(*b"EUR");
        let issuer = AccountID::from([3u8; 20]);
        let asset = Asset::iou(currency, issuer);
        
        // Direct accessors
        if let Some(curr) = asset.currency() {
            assert_eq!(curr, currency);
        }
        
        assert_eq!(asset.account(), issuer);
        
        // Via Issue
        if let Some(issue) = asset.as_issue() {
            assert_eq!(issue.currency, currency);
            assert_eq!(issue.account, issuer);
        }
    }

    #[test]
    fn test_serialization() {
        let mut buffer = [0u8; 64];
        
        // Test XRP
        let xrp = Asset::xrp();
        let len = xrp.to_bytes(&mut buffer);
        assert_eq!(len, 20);
        assert_eq!(&buffer[..20], &[0u8; 20]);
        
        // Test IOU
        let currency = CurrencyCode::from(*b"USD");
        let issuer = AccountID::from([1u8; 20]);
        let iou = Asset::iou(currency, issuer);
        let len = iou.to_bytes(&mut buffer);
        assert_eq!(len, 40);
        
        // Test MPT
        let mpt_id = MptId::from([2u8; 24]);
        let mpt = Asset::mpt(mpt_id);
        let len = mpt.to_bytes(&mut buffer);
        assert_eq!(len, 44);
    }
}