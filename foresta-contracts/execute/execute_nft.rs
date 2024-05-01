// Handles the execution permissions associated with NFTs. This is critical for ensuring that only authorized users can execute the
// algorithms associated with specific NFTs.
use crate::error::AlgoExecuteError;
use ink::storage::Mapping;
use openbrush::{
    contracts::{
        ownable::*,
        psp34::Id,
    },
    traits::Storage,
};

#[openbrush::implementation(PSP34)]
#[openbrush::contract]
pub mod execute_nft {
    use super::*;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct ExecutionNFT {
        #[storage_field]
        psp34: psp34::Data,
        ownable: ownable::Data,
        parent_id: Mapping<Id, Id>,
        token_id: u64,
    }

    impl ExecutionNFT {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                psp34: Default::default(),
                ownable: Default::default(),
                parent_id: Default::default(),
                token_id: Default::default(),
            }
        }

        #[ink(message)]
        pub fn mint(&mut self, to: AccountId, parent_id: Id) -> Result<(), AlgoExecuteError> {
            let mint_id = self.token_id.saturating_add(1);
            self.token_id = mint_id;
            self.parent_id.insert(Id::U64(mint_id), &parent_id);
            let _ = psp34::Internal::_mint_to(self, to, Id::U64(mint_id));
            Ok(())
        }

        #[ink(message)]
        pub fn set_parent_id(
            &mut self,
            token_id: Id,
            parent_id: Id,
        ) -> Result<(), AlgoExecuteError> {
            self.ensure_owner()?;
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

        pub fn verify_execution_rights(
            &self,
            exec_id: Id,
            algo_id: Id,
        ) -> Result<(), AlgoExecuteError> {
            let parent_id = self
                .parent_id
                .get(&exec_id)
                .ok_or(AlgoExecuteError::InvalidExecutionNFT)?;
            if parent_id != algo_id {
                Err(AlgoExecuteError::UnauthorizedAccess)
            } else {
                Ok(())
            }
        }

        fn ensure_owner(&mut self) -> Result<(), AlgoExecuteError> {
            let owner = self
                .ownable
                .owner
                .get()
                .ok_or(AlgoExecuteError::NotAuthorized)?;
            if Some(owner) == Some(Some(self.env().caller())) {
                Ok(())
            } else {
                Err(AlgoExecuteError::NotAuthorized)
            }
        }
    }
}
