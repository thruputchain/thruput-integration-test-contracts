#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "dev")]
extern crate std;


mod interface;
mod implementation;

// A client stub that provides the interface via cross-contract calls
pub use interface::FibonacciCaller;
pub use interface::CallerClient as Client;

// An implementation of the contract interface
pub use implementation::FibonacciContractCaller;
