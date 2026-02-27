#![no_std]

#[cfg(feature = "mock")]
extern crate std;

extern crate alloc;

mod interface;

// A trait describing the contract interface
pub use interface::DelegateCallTest;

// A client stub that provides the interface via cross-contract calls
pub use interface::Client;

// An implementation of the contract interface
mod implementation;

// Contract methods exposed with C ABI for running as guest
pub use implementation::exports::*;
