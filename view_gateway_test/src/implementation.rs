use super::interface::ViewGatewayTest;
use alloc::vec::Vec;
use sdk::*;

pub struct ViewGatewayTestContract;

#[contract]
impl ViewGatewayTest for ViewGatewayTestContract {
    fn read_value(key: &[u8; 32]) -> Vec<u8> {
        let storage_key = StorageKey::new(*key);
        bytestring_get(&storage_key).to_vec()
    }
}
