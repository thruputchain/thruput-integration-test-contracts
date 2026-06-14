#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "dev")]
extern crate std;


mod implementation;
mod interface;

pub use implementation::DeployAndTransferContract;
pub use interface::Client;
pub use interface::DeployAndTransfer;

sdk::declare_mock_contract!();
