use alloc::vec::Vec;
use thruput_sdk::*;

#[contract(client = Client)]
pub trait ViewGatewayTest {
    fn read_value(key: &[u8; 32]) -> Vec<u8>;
    fn emit_events(count: u32);
}
