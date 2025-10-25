//! NFToken (Non-Fungible Token) type for XRPL.
//!
//! Provides a high-level interface for working with NFTokens on the XRP Ledger.
//!
//! ## NFTokenID Structure
//!
//! An NFTokenID is a 32-byte identifier with the following structure:
//!
//! ```text
//! 000B 0539 C35B55AA096BA6D87A6E6C965A6534150DC56E5E 12C5D09E 0000000C
//! +--- +--- +--------------------------------------- +------- +-------
//! |    |    |                                        |        |
//! |    |    |                                        |        └─> Sequence (32 bits)
//! |    |    |                                        └─> Scrambled Taxon (32 bits)
//! |    |    └─> Issuer Address (160 bits / 20 bytes)
//! |    └─> Transfer Fee (16 bits)
//! └─> Flags (16 bits)
//! ```

use crate::core::types::account_id::{AccountID, ACCOUNT_ID_SIZE};
use crate::core::types::blob::Blob;
use crate::core::types::contract_data::XRPL_CONTRACT_DATA_SIZE;
use crate::host;
use crate::host::{Error, Result};

/// Size of an NFTokenID in bytes (256 bits)
pub const NFTID_SIZE: usize = 32;

/// Maximum size for NFT URI data (256 bytes)
pub const NFT_URI_MAX_SIZE: usize = 256;

/// NFToken flags - see [NFToken documentation](https://xrpl.org/docs/references/protocol/data-types/nftoken)
pub mod flags {
    /// The issuer (or an entity authorized by the issuer) may destroy the object.
    /// If this flag is set, the object may be burned by the issuer even if the issuer
    /// does not currently hold the object. The object's owner can always burn it.
    pub const BURNABLE: u16 = 0x0001;

    /// If set, indicates that the minted token may only be bought or sold for XRP.
    /// This can be useful for compliance purposes if the issuer wants to avoid
    /// other tokens.
    pub const ONLY_XRP: u16 = 0x0002;

    /// If set, automatically create trust lines to hold transfer fees as specified
    /// in the TransferFee field.
    pub const TRUST_LINE: u16 = 0x0004;

    /// If set, indicates that the minted token may be transferred to others.
    /// If not set, the token can only be transferred back to the issuer.
    pub const TRANSFERABLE: u16 = 0x0008;
}

/// Represents an NFToken (Non-Fungible Token) on the XRP Ledger.
///
/// The `NFToken` type wraps a 32-byte NFTokenID and provides methods to extract
/// all fields encoded within the identifier, as well as retrieve associated
/// metadata like the NFT's URI.
///
/// # NFTokenID Encoding
///
/// The 32-byte identifier contains:
/// - **Bytes 0-1**: Flags (16 bits, big-endian)
/// - **Bytes 2-3**: Transfer fee (16 bits, big-endian, in 1/100,000 units)
/// - **Bytes 4-23**: Issuer account address (160 bits)
/// - **Bytes 24-27**: Scrambled taxon (32 bits, big-endian)
/// - **Bytes 28-31**: Sequence number (32 bits, big-endian)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct NFToken(pub [u8; NFTID_SIZE]);

impl NFToken {
    /// Creates a new NFToken from a 32-byte identifier.
    ///
    /// # Arguments
    ///
    /// * `id` - The 32-byte NFTokenID
    ///
    #[inline]
    pub const fn new(id: [u8; NFTID_SIZE]) -> Self {
        NFToken(id)
    }

    /// Returns the raw NFTokenID as a byte array.
    ///
    #[inline]
    pub const fn as_bytes(&self) -> &[u8; NFTID_SIZE] {
        &self.0
    }

