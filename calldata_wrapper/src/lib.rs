#![no_std]

#[cfg(feature = "dev")]
extern crate std;

extern crate alloc;

mod interface;
pub use interface::*;

mod implementation;

pub use implementation::CalldataWrapperContract;

#[cfg(test)]
mod tests;

#[cfg(feature = "bottle-test")]
mod bottle_test;
#[cfg(feature = "bottle-test")]
pub use bottle_test::*;
