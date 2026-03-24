#![no_std]

#[cfg(any(feature = "dev", feature = "spec"))]
extern crate std;

extern crate alloc;

pub mod interface;
mod implementation;

pub use interface::Sha256Wrapper;

// A client stub that provides the interface via cross-contract calls
pub use interface::Client;

pub use implementation::Sha256WrapperContract;

#[cfg(feature = "dev")]
pub use implementation::Sha256WrapperTestSetup;

#[cfg(test)]
mod tests;

#[cfg(feature = "bottle-test")]
mod bottle_test;
#[cfg(feature = "bottle-test")]
pub use bottle_test::*;
