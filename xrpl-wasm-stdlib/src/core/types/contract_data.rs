pub const XRPL_CONTRACT_DATA_SIZE: usize = 4096; //TODO size??

pub struct ContractData {
    pub data: [u8; XRPL_CONTRACT_DATA_SIZE],

    /// The actual length of this Fulfillment, if less than data.len()
    pub len: usize,
}
