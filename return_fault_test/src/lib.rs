#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std;

mod interface;
mod implementation;

pub use interface::ReturnFaultTest;

// A client stub that provides the interface via cross-contract calls
pub use interface::Client;
pub use interface::ClientError;

pub use implementation::ReturnFaultTestContract;
