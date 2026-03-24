#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "dev")]
extern crate std;

extern crate alloc;

mod interface;
mod implementation;

// A trait describing the contract interface
pub use interface::Client;

pub use implementation::Ed25519VerifyWrapperContract;

#[cfg(feature = "dev")]
pub use implementation::Ed25519VerifyWrapperTestSetup;

#[cfg(test)]
mod tests;
