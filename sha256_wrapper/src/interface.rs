use sdk::prelude::*;

pub use sdk::prelude::{Error, Result};

use alloc::vec::Vec;

#[contract(client = "client")]
pub trait Sha256Wrapper {
    fn sha256(&self, data: &Vec<u8>) -> SHA256Hash;
    fn sha256_repeated(&self, n: u32, data: &Vec<u8>);
}
