#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std;

// A trait describing the contract interface
pub mod interface;
pub use interface::NoSyscallTestClient;

// An implementation of the contract interface
pub mod implementation;


