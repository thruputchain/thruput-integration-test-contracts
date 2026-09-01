#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std;


pub mod interface;
mod implementation;

pub use interface::ManualRevertTestError;

// A client stub that provides the interface via cross-contract calls
pub use interface::Client;

