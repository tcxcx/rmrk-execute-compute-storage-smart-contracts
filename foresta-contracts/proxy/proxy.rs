// Manages the mapping of NFT IDs to CIDs, essential for resolving content linked to NFTs.


#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use ink::storage::Mapping;
use ink::env::DefaultEnvironment;

pub use scale::{Decode, Encode};

#[ink::contract(env = DefaultEnvironment)]
pub mod rmrk_proxy {
    use super::*;

    pub type Id = u32; // Define how NFT IDs are represented

    #[derive(Encode, Decode, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotAuthorized,
        NotFound,
        StorageError,
        UnexpectedError
    }

    pub type Result<T> = core::result::Result<T, Error>;

    #[ink(storage)]
    pub struct RmrkProxy {
        /// Mapping from NFT ID to content identifier on IPFS
        nft_to_cid: Mapping<Id, String>,
        owner: AccountId,
    }

    impl RmrkProxy {
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            Self {
                nft_to_cid: Mapping::new(),
                owner: caller,
            }
        }

        #[ink(message)]
        pub fn add_nft(&mut self, nft_id: Id, cid: String) -> Result<()> {
            self.only_owner()?;
            self.nft_to_cid.insert(&nft_id, &cid);
            Ok(())
        }

        #[ink(message)]
        pub fn get_cid(&self, nft_id: Id) -> Result<String> {
            let cid = self.nft_to_cid.get(&nft_id).ok_or(Error::NotFound)?;
            Ok(cid)
        }

        #[ink(message)]
        pub fn change_owner(&mut self, new_owner: AccountId) -> Result<()> {
            self.only_owner()?;
            self.owner = new_owner;
            Ok(())
        }

        fn only_owner(&self) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotAuthorized);
            }
            Ok(())
        }
    }
}
