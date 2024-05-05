use crate::error::AlgoExecuteError;
use ink::storage::Mapping;
use openbrush::{
    contracts::{
        ownable::*,
        psp34::Id,
    },
    traits::{
        Storage,
        String,
    },
};

#[openbrush::implementation(PSP34)]
#[openbrush::contract]
pub mod algo_nft {
    use super::*;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct AlgorithmNFT {
        #[storage_field]
        psp34: psp34::Data,
        ownable: ownable::Data,
        metadata: Mapping<Id, String>,
        algorithm_cid: Mapping<Id, String>,
        algo_id: u64,
        execute_nfts: Mapping<Id, Vec<Id>>,
    }

    impl AlgorithmNFT {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                psp34: Default::default(),
                ownable: Default::default(),
                metadata: Default::default(),
                algorithm_cid: Mapping::default(),
                algo_id: Default::default(),
                execute_nfts: Mapping::default(),
            }
        }

        #[ink(message)]
        pub fn mint(
            &mut self,
            to: AccountId,
            algorithm_cid: String,
        ) -> Result<Id, AlgoExecuteError> {
            let mint_id = self.algo_id.saturating_add(1);
            self.algo_id = mint_id;
            self.algorithm_cid.insert(Id::U64(mint_id), &algorithm_cid);
            let _ = psp34::Internal::_mint_to(self, to, Id::U64(mint_id));
            Ok(Id::U64(mint_id))
        }

        #[ink(message)]
        pub fn add_execute_nft(
            &mut self,
            algo_id: Id,
            exec_id: Id,
        ) -> Result<(), AlgoExecuteError> {
            self.ensure_owner()?;
            let mut exec_nfts = self.execute_nfts.get(&algo_id).unwrap_or_default();
            exec_nfts.push(exec_id);
            self.execute_nfts.insert(&algo_id, &exec_nfts);
            Ok(())
        }

        #[ink(message)]
        pub fn get_execute_nfts(&self, algo_id: Id) -> Vec<Id> {
            self.execute_nfts.get(&algo_id).unwrap_or_default()
        }

        #[ink(message)]
        pub fn set_metadata(
            &mut self,
            algo_id: Id,
            metadata: String,
        ) -> Result<(), AlgoExecuteError> {
            self.ensure_owner()?;
            self.metadata.insert(&algo_id, &metadata);
            Ok(())
        }

        #[ink(message)]
        pub fn set_algorithm_cid(
            &mut self,
            algo_id: Id,
            algorithm_cid: String,
        ) -> Result<(), AlgoExecuteError> {
            self.ensure_owner()?;
            self.algorithm_cid.insert(&algo_id, &algorithm_cid);
            Ok(())
        }

        #[ink(message)]
        pub fn get_metadata(&self, algo_id: Id) -> Option<String> {
            self.metadata.get(&algo_id)
        }

        #[ink(message)]
        pub fn get_algorithm_cid(&self, algo_id: Id) -> Option<String> {
            self.algorithm_cid.get(&algo_id)
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

        pub fn fetch_algorithm_data(&self, algo_id: Id) -> Result<String, AlgoExecuteError> {
            self.algorithm_cid
                .get(&algo_id)
                .ok_or(AlgoExecuteError::DataNotFound)
        }
    }    
}
