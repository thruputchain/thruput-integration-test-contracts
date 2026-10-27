//! RV32 binary wrapper for the `fibonacci` contract.
//!
//! The runtime (`_start`, panic handler, allocator, linker script) is
//! auto-wired by the SDK on the riscv32 target. This wrapper exists so
//! `cargo build --bin fibonacci` produces an ELF executable; the
//! `extern crate` force-links the contract rlib so the entrypoint records its
//! `#[contract]` impls emit land in the guest dispatch table.
#![no_std]
#![no_main]

extern crate fibonacci;
