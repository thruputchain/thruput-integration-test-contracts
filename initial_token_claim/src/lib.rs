#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "dev")]
extern crate std;

extern crate alloc;

mod interface;
mod implementation;

pub use interface::InitialTokenClaim;

// A client stub that provides the interface via cross-contract calls
pub use interface::Client;
