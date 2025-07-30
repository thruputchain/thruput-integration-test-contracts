use sdk::prelude::*;

pub use sdk::prelude::{Error, Result};

#[contract(client = "Client")]
pub trait ReadOnlyTest {
    fn read_only_bytestring_set(&self, expect_error: bool);
}
