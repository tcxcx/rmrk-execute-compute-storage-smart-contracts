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
        token_id: u64,
    }

    impl ExecutionNFT {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                psp34: Default::default(),
                ownable: Default::default(),
                token_id: Default::default(),
            }
        }

        #[ink(message)]
        pub fn mint(&mut self, to: AccountId) -> Result<Id, AlgoExecuteError> {
            let mint_id = self.token_id.saturating_add(1);
            self.token_id = mint_id;
            let _ = psp34::Internal::_mint_to(self, to, Id::U64(mint_id));
            Ok(Id::U64(mint_id))
        }
        
        #[ink(message)]
        pub fn is_owner(&self, exec_id: Id, owner_address: AccountId) -> bool {
            match psp34::Internal::_owner_of(self, &exec_id) {
                Some(owner) => owner == owner_address,
                None => false,
            }
        }
    }
}