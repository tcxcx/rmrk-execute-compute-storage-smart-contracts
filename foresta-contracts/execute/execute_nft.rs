// Handles the execution permissions associated with NFTs. This is critical for ensuring that only authorized users can execute the 
// algorithms associated with specific NFTs.

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
        
        #[ink(message)]
        pub fn is_valid_execution(&self, token_id: Id, parent_id: Id) -> bool {
            match self.get_parent_id(token_id) {
                Some(id) => id == parent_id,
                None => false,
            }
        }
        
        pub fn verify_execution_rights(&self, exec_id: Id, algo_id: Id) -> Result<(), AlgoExecuteError> {
            let parent_id = self.parent_id.get(&exec_id).ok_or(AlgoExecuteError::InvalidExecutionNFT)?;
            if parent_id != algo_id {
                Err(AlgoExecuteError::UnauthorizedAccess)
            } else {
                Ok(())
            }
        }
    }
}