//! ERC20 implementation without storage feature

#![cfg_attr(not(feature = "std"), no_std)]

pub use groundhog_erc20_if as interface;

pub use interface::ERC20Client;

#[cfg(feature = "alloc")]
extern crate alloc;

// An implementation of the contract interface
#[cfg(feature = "guest")]
mod implementation;

// Contract methods exposed with C ABI for running as guest
#[cfg(feature = "guest")]
pub use implementation::*;

#[cfg(feature = "mock")]
pub use implementation::test_setup;

#[cfg(feature = "bottle-test")]
pub use erc20_bottle_test::*;
