#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "dev")]
extern crate std;


mod interface;
mod implementation;

// A client stub that provides the interface via cross-contract calls
pub use interface::Client;

// An implementation of the contract interface
pub use implementation::FibonacciContract;

pub use interface::Fibonacci;

#[cfg(all(test, feature = "dev"))]
mod tests;

#[cfg(feature = "dev")]
pub mod checks;
