//! RV32 binary wrapper for the ITC smoke counter contract.

#![no_std]
#![no_main]

// Link the contract library so its macro-emitted method entry symbols are
// linked in for the lowerer to discover. The runtime (_start, panic
// handler, allocator, linker script) arrives transitively via
// rv32_smoke_counter -> thruput-sdk, which force-links thruput-rv32-runtime on
// rv32 targets.
extern crate rv32_smoke_counter;
