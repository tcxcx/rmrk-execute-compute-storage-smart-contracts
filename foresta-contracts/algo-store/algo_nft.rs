#![cfg_attr(not(feature = "std"), no_std)]

use crate::error::AlgoExecuteError;
use ink::storage::Mapping;
use openbrush::{
    contracts::ownable::*,
    traits::Storage,
};
use proxy::rmrk_proxy::Id;

#[openbrush::contract]
pub mod algorithm_nft {
    use super::*;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct AlgorithmNFT {
        #[storage_field]
        ownable: ownable::Data,
        metadata: Mapping<Id, String>,
        algorithm_cid: Mapping<Id, String>,
    }

    impl AlgorithmNFT {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                ownable: Default::default(),
                metadata: Default::default(),
                algorithm_cid: Default::default(),
            }
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

        fn ensure_owner(&mut self) -> Result<(), AlgoExecuteError> {
            let owner = self.ownable.owner.get().ok_or(AlgoExecuteError::NotAuthorized)?;
            if Some(owner) == Some(Some(self.env().caller())) {
                Ok(())
            } else {
                Err(AlgoExecuteError::NotAuthorized)
            }
        }
    }
}