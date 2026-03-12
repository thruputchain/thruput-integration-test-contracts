#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "dev")]
extern crate std;

extern crate alloc;

mod interface;
mod implementation;

// A trait describing the contract interface
pub use interface::Client;

pub use implementation::Ed25519VerifyWrapperContract;

// Contract methods exposed with C ABI for running as guest
pub use implementation::exports::*;

#[cfg(feature = "dev")]
pub use implementation::test_setup;

#[cfg(test)]
mod tests;
