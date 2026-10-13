#![no_std]

#[cfg(feature = "dev")]
extern crate std;


mod implementation;
pub mod interface;

// A trait describing the contract interface
pub use interface::SyscallMiscTests;
pub use interface::SyscallMiscTestsEntrypoints;

// A client stub that provides the interface via cross-contract calls
pub use interface::Client;

// Raw entrypoint 0xFF: a function that returns without calling the Return
// syscall (see test_invoke_no_return_syscall).
//
// rv32 dispatches guest-side: the #[contract] macro normally emits one
// `Entrypoint` record per method into the `groundhog_rv32_entrypoints`
// section, which the SDK's `_start` dispatcher scans. Hand-writing a record
// registers a raw entrypoint outside the macro's calldata/Return glue. The
// dispatcher falls through to its unknown-method arm when a wrapper returns,
// so invoking this entrypoint aborts the child frame with
// LibraryExn::UnknownMethod.
#[cfg(target_arch = "riscv32")]
mod rv32_no_return_entry {
    extern "C" fn noop_no_return_syscall() {}

    #[used]
    #[unsafe(link_section = "groundhog_rv32_entrypoints")]
    static NO_RETURN_ENTRY: thruput_sdk::__rv32::Entrypoint =
        thruput_sdk::__rv32::Entrypoint {
            method_id: 0xFF,
            wrapper: noop_no_return_syscall,
        };
}
