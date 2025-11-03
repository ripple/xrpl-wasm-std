//! Account identifiers used throughout XRPL.
//!
//! This type wraps a 20-byte AccountID and is returned by many accessors.
//! See also: <https://xrpl.org/docs/references/protocol/common-fields#accountid-fields>

pub const ACCOUNT_ID_SIZE: usize = 20;

/// A 20-byte account identifier on the XRP Ledger.
///
/// AccountIDs are derived from a public key and uniquely identify accounts on the ledger.
/// They are used throughout XRPL for specifying senders, receivers, issuers, and other
/// account-related fields.
///
/// ## Derived Traits
///
/// - `Copy`: Efficient for this small 20-byte struct, enabling implicit copying
/// - `Clone`: Automatically derived with Copy for consistency
/// - `PartialEq, Eq`: Enable account comparisons and use in hash-based collections
/// - `Debug`: Useful for development and debugging
///
/// The `Copy` trait is appropriate here because:
/// - The struct is only 20 bytes, making copies very cheap
/// - AccountIDs are frequently passed around in XRPL operations
/// - Implicit copying improves ergonomics without performance concerns
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct AccountID(pub [u8; ACCOUNT_ID_SIZE]);

impl From<[u8; ACCOUNT_ID_SIZE]> for AccountID {
    fn from(value: [u8; ACCOUNT_ID_SIZE]) -> Self {
        AccountID(value)
    }
}
