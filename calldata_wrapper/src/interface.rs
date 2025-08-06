use sdk::prelude::*;

use alloc::vec::Vec;

#[contract(client = "Client")]
pub trait CalldataWrapper {
    fn echo(&self, data: &[u8]) -> Vec<u8>;
}
