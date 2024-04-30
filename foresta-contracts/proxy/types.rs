#![cfg_attr(not(feature = "std"), no_std)]

use ink::storage::Mapping;
use openbrush::{
    contracts::{
        access_control::AccessControlError, ownable::OwnableError, psp34::PSP34Error,
        reentrancy_guard::ReentrancyGuardError,
    },
    traits::{AccountId, Balance},
};
use rmrk::errors::Error as RmrkError;

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::storage(STORAGE_KEY)]
pub struct Data {
    pub rmrk_contract: Option<AccountId>,
    pub catalog_contract: Option<AccountId>,
    pub schrodinger_contract: Option<AccountId>,
    pub mint_price: Balance,
    pub salt: u64,
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ProxyError {
    AccessControlError(AccessControlError),
    IPFSError,
    OwnableError(OwnableError),
    SchrodingerError,
    MintingError,
    PSP34Error(PSP34Error),
    ReentrancyError(ReentrancyGuardError),
    RmrkError(RmrkError),
    NoParentId,
    InvalidExecutionNFT,
}

// AlgorithmNFT and ExecutionNFT types would be defined here
pub type AlgorithmNFT = psp34::Data<psp34::app::Data>;
pub type ExecutionNFT = psp34::Data<psp34::app::Data>;
pub type Id = psp34::Id;