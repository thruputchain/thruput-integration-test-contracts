#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod interface;
pub use interface::Client;

mod implementation;
pub use implementation::StorageFillerContract;

pub use interface::StorageFiller;
pub use interface::StorageFillerEntrypoints;
