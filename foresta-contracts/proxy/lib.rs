#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

pub mod error;
pub mod proxy;
pub mod proxy_traits;

pub use proxy::*;
pub use proxy_traits::*;
pub use error::*;
