// This contract manages metadata and content identifiers (CIDs) related to algorithms stored as NFTs. 
// It includes ownership checks, which is crucial for managing access rights.
#![cfg_attr(not(feature = "std"), no_std, no_main)]

use crate::error::AlgoExecuteError;
use ink::storage::Mapping;
use openbrush::{
    contracts::{
        ownable::*,
        psp34::Id,
    },
    traits::Storage,
};

use openbrush::traits::String;

#[openbrush::implementation(PSP34)]
#[openbrush::contract]
pub mod algorithm_nft {
    use super::*;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct AlgorithmNFT {
        #[storage_field]
        psp34: psp34::Data,
        ownable: ownable::Data,
        metadata: Mapping<Id, String>,
        algorithm_cid: Mapping<Id, String>,
        token_id: u64,
    }

    impl AlgorithmNFT {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                psp34: Default::default(),
                ownable: Default::default(),
                metadata: Default::default(),
                algorithm_cid: Default::default(),
                token_id: Default::default(),
            }
        }

        #[ink(message)]
        pub fn mint(&mut self,to: AccountId, algorithm_cid: String) -> Result<(), AlgoExecuteError> {
            let mint_id = self.token_id.saturating_add(1);
            self.token_id = mint_id;
            self.algorithm_cid.insert(Id::U64(mint_id),&algorithm_cid);
            let _ = psp34::Internal::_mint_to(self, to, Id::U64(mint_id));
            Ok(())
        }

        #[ink(message)]
        pub fn set_metadata(
            &mut self,
            token_id: Id,
            metadata: String,
        ) -> Result<(), AlgoExecuteError> {
            self.ensure_owner()?;
            self.metadata.insert(&token_id, &metadata);
            Ok(())
        }

        #[ink(message)]
        pub fn set_algorithm_cid(
            &mut self,
            token_id: Id,
            cid: String,
        ) -> Result<(), AlgoExecuteError> {
            self.ensure_owner()?;
            self.algorithm_cid.insert(&token_id, &cid);
            Ok(())
        }
        
        #[ink(message)]
        pub fn get_metadata(&self, token_id: Id) -> Option<String> {
            self.metadata.get(&token_id)
        }

        #[ink(message)]
        pub fn get_algorithm_cid(&self, token_id: Id) -> Option<String> {
            self.algorithm_cid.get(&token_id)
        }

        fn ensure_owner(&mut self) -> Result<(), AlgoExecuteError> {
            let owner = self.ownable.owner.get().ok_or(AlgoExecuteError::NotAuthorized)?;
            if Some(owner) == Some(Some(self.env().caller())) {
                Ok(())
            } else {
                Err(AlgoExecuteError::NotAuthorized)
            }
        }
        
        pub fn fetch_algorithm_data(&self, algorithm_id: Id) -> Result<String, AlgoExecuteError> {
            self.algorithm_cid.get(&algorithm_id).ok_or(AlgoExecuteError::DataNotFound)
        }
    }
}