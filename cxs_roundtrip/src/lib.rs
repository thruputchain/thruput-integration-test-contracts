#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

// generated code for testing
#[rustfmt::skip]
mod roundtrip;

// A trait describing the contract interface
pub mod interface;

// A client stub that provides the interface via cross-contract calls
pub use interface::Client;

// An implementation of the contract interface
pub mod implementation;

#[cfg(feature = "dev")]
pub use implementation::CxsRoundtripContract;
#[cfg(feature = "dev")]
pub use interface::CxsRoundtrip;

#[cfg(test)]
mod tests;
