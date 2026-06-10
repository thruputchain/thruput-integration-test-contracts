use super::interface::ViewGatewayTest;

use alloc::vec::Vec;
use sdk::*;

#[derive(Default)]
pub struct ViewGatewayTestContract;

#[contract(storage = false)]
impl ViewGatewayTest for ViewGatewayTestContract {
    fn read_value(&self, key: &[u8; 32]) -> Vec<u8> {
        let storage_key = StorageKey::new(*key);
        bytestring_get(&storage_key).to_vec()
    }
}
