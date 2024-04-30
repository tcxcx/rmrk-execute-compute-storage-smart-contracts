#![cfg_attr(not(feature = "std"), no_std)]

use ink::storage::Mapping;
use openbrush::contracts::ownable::*;
use rmrk::types::{Part, ChildNft, PartType};

#[ink::contract]
pub mod catalog {
    use super::*;

    #[ink(storage)]
    #[derive(Default)]
    pub struct Catalog {
        ownable: ownable::Data,
        catalog: Mapping<u64, Part>,
        next_part_id: u64,
        part_owners: Mapping<u64, AccountId>,
        nested_children: Mapping<u64, Vec<ChildNft>>,
    }

    impl Catalog {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            ownable::Internal::_init_with_owner(&mut instance, Self::env().caller());
            instance
        }

        #[ink(message)]
        pub fn add_part(&mut self, part: Part) -> Result<u64, ()> {
            let part_id = self.next_part_id;
            self.catalog.insert(part_id, &part);
            self.part_owners.insert(part_id, &Self::env().caller());
            self.next_part_id += 1;
            Ok(part_id)
        }

        #[ink(message)]
        pub fn get_part(&self, part_id: u64) -> Option<Part> {
            self.catalog.get(&part_id)
        }

        #[ink(message)]
        pub fn get_part_owner(&self, part_id: u64) -> Option<AccountId> {
            self.part_owners.get(&part_id)
        }

        #[ink(message)]
        pub fn add_nested_child(&mut self, part_id: u64, child: ChildNft) -> Result<(), ()> {
            let mut children = self.nested_children.get(&part_id).unwrap_or_default();
            children.push(child);
            self.nested_children.insert(part_id, &children);
            Ok(())
        }

        #[ink(message)]
        pub fn get_nested_children(&self, part_id: u64) -> Vec<ChildNft> {
            self.nested_children.get(&part_id).unwrap_or_default()
        }

        #[ink(message)]
        pub fn lazy_mint(&mut self, part_id: u64, part_type: PartType) -> Result<(), ()> {
            let mut part = self.catalog.get(&part_id).ok_or(())?;
            part.part_type = part_type;
            self.catalog.insert(part_id, &part);
            Ok(())
        }
    }

    impl Ownable for Catalog {
        fn owner(&self) -> Option<AccountId> {
            self.ownable.owner.get()
        }

        fn transfer_ownership(&mut self, new_owner: Option<AccountId>) -> Result<(), OwnableError> {
            let caller = Self::env().caller();
            self.ownable._transfer_ownership(caller, new_owner)
        }

        fn renounce_ownership(&mut self) -> Result<(), OwnableError> {
            let caller = Self::env().caller();
            self.ownable._transfer_ownership(caller, None)
        }
    }
}