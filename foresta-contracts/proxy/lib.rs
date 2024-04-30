// lib.rs
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

pub mod proxy;
pub mod types;

pub use proxy::*;
pub use types::*;


use ink::storage::Mapping;
use openbrush::{
    contracts::{
        access_control::*,
        ownable::*,
        psp34::*,
        reentrancy_guard::*,
    },
    modifiers,
    traits::{AccountId, Balance, Storage, String},
};
use rmrk::storage::catalog_external::Catalog;