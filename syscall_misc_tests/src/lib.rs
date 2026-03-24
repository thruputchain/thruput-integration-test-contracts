#![no_std]

#[cfg(any(feature = "dev", feature = "spec"))]
extern crate std;

extern crate alloc;

pub mod interface;
mod implementation;

// A trait describing the contract interface
pub use interface::SyscallMiscTests;

// A client stub that provides the interface via cross-contract calls
pub use interface::Client;

// Contract methods exposed with C ABI for running as guest
pub use implementation::SyscallMiscTests_exports::*;
