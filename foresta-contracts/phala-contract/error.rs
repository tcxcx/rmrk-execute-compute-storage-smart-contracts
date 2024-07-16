pub use scale::{Decode, Encode};

#[derive(Encode, Decode, Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PhalaError {
    EcdhInvalidSecretKey,
    EcdhInvalidPublicKey,
    AESCannotEncrypt,
    AESCannotDecrypt,
    InvalidAddress,
    BalanceOverflow,
    FetchDataFailed,
    FailedToGetBlockNumber,
    RequestFailed,
    Test,
    NoPermission,
    CidMissingFordNftId,
    BadTimestamp,
    NotNftOwner,
    DownloadError,
    DecryptionError,
    DatabaseDepositError,
    CrossContractCallFailed,
    InvalidAlgoId,
    DatabaseError,
    NotExecuteNftOwner,
    InvalidContractAbi,
    SignatureRecoveryFailed,
}