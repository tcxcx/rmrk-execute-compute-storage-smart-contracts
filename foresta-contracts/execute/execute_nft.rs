#![cfg_attr(not(feature = "std"), no_std)]

use ink::storage::Mapping;
use proxy::rmrk_proxy::Id;
use crate::error::AlgoExecuteError;


#[openbrush::contract]
pub mod execution_nft {
    use super::*;

    #[derive(Default)]
    #[ink(storage)]
    pub struct ExecutionNFT {
        parent_id: Mapping<Id, Id>,
    }

    impl ExecutionNFT {
        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                parent_id: Default::default(),
            }
        }

        #[ink(message)]
        pub fn set_parent_id(&mut self, token_id: Id, parent_id: Id) -> Result<(), AlgoExecuteError> {
            self.parent_id.insert(&token_id, &parent_id);
            Ok(())
        }

        #[ink(message)]
        pub fn get_parent_id(&self, token_id: Id) -> Option<Id> {
            self.parent_id.get(&token_id)
        }
    }
}