#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "dev")]
extern crate std;

extern crate alloc;

// A trait describing the contract interface
pub mod interface;

// A client stub that provides the interface via cross-contract calls
#[cfg(feature = "client")]
pub use interface::Client;

// An implementation of the contract interface
#[cfg(feature = "guest")]
pub mod implementation;

pub use implementation::TupleReturnContract;
pub use interface::TupleReturn;

#[cfg(all(test, feature = "dev"))]
mod tests;
