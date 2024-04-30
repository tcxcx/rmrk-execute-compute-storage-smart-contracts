#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

pub mod algo_nft;
pub mod error;

pub use algo_nft::*;
pub use error::AlgoExecuteError;