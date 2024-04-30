#![cfg_attr(not(feature = "std"), no_std)]

use scale::{Decode, Encode};

#[derive(Encode, Decode, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ProxyError {
    NotAuthorized,
    NotFound,
    UnexpectedError,
    OwnableError,
    PSP34Error,
    ReentrancyGuardError,
    SchrodingerError,
    NoParentId,
    NoMintError,
    InvalidExecutionNFT,
    BadMintValue,
    IPFSError,
}
