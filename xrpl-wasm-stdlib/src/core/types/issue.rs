use crate::core::types::account_id::AccountID;
use crate::core::types::currency::Currency;
use crate::core::types::mpt_id::MptId;

/// Struct to represent an Issue of type XRP. Exists so that other structs can restrict type
/// information to XRP in their declarations (this is not possible with just the `Issue` enum below).
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct XrpIssue {}

/// Defines an issue for IOUs.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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
/// information to XRP in their declarations (this is not possible with just the `Issue` enum below).
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct MptIssue {
    mpt_id: MptId,
}

impl MptIssue {
    pub fn new(mpt_id: MptId) -> Self {
        Self { mpt_id }
    }

    pub fn mpt_id(&self) -> MptId {
        self.mpt_id
    }
}

/// Represents an issue without a value, such as reading `Asset1` and `Asset2` in AMM ledger
/// objects.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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