    /// Returns a pointer to the NFTokenID data.
    ///
    /// This is primarily used internally for FFI calls to host functions.
    #[inline]
    pub const fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }

    /// Returns the length of the NFTokenID (always 32 bytes).
    #[inline]
    #[allow(clippy::len_without_is_empty)]
    pub const fn len(&self) -> usize {
        NFTID_SIZE
    }

    /// Retrieves the flags associated with this NFToken.
    ///
    /// Flags are stored in the first 2 bytes of the NFTokenID (big-endian).
    /// Use the constants in the [`flags`] module to check for specific flags.
    ///
    /// # Returns
    ///
    /// * `Ok(u16)` - The flags bitmask
    /// * `Err(Error)` - If the host function fails
    ///
    pub fn flags(&self) -> Result<u16> {
        let result = unsafe { host::get_nft_flags(self.as_ptr(), self.len()) };

        match result {
            code if code >= 0 => Result::Ok(code as u16),
            code => Result::Err(Error::from_code(code)),
        }
    }

    /// Checks if the NFToken has the `BURNABLE` flag set.
    ///
    pub fn is_burnable(&self) -> Result<bool> {
        match self.flags() {
            Result::Ok(flags) => Result::Ok(flags & flags::BURNABLE != 0),
            Result::Err(e) => Result::Err(e),
        }
    }

    /// Checks if the NFToken has the `ONLY_XRP` flag set.
    ///
    pub fn is_only_xrp(&self) -> Result<bool> {
        match self.flags() {
            Result::Ok(flags) => Result::Ok(flags & flags::ONLY_XRP != 0),
            Result::Err(e) => Result::Err(e),
        }
    }

    /// Checks if the NFToken has the `TRANSFERABLE` flag set.
    ///
    pub fn is_transferable(&self) -> Result<bool> {
        match self.flags() {
            Result::Ok(flags) => Result::Ok(flags & flags::TRANSFERABLE != 0),
            Result::Err(e) => Result::Err(e),
        }
    }

    /// Retrieves the transfer fee for this NFToken.
    ///
    /// The transfer fee is expressed in 1/100,000 units, meaning:
    /// - A value of 1 represents 0.001% (1/10 of a basis point)
    /// - A value of 100 represents 0.1% (10 basis points)
    /// - A value of 1000 represents 1% (100 basis points)
    /// - Maximum allowed value is 50,000 (representing 50%)
    ///
    /// # Returns
    ///
    /// * `Ok(u16)` - The transfer fee (0-50,000)
    /// * `Err(Error)` - If the host function fails
    ///
    pub fn transfer_fee(&self) -> Result<u16> {
        let result = unsafe { host::get_nft_transfer_fee(self.as_ptr(), self.len()) };

        match result {
            code if code >= 0 => Result::Ok(code as u16),
            code => Result::Err(Error::from_code(code)),
        }
    }

    /// Calculates the transfer fee as a percentage.
    ///
    /// This is a convenience method that converts the raw transfer fee value
    /// into a human-readable percentage.
    ///
    /// # Returns
    ///
    /// * `Ok(f64)` - The transfer fee as a percentage (0.0 to 50.0)
    /// * `Err(Error)` - If the host function fails
    ///
    #[cfg(not(target_arch = "wasm32"))]
    pub fn transfer_fee_percentage(&self) -> Result<f64> {
        match self.transfer_fee() {
            Result::Ok(fee) => Result::Ok((fee as f64) / 1000.0),
            Result::Err(e) => Result::Err(e),
        }
    }

    /// Retrieves the issuer account of this NFToken.
    ///
    /// The issuer is encoded in bytes 4-23 of the NFTokenID (160 bits / 20 bytes).
    ///
    /// # Returns
    ///
    /// * `Ok(AccountID)` - The issuer's account identifier
    /// * `Err(Error)` - If the host function fails
    ///
    pub fn issuer(&self) -> Result<AccountID> {
        let mut account_buf = [0u8; ACCOUNT_ID_SIZE];
        let result = unsafe {
            host::get_nft_issuer(
                self.as_ptr(),
                self.len(),
                account_buf.as_mut_ptr(),
                account_buf.len(),
            )
        };

        match result {
            code if code > 0 => Result::Ok(AccountID(account_buf)),
            code => Result::Err(Error::from_code(code)),
        }
    }

    /// Retrieves the taxon of this NFToken.
    ///
    /// The taxon is an issuer-defined value that groups related NFTs together.
    /// # Returns
    ///
    /// * `Ok(u32)` - The taxon value
    /// * `Err(Error)` - If the host function fails
    ///
    pub fn taxon(&self) -> Result<u32> {
        let mut taxon_buf = [0u8; 4];
        let result = unsafe {
            host::get_nft_taxon(
                self.as_ptr(),
                self.len(),
                taxon_buf.as_mut_ptr(),
                taxon_buf.len(),
            )
        };

        match result {
            code if code > 0 => {
                // Convert big-endian bytes to u32
                let taxon = u32::from_be_bytes(taxon_buf);
                Result::Ok(taxon)
            }
            code => Result::Err(Error::from_code(code)),
        }
    }

    /// Retrieves the serial/sequence number of this NFToken.
    ///
    /// The sequence number is automatically incremented for each NFToken minted
    /// by the issuer, based on the `MintedNFTokens` field of the issuer's account.
    /// This ensures each NFToken has a unique identifier.
    ///
    /// # Returns
    ///
    /// * `Ok(u32)` - The sequence number
    /// * `Err(Error)` - If the host function fails
    ///
    pub fn serial(&self) -> Result<u32> {
        let mut serial_buf = [0u8; 4];
        let result = unsafe {
            host::get_nft_serial(
                self.as_ptr(),
                self.len(),
                serial_buf.as_mut_ptr(),
                serial_buf.len(),
            )
        };

        match result {
            code if code > 0 => {
                // Convert big-endian bytes to u32
                let serial = u32::from_be_bytes(serial_buf);
                Result::Ok(serial)
            }
            code => Result::Err(Error::from_code(code)),
        }
    }

    /// Retrieves the URI of this NFToken for a given owner.
    /// # Arguments
    ///
    /// * `owner` - The account that owns this NFToken
    ///
    /// # Returns
    ///
    /// * `Ok(Blob)` - The URI data (variable length, up to 256 bytes)
    /// * `Err(Error)` - If the NFT is not found or the host function fails
    ///
    pub fn uri(&self, owner: &AccountID) -> Result<Blob> {
        let mut uri_buf = [0u8; NFT_URI_MAX_SIZE];
        let result = unsafe {
            host::get_nft(
                owner.0.as_ptr(),
                owner.0.len(),
                self.as_ptr(),
                self.len(),
                uri_buf.as_mut_ptr(),
                uri_buf.len(),
            )
        };

        match result {
            code if code > 0 => {
                let actual_len = code as usize;
                // Create a Blob with a properly sized buffer (1024 bytes)
                let mut blob_data = [0u8; 1024];
                let copy_len = actual_len.min(uri_buf.len()).min(blob_data.len());
                blob_data[..copy_len].copy_from_slice(&uri_buf[..copy_len]);
                Result::Ok(Blob::new(blob_data, copy_len))
            }
            code => Result::Err(Error::from_code(code)),
        }
    }

    /// Checks if the specified owner owns this NFToken.
    ///
    /// This is an optimized ownership check that doesn't retrieve the full URI data.
    ///
    /// # Arguments
    ///
    /// * `owner` - The account to check for ownership
    ///
    /// # Returns
    ///
    /// * `Ok(true)` - The owner possesses this NFToken
    /// * `Ok(false)` - The owner does not possess this NFToken (NFT not found)
    /// * `Err(Error)` - If the host function fails with an error other than "not found"
    ///
    pub fn is_owned_by(&self, owner: &AccountID) -> Result<bool> {
        let mut data = [0u8; XRPL_CONTRACT_DATA_SIZE];
        let result_code = unsafe {
            host::get_nft(
                owner.0.as_ptr(),
                owner.0.len(),
                self.as_ptr(),
                self.len(),
                data.as_mut_ptr(),
                data.len(),
            )
        };

        match result_code {
            code if code > 0 => Result::Ok(true),
            code => Result::Err(Error::from_code(code)),
        }
    }
}

impl From<[u8; NFTID_SIZE]> for NFToken {
    fn from(value: [u8; NFTID_SIZE]) -> Self {
        NFToken(value)
    }
}

impl AsRef<[u8]> for NFToken {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nft_creation() {
        let nft_id = [0u8; 32];
        let nft = NFToken::new(nft_id);
        assert_eq!(nft.as_bytes(), &nft_id);
        assert_eq!(nft.len(), 32);
    }

    #[test]
    fn test_nft_from_array() {
        let nft_id = [0u8; 32];
        let nft: NFToken = nft_id.into();
        assert_eq!(nft.as_bytes(), &nft_id);
    }
}
