#![cfg_attr(not(feature = "std"), no_std)]

use openbrush::{
    contracts::psp34::{
        Data as PSP34Data,
        Id as PSP34Id,
    },
    traits::{
        AccountId,
        Balance,
    },
};
use scale::{
    Decode,
    Encode,
};

#[derive(Encode, Decode, Default, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Data {
    pub rmrk_contract: Option<AccountId>,
    pub catalog_contract: Option<AccountId>,
    pub schrodinger_contract: Option<AccountId>,
    pub mint_price: Balance,
    pub salt: u64,
}

pub type AlgorithmNFT = PSP34Data;
pub type ExecutionNFT = PSP34Data;
pub type Id = PSP34Id;
