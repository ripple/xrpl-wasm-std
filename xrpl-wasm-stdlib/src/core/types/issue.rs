use crate::core::types::account_id::AccountID;
use crate::core::types::currency::Currency;
use crate::core::types::mpt_id::MptId;
use crate::host::Result;

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

    /// Creates an Issue from a buffer and length, detecting the type based on the byte count.
    ///
    /// # Arguments
    ///
    /// * `buffer` - A 40-byte buffer containing the issue data
    /// * `len` - The actual number of bytes written to the buffer
    ///
    /// # Returns
    ///
    /// Returns `Result<Issue>` where:
    /// * `Ok(Issue::XRP(...))` - If len is 20 (XRP issue)
    /// * `Ok(Issue::MPT(...))` - If len is 24 (MPT issue)
    /// * `Ok(Issue::IOU(...))` - If len is 40 (IOU issue)
    /// * `Err(Error)` - If len is not one of the expected values
    #[inline]
    pub fn from_buffer(buffer: [u8; 40], len: usize) -> Result<Self> {
        match len {
            20 => Result::Ok(Issue::XRP(XrpIssue {})),
            24 => {
                let mpt_bytes: [u8; 24] = buffer[..24].try_into().unwrap_or([0u8; 24]);
                let mpt_id = MptId::from(mpt_bytes);
                Result::Ok(Issue::MPT(MptIssue::new(mpt_id)))
            }
            40 => {
                let currency_bytes: [u8; 20] = buffer[..20].try_into().unwrap_or([0u8; 20]);
                let issuer_bytes: [u8; 20] = buffer[20..40].try_into().unwrap_or([0u8; 20]);
                let currency = Currency::from(currency_bytes);
                let issuer = AccountID::from(issuer_bytes);
                Result::Ok(Issue::IOU(IouIssue::new(issuer, currency)))
            }
            _ => Result::Err(crate::host::Error::from_code(len as i32)),
        }
    }
}
