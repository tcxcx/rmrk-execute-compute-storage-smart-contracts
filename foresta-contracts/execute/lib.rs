#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

pub mod execute_nft;
pub mod error;

pub use execute_nft::*;
pub use error::AlgoExecuteError;