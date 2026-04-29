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

// Raw wasm export that returns without calling the Return syscall.
// Entrypoint 0xFF → export name "00000000000000FF".
#[export_name = "00000000000000FF"]
pub extern "C" fn noop_no_return_syscall() {}
