#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std;

pub mod interface;

#[cfg(feature = "client")]
pub use interface::Client;

#[cfg(feature = "guest")]
pub mod implementation;

#[cfg(feature = "guest")]
pub use implementation::guest_exports::*;
