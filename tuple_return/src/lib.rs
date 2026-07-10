#![cfg_attr(not(feature = "std"), no_std)]
// This contract deliberately exercises gnarly return signatures (kitchen-sink
// tuples, explicit unit returns) — that IS the test, so silence the lints
// that would "fix" it away.
#![allow(clippy::type_complexity)]
#![allow(clippy::unused_unit)]

#[cfg(feature = "dev")]
extern crate std;

extern crate alloc;

// A trait describing the contract interface
pub mod interface;

// A client stub that provides the interface via cross-contract calls
pub use interface::Client;

// An implementation of the contract interface
pub mod implementation;

pub use implementation::TupleReturnContract;
pub use interface::TupleReturn;

#[cfg(all(test, feature = "dev"))]
mod tests;
