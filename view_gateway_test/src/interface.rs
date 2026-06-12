use alloc::vec::Vec;
use sdk::*;

#[contract(client = Client)]
pub trait ViewGatewayTest {
    fn read_value(key: &[u8; 32]) -> Vec<u8>;
}
