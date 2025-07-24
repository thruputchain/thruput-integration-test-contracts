//! ERC20 implementation without storage feature

#![cfg_attr(not(feature = "mock"), no_std)]

pub use erc20_if as interface;

// An implementation of the contract interface
#[cfg(feature = "guest")]
mod implementation;

// Contract methods exposed with C ABI for running as guest
#[cfg(feature = "guest")]
pub use implementation::*;

#[cfg(feature = "mock")]
pub use implementation::test_setup;
