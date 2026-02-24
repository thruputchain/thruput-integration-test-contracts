#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "dev")]
extern crate std;

extern crate alloc;

mod interface;
mod implementation;

// generated code for testing
#[rustfmt::skip]
mod roundtrip;

// A client stub that provides the interface via cross-contract calls
pub use interface::Client;

// Contract methods exposed with C ABI for running as guest
pub use implementation::exports::*;

#[cfg(test)]
mod tests;
