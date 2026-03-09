#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "dev")]
extern crate std;

extern crate alloc;

pub mod interface;
pub use interface::Client;

mod implementation;

// Contract methods exposed with C ABI for running as guest
pub use implementation::exports::*;

#[cfg(test)]
mod tests;

#[cfg(feature = "bottle-test")]
mod bottle_test;
#[cfg(feature = "bottle-test")]
pub use bottle_test::*;
