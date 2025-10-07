// TODO: Move these to the `types` crate.
pub const XRPL_NFTID_SIZE: usize = 32;
pub const XRPL_CONTRACT_DATA_SIZE: usize = 4096; //TODO size??

pub type NFT = [u8; XRPL_NFTID_SIZE];
pub type ContractData = [u8; XRPL_CONTRACT_DATA_SIZE];

pub const XRPL_NFT_URI_SIZE: usize = 256;
pub type NftUri = [u8; XRPL_NFT_URI_SIZE];
