/// The type of any given XRPL transaction
#[repr(i16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionType {
    Invalid = -1,
    Payment = 0,
    EscrowCreate = 1,
    EscrowFinish = 2,
    AccountSet = 3,
    EscrowCancel = 4,
    SetRegularKey = 5,
    NickNameSet = 6,
    OfferCreate = 7,
    OfferCancel = 8,
    Contract = 9,
    TicketCreate = 10,
    TicketCancel = 11,
    SignerListSet = 12,
    PaymentChannelCreate = 13,
    PaymentChannelFund = 14,
    PaymentChannelClaim = 15,
    CheckCreate = 16,
    CheckCash = 17,
    CheckCancel = 18,
    DepositPreauth = 19,
    TrustSet = 20,
    AccountDelete = 21,
    SetHook = 22,
    NFTokenMint = 25,
    NFTokenBurn = 26,
    NFTokenCreateOffer = 27,
    NFTokenCancelOffer = 28,
    NFTokenAcceptOffer = 29,
    Clawback = 30,
    AMMCreate = 35,
    AMMDeposit = 36,
    AMMWithdraw = 37,
    AMMVote = 38,
    AMMBid = 39,
    AMMDelete = 40,
    XChainCreateClaimID = 41,
    XChainCommit = 42,
    XChainClaim = 43,
    XChainAccountCreateCommit = 44,
    XChainAddClaimAttestation = 45,
    XChainAddAccountCreateAttestation = 46,
    XChainModifyBridge = 47,
    XChainCreateBridge = 48,
    DIDSet = 49,
    DIDDelete = 50,
    OracleSet = 51,
    OracleDelete = 52,
    EnableAmendment = 100,
    SetFee = 101,
    UNLModify = 102,
}

impl From<[u8; 2]> for TransactionType {
    fn from(value: [u8; 2]) -> Self {
        let value_16 = i16::from_le_bytes(value.as_slice().try_into().expect("Incorrect length"));
        value_16.into()
    }
}

impl From<i16> for TransactionType {
    fn from(value: i16) -> Self {
        match value {
            // List every single variant and its corresponding i16 value
            -1 => TransactionType::Invalid,
            0 => TransactionType::Payment,
            1 => TransactionType::EscrowCreate,
            2 => TransactionType::EscrowFinish,
            3 => TransactionType::AccountSet,
            4 => TransactionType::EscrowCancel,
            5 => TransactionType::SetRegularKey,
            6 => TransactionType::NickNameSet,
            7 => TransactionType::OfferCreate,
            8 => TransactionType::OfferCancel,
            9 => TransactionType::Contract,
            10 => TransactionType::TicketCreate,
            11 => TransactionType::TicketCancel,
            12 => TransactionType::SignerListSet,
            13 => TransactionType::PaymentChannelCreate,
            14 => TransactionType::PaymentChannelFund,
            15 => TransactionType::PaymentChannelClaim,
            16 => TransactionType::CheckCreate,
            17 => TransactionType::CheckCash,
            18 => TransactionType::CheckCancel,
            19 => TransactionType::DepositPreauth,
            20 => TransactionType::TrustSet,
            21 => TransactionType::AccountDelete,
            22 => TransactionType::SetHook,
            25 => TransactionType::NFTokenMint,
            26 => TransactionType::NFTokenBurn,
            27 => TransactionType::NFTokenCreateOffer,
            28 => TransactionType::NFTokenCancelOffer,
            29 => TransactionType::NFTokenAcceptOffer,
            30 => TransactionType::Clawback,
            35 => TransactionType::AMMCreate,
            36 => TransactionType::AMMDeposit,
            37 => TransactionType::AMMWithdraw,
            38 => TransactionType::AMMVote,
            39 => TransactionType::AMMBid,
            40 => TransactionType::AMMDelete,
            41 => TransactionType::XChainCreateClaimID,
            42 => TransactionType::XChainCommit,
            43 => TransactionType::XChainClaim,
            44 => TransactionType::XChainAccountCreateCommit,
            45 => TransactionType::XChainAddClaimAttestation,
            46 => TransactionType::XChainAddAccountCreateAttestation,
            47 => TransactionType::XChainModifyBridge,
            48 => TransactionType::XChainCreateBridge,
            49 => TransactionType::DIDSet,
            50 => TransactionType::DIDDelete,
            51 => TransactionType::OracleSet,
            52 => TransactionType::OracleDelete,
            100 => TransactionType::EnableAmendment,
            101 => TransactionType::SetFee,
            102 => TransactionType::UNLModify,

            // If the value doesn't match any known variant, return an error
            _ => TransactionType::Invalid,
        }
    }
}

impl From<TransactionType> for [u8; 2] {
    fn from(value: TransactionType) -> Self {
        // 1. Cast the enum variant `self` to its underlying i16 value.
        let value_i16: i16 = value as i16;

        // 2. Convert the i16 value into a fixed-size byte array ([u8; 2]).
        let bytes_array: [u8; 2] = value_i16.to_le_bytes();

        bytes_array
    }
}
