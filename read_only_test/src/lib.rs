#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "dev")]
extern crate std;

extern crate alloc;

pub mod interface;
pub use interface::Client;

mod implementation;

pub use implementation::ReadOnlyTestContract;

// Contract methods exposed with C ABI for running as guest
pub use implementation::read_only_test_exports::*;

pub use interface::ReadOnlyTest;

#[cfg(test)]
mod tests;

#[cfg(feature = "bottle-test")]
mod bottle_test;
#[cfg(feature = "bottle-test")]
pub use bottle_test::*;
