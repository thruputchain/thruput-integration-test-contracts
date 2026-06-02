//! RV32 binary wrapper for the `fibonacci` contract.
//!
//! The runtime (`_start`, panic handler, allocator, linker script) is
//! auto-wired by the SDK on the riscv32 target. This wrapper exists so
//! `cargo build --bin fibonacci` produces an ELF executable; the
//! `extern crate` force-links the contract rlib so its `#[contract]`-emitted
//! `gh_*` method-entry symbols land in `.text.gh_*` for the lowerer.
#![no_std]
#![no_main]

extern crate fibonacci;
