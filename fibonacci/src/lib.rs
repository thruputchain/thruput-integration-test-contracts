#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "dev")]
extern crate std;

extern crate alloc;

mod interface;
mod implementation;

// A client stub that provides the interface via cross-contract calls
pub use interface::Client;

// An implementation of the contract interface
pub use implementation::FibonacciContract;

// Contract methods exposed with C ABI for running as guest
pub use implementation::exports::*;


#[cfg(feature = "dev")]
pub use implementation::test_setup;

#[cfg(all(test, feature = "dev"))]
mod tests;

#[cfg(feature = "bottle-test")]
mod bottle_test;
#[cfg(feature = "bottle-test")]
pub use bottle_test::*;
