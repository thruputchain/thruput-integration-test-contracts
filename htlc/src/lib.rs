#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std;

mod interface;
mod implementation;

// A trait describing the contract interface
pub use interface::HTLC;

// A client stub that provides the interface via cross-contract calls
pub use interface::Client;

// Contract methods exposed with C ABI for running as guest
pub use implementation::exports::*;

