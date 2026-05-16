#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod interface;
pub use interface::Client;

mod implementation;
pub use implementation::ReadOnlyTestContract;

pub use interface::ReadOnlyTest;
pub use interface::ReadOnlyTestEntrypoints;

#[cfg(test)]
mod tests;

#[cfg(feature = "bottle-test")]
mod bottle_test;
#[cfg(feature = "bottle-test")]
pub use bottle_test::*;
