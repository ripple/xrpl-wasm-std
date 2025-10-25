//! NFT (Non-Fungible Token) ledger object access.
//!
//! This module provides functions to interact with NFTokens on the XRP Ledger.
//!
//! See [`NFToken`](crate::core::types::nft::NFToken) for detailed documentation.

use crate::core::types::account_id::AccountID;
use crate::core::types::contract_data::XRPL_CONTRACT_DATA_SIZE;
use crate::core::types::nft::NFToken;
use crate::host;
use crate::types::NFT;
use host::{Error, Result, Result::Ok};

/// Retrieves the NFT data for the given owner and NFT ID.
///
/// Returns the raw NFT URI data in a 4096-byte buffer. This also serves as
/// an ownership check - it only succeeds if the owner possesses the NFT.
pub fn get_nft(owner: &AccountID, nft: &NFT) -> Result<[u8; XRPL_CONTRACT_DATA_SIZE]> {
    let mut data = [0u8; XRPL_CONTRACT_DATA_SIZE];
    let result_code = unsafe {
        host::get_nft(
            owner.0.as_ptr(),
            owner.0.len(),
            nft.as_ptr(),
            nft.len(),
            data.as_mut_ptr(),
            data.len(),
        )
    };

    match result_code {
        code if code > 0 => Ok(data),
        code => Result::Err(Error::from_code(code)),
    }
}

/// Checks if the specified account owns the given NFToken.
///
/// Returns `true` if the account owns the NFT, `false` otherwise.
pub fn is_nft_owned_by(owner: &AccountID, nft_id: &NFT) -> bool {
    get_nft(owner, nft_id).is_ok()
}

/// Creates an NFToken wrapper with typed access to all NFT fields.
///
/// Use this to access NFT properties like flags, transfer fee, issuer, taxon, and serial.
#[inline]
pub const fn nft_token(nft_id: [u8; 32]) -> NFToken {
    NFToken::new(nft_id)
}
