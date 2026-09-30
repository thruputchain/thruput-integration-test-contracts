#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "dev")]
extern crate std;

extern crate alloc;

mod implementation;
mod interface;

pub use implementation::GenesisRootContract;
pub use interface::{AccountAllocation, Client, Distribution, GenesisRoot};

sdk::declare_mock_contract!();
