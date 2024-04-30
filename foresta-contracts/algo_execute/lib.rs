#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

pub use proxy::*;
pub use types::*;
pub use catalog::*;

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

// AS-NFT
#[derive(Default, Storage)]
pub struct AlgorithmNFT {
    #[storage_field]
    access: access_control::Data,
    #[storage_field]
    ownable: ownable::Data,
    #[storage_field]
    psp34: psp34::Data<psp34::app::Data>,
    #[storage_field]
    metadata: Mapping<Id, String>,
    #[storage_field]
    algorithm_cid: Mapping<Id, String>,
}

impl AlgorithmNFT {
    #[modifiers(only_owner)]
    pub fn set_metadata(&mut self, token_id: Id, metadata: String) -> Result<(), PSP34Error> {
        self.metadata.insert(&token_id, &metadata);
        Ok(())
    }

    #[modifiers(only_owner)]
    pub fn set_algorithm_cid(
        &mut self,
        token_id: Id,
        cid: String,
    ) -> Result<(), PSP34Error> {
        self.algorithm_cid.insert(&token_id, &cid);
        Ok(())
    }
}

// EA-NFT
#[derive(Default, Storage)]
pub struct ExecutionNFT {
    #[storage_field]
    psp34: psp34::Data<psp34::app::Data>,
    #[storage_field]
    parent_id: Mapping<Id, Id>,
}

impl ExecutionNFT {
    pub fn set_parent_id(&mut self, token_id: Id, parent_id: Id) -> Result<(), PSP34Error> {
        self.parent_id.insert(&token_id, &parent_id);
        Ok(())
    }

    pub fn get_parent_id(&self, token_id: Id) -> Option<Id> {
        self.parent_id.get(&token_id)
    }
}