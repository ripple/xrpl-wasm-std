use crate::core::types::account_id::AccountID;
use crate::core::types::contract_data::XRPL_CONTRACT_DATA_SIZE;
use crate::host;
use crate::types::NFT;
use host::{Error, Result, Result::Ok};

// TODO: Add documentation and examples.

// TODO: Define an Nft struct
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
