use ink::storage::Mapping;
use openbrush::{
    contracts::{
        access_control::*,
        ownable::*,
        psp34::*,
        reentrancy_guard::*,
    },
    modifiers,
    traits::{AccountId, Balance, Storage, String},
};
use rmrk::storage::catalog_external::Catalog;

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
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
    PSP34Error(PSP34Error),
    ReentrancyGuardError(ReentrancyGuardError),
    RmrkError(rmrk::errors::Error),
    NoParentId,
    InvalidExecutionNFT,
}