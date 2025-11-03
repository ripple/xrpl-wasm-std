use crate::core::types::account_id::AccountID;
use crate::core::types::currency::Currency;
use crate::core::types::mpt_id::MptId;

/// Struct to represent an Issue of type XRP. Exists so that other structs can restrict type
/// information to XRP in their declarations (this is not possible with just the `Issue` enum below).
///
/// ## Derived Traits
///
/// - `Debug`: Useful for development and debugging
/// - `Clone`: Automatically derived with Copy for consistency
/// - `Copy`: Efficient for this zero-sized struct
/// - `PartialEq, Eq`: Enable comparisons
///
/// The `Copy` trait is appropriate here because this is a zero-sized type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct XrpIssue {}

/// Defines an issue for IOUs (40 bytes: 20-byte currency + 20-byte issuer).
///
/// ## Derived Traits
///
/// - `Debug`: Useful for development and debugging
/// - `Clone`: Reasonable for this 40-byte struct when explicit copying is needed
/// - `PartialEq, Eq`: Enable issue comparisons and use in collections
///
/// Note: `Copy` is intentionally not derived due to the struct's size (40 bytes).
/// Large `Copy` types can lead to accidental expensive copies and poor performance.
/// Use `.clone()` when you need to duplicate an IOU issue.
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct IouIssue {
    issuer: AccountID,
    currency: Currency,
    _bytes: [u8; 40],
}

impl IouIssue {
    pub fn new(issuer: AccountID, currency: Currency) -> Self {
        let mut bytes = [0u8; 40];
        bytes[..20].copy_from_slice(currency.as_bytes());
        bytes[20..].copy_from_slice(&issuer.0);
        Self {
            issuer,
            currency,
            _bytes: bytes,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self._bytes
    }
}

/// Struct to represent an Issue of type MPT. Exists so that other structs can restrict type
/// information to MPT in their declarations (this is not possible with just the `Issue` enum below).
///
/// ## Derived Traits
///
/// - `Debug`: Useful for development and debugging
/// - `Clone`: Automatically derived with Copy for consistency
/// - `Copy`: Efficient for this 24-byte struct
/// - `PartialEq, Eq`: Enable MPT issue comparisons
///
/// The `Copy` trait is appropriate here because:
/// - The struct is only 24 bytes (wrapping MptId)
/// - MPT issues are frequently used in token operations
/// - Implicit copying improves ergonomics
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct MptIssue {
    mpt_id: MptId,
}

/// Represents an issue without a value, such as reading `Asset1` and `Asset2` in AMM ledger
/// objects.
///
/// ## Derived Traits
///
/// - `Debug`: Useful for development and debugging
/// - `Clone`: Reasonable for this enum when explicit copying is needed
/// - `PartialEq, Eq`: Enable issue comparisons and use in collections
///
/// Note: `Copy` is intentionally not derived because the `IOU` variant is 40 bytes.
/// Large `Copy` types can lead to accidental expensive copies and poor performance.
/// Use `.clone()` when you need to duplicate an issue.
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum Issue {
    XRP(XrpIssue),
    IOU(IouIssue),
    MPT(MptIssue),
}

impl Issue {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Issue::XRP(_) => {
                static XRP_BUF: [u8; 20] = [0; 20];
                &XRP_BUF
            }
            Issue::IOU(iou) => iou.as_bytes(),
            Issue::MPT(mpt) => mpt.mpt_id.as_bytes(),
        }
    }
}
