#![no_std]

#[cfg(feature = "mock")]
extern crate std;

extern crate alloc;

// A trait describing the contract interface
pub mod interface;

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
