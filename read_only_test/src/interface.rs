use sdk::*;

pub use sdk::contract_result::*;

#[contract(client = "Client")]
pub trait ReadOnlyTest {
    fn read_only_bytestring_set(&self, expect_error: bool);
}
