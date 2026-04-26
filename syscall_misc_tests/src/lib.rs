#![no_std]

#[cfg(any(feature = "dev", feature = "spec"))]
extern crate std;

extern crate alloc;

mod implementation;
pub mod interface;

// A trait describing the contract interface
pub use interface::SyscallMiscTests;
pub use interface::SyscallMiscTestsEntrypoints;

// A client stub that provides the interface via cross-contract calls
pub use interface::Client;
