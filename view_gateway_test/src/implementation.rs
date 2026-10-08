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

    fn emit_events(count: u32) {
        for _ in 0..count {
            groundhog_xdr::syscalls::raw_LogEvent(None, None, None, &[0x5a; 1024])
                .expect("test event must fit");
        }
    }
}
