#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "dev")]
extern crate std;

// A trait describing the contract interface
pub mod interface;
pub use interface::SimpleWalletAuthClaimClient as Client;

#[cfg(feature="alloc")]
extern crate alloc;

// An implementation of the contract interface
#[cfg(feature = "guest")]
pub mod implementation;

