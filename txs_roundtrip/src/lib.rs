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

pub use implementation::TxsRoundtripContract;

#[cfg(feature = "dev")]
pub use implementation::TxsRoundtripTestSetup;

#[cfg(test)]
mod tests;
