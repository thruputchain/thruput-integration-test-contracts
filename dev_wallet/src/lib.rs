#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "dev")]
extern crate std;

pub mod implementation;
pub mod interface;

pub use interface::Client;
