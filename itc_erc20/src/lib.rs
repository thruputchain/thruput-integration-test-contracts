//! ERC20 implementation without storage feature

#![cfg_attr(not(feature = "std"), no_std)]

pub use groundhog_erc20_if as interface;

pub use interface::ERC20Client;

// This is required if the logging feature is enabled
#[cfg(feature = "alloc")]
extern crate alloc;

// An implementation of the contract interface
mod implementation;

// Contract methods exposed with C ABI for running as guest
pub use implementation::*;

#[cfg(feature = "dev")]
pub use implementation::test_setup;
