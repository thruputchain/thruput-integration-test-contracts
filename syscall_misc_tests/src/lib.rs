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
// syscall. Wired per target because the two runtimes dispatch differently,
// and the two runtimes give it different semantics (see
// test_invoke_no_return_syscall).

// wasm dispatches host-side via the export table: export name
// "00000000000000FF". A bare export return maps to a successful empty
// Return.
#[cfg(target_arch = "wasm32")]
#[export_name = "00000000000000FF"]
pub extern "C" fn noop_no_return_syscall() {}

// rv32 dispatches guest-side: the #[contract] macro normally emits one
// `Entrypoint` record per method into the `groundhog_rv32_entrypoints`
// section, which the SDK's `_start` dispatcher scans. Hand-writing a record
// registers a raw entrypoint outside the macro's calldata/Return glue. The
// dispatcher traps (ebreak) when a wrapper returns, so invoking this
// entrypoint deterministically faults the child frame.
#[cfg(target_arch = "riscv32")]
mod rv32_no_return_entry {
    extern "C" fn noop_no_return_syscall() {}

    #[used]
    #[unsafe(link_section = "groundhog_rv32_entrypoints")]
    static NO_RETURN_ENTRY: sdk::__rv32::Entrypoint =
        sdk::__rv32::Entrypoint {
            method_id: 0xFF,
            wrapper: noop_no_return_syscall,
        };
}
