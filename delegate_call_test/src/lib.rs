#![no_std]

#[cfg(feature = "dev")]
extern crate std;

extern crate alloc;

mod interface;

// A trait describing the contract interface
pub use interface::DelegateCallTest;

// A client stub that provides the interface via cross-contract calls
pub use interface::Client;

// An implementation of the contract interface
mod implementation;
