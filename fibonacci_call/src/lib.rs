#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "dev")]
extern crate std;

extern crate alloc;

mod interface;
mod implementation;

// A client stub that provides the interface via cross-contract calls
pub use interface::FibonacciCaller;

// An implementation of the contract interface
pub use implementation::FibonacciContractCaller;

// Contract methods exposed with C ABI for running as guest
pub use implementation::exports::*;
