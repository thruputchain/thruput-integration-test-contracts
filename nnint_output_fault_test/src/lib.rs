#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std;

pub mod interface;

pub use interface::Client;

#[cfg(feature = "guest")]
pub mod implementation;
