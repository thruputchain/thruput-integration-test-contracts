#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "mock")]
extern crate std;

extern crate alloc;

// A trait describing the contract interface
pub mod interface;

// A client stub that provides the interface via cross-contract calls
#[cfg(feature = "client")]
pub use interface::Client;

// An implementation of the contract interface
#[cfg(feature = "guest")]
pub mod implementation;

// Contract methods exposed with C ABI for running as guest
#[cfg(feature = "guest")]
pub use implementation::guest_exports::*;

#[cfg(feature = "mock")]
pub use implementation::test_setup;

#[cfg(test)]
mod tests;

#[cfg(feature = "bottle-test")]
mod bottle_test;
#[cfg(feature = "bottle-test")]
pub use bottle_test::*;
