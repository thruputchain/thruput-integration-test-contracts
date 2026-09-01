#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod interface;
mod implementation;

pub use interface::Client;
pub use implementation::ViewGatewayTestContract;
pub use interface::ViewGatewayTest;
