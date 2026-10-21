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

#[cfg(feature = "dev")]
pub mod checks;
