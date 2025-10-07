use crate::core::types::account_id::AccountID;
use crate::core::types::amount::currency_code::CurrencyCode;
use crate::core::types::amount::mpt_id::MptId;

/// Struct to represent an Asset of type XRP. Exists so that other structs can restrict type
/// information to XRP in their declarations (this is not possible with just the `Asset` enum below).
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct XrpAsset {}

/// Defines an asset for IOUs.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct IouAsset {
    issuer: AccountID,
    currency_code: CurrencyCode,
    _bytes: [u8; 40],
}

impl IouAsset {
    pub fn new(issuer: AccountID, currency_code: CurrencyCode) -> Self {
        let mut bytes = [0u8; 40];
        bytes[..20].copy_from_slice(currency_code.as_bytes());
        bytes[20..].copy_from_slice(&issuer.0);
        Self {
            issuer,
            currency_code,
            _bytes: bytes,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self._bytes
    }
}

/// Struct to represent an Asset of type MPT. Exists so that other structs can restrict type
/// information to XRP in their declarations (this is not possible with just the `Asset` enum below).
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct MptAsset {
    mpt_id: MptId,
}

/// Represents an asset without a value, such as reading `Asset1` and `Asset2` in AMM ledger
/// objects.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub enum Asset {
    XRP(XrpAsset),
    IOU(IouAsset),
    MPT(MptAsset),
}

impl Asset {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Asset::XRP(_) => {
                static XRP_BUF: [u8; 20] = [0; 20];
                &XRP_BUF
            }
            Asset::IOU(iou) => iou.as_bytes(),
            Asset::MPT(mpt) => mpt.mpt_id.as_bytes(),
        }
    }
}
