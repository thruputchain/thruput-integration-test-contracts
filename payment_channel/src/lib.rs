#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std;

extern crate alloc;

mod interface;
mod implementation;

pub use interface::PaymentChannel;

// Re-export ChannelState for use in tests and types generation
pub use interface::ChannelState;

// A client stub that provides the interface via cross-contract calls
pub use interface::Client;
