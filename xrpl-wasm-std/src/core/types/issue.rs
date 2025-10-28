use crate::core::types::account_id::AccountID;
use crate::core::types::currency::Currency;
use crate::core::types::mpt_id::MptId;
use crate::core::types::hash_192::HASH192_SIZE;

/// XRP currency constant (all zeros)
pub const XRP_CURRENCY: Currency = Currency([0u8; 20]);

/// No account constant (all zeros) - used for native XRP
pub const XRP_ACCOUNT: AccountID = AccountID([0u8; 20]);

/// No account constant (account ID 1) - sometimes used in special cases
pub const NO_ACCOUNT: AccountID =
    AccountID([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);

/// Issue represents a currency with an issuer (IOU).
/// This mirrors the C++ Issue struct.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct Issue {
    /// The currency code (20 bytes)
    pub currency: Currency,
    /// The issuer account ID (20 bytes)
    pub account: AccountID,
}

impl Issue {
    /// Create a new Issue
    pub const fn new(currency: Currency, account: AccountID) -> Self {
        Self { currency, account }
    }

    /// Check if this issue is native XRP
    pub fn native(&self) -> bool {
        self.currency.0 == XRP_CURRENCY.0 && self.account.0 == XRP_ACCOUNT.0
    }
}

/// MPTIssue represents a Multi-Purpose Token issuance.
/// This mirrors the C++ MPTIssue class.
/// MptId is a type alias for UInt192 (24 bytes).
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct MPTIssue {
    /// The MPT ID (24 bytes: 4 bytes sequence + 20 bytes account)
    mpt_id: MptId,
}

impl MPTIssue {
    /// Create a new MPTIssue from an MPTID
    pub const fn new(mpt_id: MptId) -> Self {
        Self { mpt_id }
    }

    /// Get the MPTID
    pub const fn mpt_id(&self) -> MptId {
        self.mpt_id
    }

    /// Extract the issuer AccountID from the MPTID (bytes 4..24)
    pub fn issuer(&self) -> AccountID {
        let mut account_bytes = [0u8; 20];
        account_bytes.copy_from_slice(&self.mpt_id.0[4..24]);
        AccountID::from(account_bytes)
    }

    /// Extract the sequence number from the MPTID (bytes 0..4)
    pub fn sequence(&self) -> u32 {
        u32::from_be_bytes([
            self.mpt_id.0[0],
            self.mpt_id.0[1],
            self.mpt_id.0[2],
            self.mpt_id.0[3],
        ])
    }

    /// Construct an MPTIssue from a sequence number and issuer AccountID
    pub fn from(sequence_num: u32, issuer: AccountID) -> Self {
        let mut bytes = [0u8; HASH192_SIZE];

        // Set the sequence number (first 4 bytes)
        bytes[0..4].copy_from_slice(&sequence_num.to_be_bytes());

        // Set the issuer account ID (last 20 bytes)
        bytes[4..HASH192_SIZE].copy_from_slice(&issuer.0);

        MPTIssue::new(MptId { 0: bytes })
    }
}

// ============================================================================
// Helper functions for XRP currency/account detection
// ============================================================================

/// Returns true if the given currency is the native XRP currency (all zeros).
pub const fn is_xrp_currency(currency: Currency) -> bool {
    let mut i = 0;
    while i < 20 {
        if currency.0[i] != 0 {
            return false;
        }
        i += 1;
    }
    true
}

/// Returns true if the given account is the native XRP account (all zeros).
pub const fn is_xrp_account(account: AccountID) -> bool {
    let mut i = 0;
    while i < 20 {
        if account.0[i] != 0 {
            return false;
        }
        i += 1;
    }
    true
}

// ============================================================================
// Convenience function for constructing the XRP Issue
// ============================================================================

/// Returns the Issue representing native XRP.
pub const fn xrp_issue() -> Issue {
    Issue {
        currency: XRP_CURRENCY,
        account: XRP_ACCOUNT,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xrp_issue() {
        let xrp = xrp_issue();
        assert!(xrp.native());
    }

    #[test]
    fn test_iou_issue() {
        let currency = Currency::from(*b"USD");
        let issuer = AccountID::from([1u8; 20]);
        let issue = Issue::new(currency, issuer);

        assert!(!issue.native());
    }

    #[test]
    fn test_mpt_issue() {
        // Create a 24-byte MPTID: [4 bytes sequence][20 bytes account]
        let mut mpt_bytes = [0u8; 24];
        // First 4 bytes are sequence (e.g., 0x00000001)
        mpt_bytes[0] = 0;
        mpt_bytes[1] = 0;
        mpt_bytes[2] = 0;
        mpt_bytes[3] = 1;
        // Next 20 bytes are the account
        let mut i = 0;
        while i < 20 {
            mpt_bytes[4 + i] = 5;
            i += 1;
        }

        let mpt_id = MptId::from(mpt_bytes);
        let mpt_issue = MPTIssue::new(mpt_id);

        assert_eq!(mpt_issue.mpt_id(), mpt_id);

        // Test issuer extraction - should be bytes 4-23 (the 20-byte account)
        let issuer = mpt_issue.issuer();
        let expected_account = [5u8; 20];
        assert_eq!(issuer.0, expected_account);
    }

    #[test]
    fn test_is_xrp_currency() {
        assert!(is_xrp_currency(XRP_CURRENCY));
        assert!(!is_xrp_currency(Currency::from(*b"USD")));
    }

    #[test]
    fn test_is_xrp_account() {
        assert!(is_xrp_account(XRP_ACCOUNT));
        assert!(!is_xrp_account(AccountID::from([1u8; 20])));
    }
}
