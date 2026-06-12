use alloc::vec::Vec;
use sdk::*;

#[contract(client = Client)]
pub trait TxsRoundtrip {
    fn roundtrip_foo(&self, data: &[u8]) -> Vec<u8>;
    fn roundtrip_senum(&self, data: &[u8]) -> Vec<u8>;
    fn roundtrip_sunion(&self, data: &[u8]) -> Vec<u8>;
    fn roundtrip_sunion_no_void(&self, data: &[u8]) -> Vec<u8>;
    fn roundtrip_uunion(&self, data: &[u8]) -> Vec<u8>;
    fn roundtrip_uunion_bool(&self, data: &[u8]) -> Vec<u8>;
    fn roundtrip_test_recursive(&self, data: &[u8]) -> Vec<u8>;
    fn roundtrip_bytestring(&self, data: &[u8]) -> Vec<u8>;
    fn roundtrip_sha256hash(&self, data: &[u8]) -> Vec<u8>;
    fn roundtrip_storage_delta(&self, data: &[u8]) -> Vec<u8>;
    fn roundtrip_storage_delta_vec(&self, data: &[u8]) -> Vec<u8>;
    fn roundtrip_i32(&self, data: &[u8]) -> Vec<u8>;
    fn roundtrip_u32(&self, data: &[u8]) -> Vec<u8>;
    fn roundtrip_library_exn_xdr(&self, data: &[u8]) -> Vec<u8>; // 13
    fn roundtrip_opaque_vector(&self, data: &[u8]) -> Vec<u8>;
    fn roundtrip_opaque_slice(&self, data: &[u8]) -> Vec<u8>;
}
