//! RV32 binary wrapper for the ITC smoke counter contract.

#![no_std]
#![no_main]

// The runtime crate provides _start, panic handler, allocator, and linker
// script for the RV32 target.
extern crate groundhog_rv32_runtime;

// Link the contract library so its macro-emitted method entry symbols are
// present in `.text.gh_*` for the lowerer to discover.
extern crate rv32_smoke_counter;
