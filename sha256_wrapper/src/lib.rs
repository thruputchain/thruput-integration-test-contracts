#![no_std]

#[cfg(feature = "dev")]
extern crate std;

extern crate alloc;

pub mod interface;
mod implementation;

pub use interface::Sha256Wrapper;

// A client stub that provides the interface via cross-contract calls
pub use interface::Client;

pub use implementation::Sha256WrapperContract;

#[cfg(test)]
mod tests;

#[cfg(feature = "dev")]
pub mod checks;
