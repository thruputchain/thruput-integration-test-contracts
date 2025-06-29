use sdk::prelude::*;

pub use sdk::prelude::{Error, Result};

use alloc::vec::Vec;

#[contract(client = "client")]
pub trait CalldataWrapper {
    fn echo(&self, data: &Vec<u8>) -> Vec<u8>;
}
