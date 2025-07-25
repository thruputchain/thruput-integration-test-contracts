use sdk::prelude::*;

pub use sdk::prelude::{Error, Result};

#[contract(client = "Client")]
pub trait Fibonacci {
    fn fibonacci(&self, n: u32) -> u32;
    fn slow_fibonacci(&self, n: u32) -> u32;
}
