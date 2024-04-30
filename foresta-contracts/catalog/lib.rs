#![cfg_attr(not(feature = "std"), no_std, no_main)]

use ink::storage::Mapping;
use openbrush::contracts::ownable::*;
use openbrush::traits::Storage;
use rmrk::types::{Part, ChildNft, PartType};
use ink::prelude::vec::Vec;

#[openbrush::implementation(Ownable)]
#[openbrush::contract]
pub mod catalog {
    use super::*;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Catalog {
        #[storage_field]
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
            let _ = self.next_part_id.checked_add(1);
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

}