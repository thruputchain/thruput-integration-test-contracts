use sdk::prelude::*;

pub use sdk::prelude::{Error, Result};

use alloc::vec::Vec;

#[contract(client = "Client")]
pub trait CalldataWrapper {
    fn echo(&self, data: &[u8]) -> Vec<u8>;
}
