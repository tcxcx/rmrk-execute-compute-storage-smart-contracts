#![cfg_attr(not(feature = "std"), no_std)]
use ink::env::{call::utils::EmptyReturnType, hash::FromHex};
use ink_ipfs::{Error as IPFSError, IPFS};
use phat_offchain_rollup;
use rmrk::traits::multiasset_external::MultiAsset;

const PHALA_MESSAGE_EXECUTE_ALGORITHM: [u8; 4] = [0x12, 0x34, 0xAB, 0xCD];

#[ink::contract]
pub mod rmrk_proxy {
    use super::*;

    #[ink(storage)]
    pub struct RmrkProxy {
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        guard: reentrancy_guard::Data,
        #[storage_field]
        access: access_control::Data,
        #[storage_field]
        proxy: Data,
        #[storage_field]
        as_nfts: Mapping<AccountId, AlgorithmNFT>,
        #[storage_field]
        ea_nfts: Mapping<AccountId, ExecutionNFT>,
        #[storage_field]
        ipfs: IPFS,
    }

    impl RmrkProxy {
        #[ink(constructor)]
        pub fn new(
            rmrk_contract: AccountId,
            catalog_contract: AccountId,
            phala_contract: AccountId,
            mint_price: Balance,
        ) -> Self {
            let mut instance = Self {
                ownable: Default::default(),
                guard: Default::default(),
                access: Default::default(),
                proxy: Data {
                    rmrk_contract: Some(rmrk_contract),
                    catalog_contract: Some(catalog_contract),
                    phala_contract: Some(phala_contract),
                    mint_price,
                    salt: 0,
                },
                as_nfts: Default::default(),
                ea_nfts: Default::default(),
                ipfs: Default::default(),
            };

            let caller = instance.env().caller();
            instance._init_with_owner(caller);
            instance
        }

        #[ink(message)]
        #[modifiers(non_reentrant)]
        pub fn mint_as_nft(
            &mut self,
            metadata: String,
            algorithm_cid: String,
        ) -> Result<Id, ProxyError> {
            let caller = self.env().caller();
            let mut as_nft = self.as_nfts.get(&caller).unwrap_or_default();

            let token_id = as_nft._mint_to(&caller, Vec::new())?;
            as_nft.set_metadata(token_id.clone(), metadata)?;
            as_nft.set_algorithm_cid(token_id.clone(), algorithm_cid)?;

            self.as_nfts.insert(&caller, &as_nft);
            Ok(token_id)
        }

        #[ink(message, payable)]
        #[modifiers(non_reentrant)]
        pub fn mint_ea_nft(&mut self, parent_id: Id) -> Result<Id, ProxyError> {
            let caller = self.env().caller();
            let transferred = self.env().transferred_value();

            if transferred != self.proxy.mint_price {
                return Err(ProxyError::BadMintValue);
            }

            let as_nft = self.as_nfts.get(&caller).ok_or(ProxyError::NoParentNFT)?;
            as_nft.owner_of(&parent_id).ok_or(ProxyError::InvalidParentNFT)?;

            let mut ea_nft = self.ea_nfts.get(&caller).unwrap_or_default();
            let token_id = ea_nft._mint_to(&caller, Vec::new())?;
            ea_nft.set_parent_id(token_id.clone(), parent_id)?;

            self.ea_nfts.insert(&caller, &ea_nft);
            Ok(token_id)
        }

        #[ink(message)]
        pub fn execute_algorithm(
            &self,
            ea_token_id: Id,
            input: Vec<u8>,
        ) -> Result<Vec<u8>, ProxyError> {
            let caller = self.env().caller();
            let ea_nft = self.ea_nfts.get(&caller).ok_or(ProxyError::NoExecutionNFT)?;

            ea_nft.owner_of(&ea_token_id).ok_or(ProxyError::InvalidExecutionNFT)?;

            let parent_id = ea_nft.get_parent_id(ea_token_id).ok_or(ProxyError::NoParentId)?;
            let as_nft = self.as_nfts.get(&caller).ok_or(ProxyError::NoAlgorithmNFT)?;
            let cid = as_nft.algorithm_cid.get(&parent_id).ok_or(ProxyError::NoAlgorithmCID)?;

            // Fetch encrypted algorithm from IPFS
            let encrypted_algorithm = self.ipfs.get(&cid).map_err(|_| ProxyError::IPFSError)?;

            // Call the Schrodinger contract to execute the algorithm securely
            let schrodinger_contract = self.proxy.schrodinger_contract.ok_or(ProxyError::SchrodingerError)?;
            let output = ink::env::call::build_call::<ink::env::DefaultEnvironment>()
                .call(schrodinger_contract)
                .exec_input(
                    ExecutionInput::new(Selector::new([0x12, 0x34, 0xAB, 0xCD]))
                        .push_arg(encrypted_algorithm)
                        .push_arg(input),
                )
                .returns::<Vec<u8>>()
                .fire()
                .map_err(|_| ProxyError::SchrodingerError)?;

            Ok(output)
        }
    }
}
