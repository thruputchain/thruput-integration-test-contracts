use thruput_sdk::*;

use alloc::vec::Vec;

#[contract(client = Client)]
pub trait CalldataWrapper {
    fn echo(data: &[u8]) -> Vec<u8>;
}
