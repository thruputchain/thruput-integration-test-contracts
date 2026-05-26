use sdk::*;

#[contract(client = Client)]
pub trait ViewGatewayTest {
    fn read_value(&self, key: &[u8; 32]) -> Vec<u8>;
}
