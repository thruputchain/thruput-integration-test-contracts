use alloc::vec::Vec;
use sdk::*;

#[contract(client = Client)]
pub trait CxsRoundtrip {
    fn roundtrip_foo(data: &[u8]) -> Vec<u8>; // 0
    fn roundtrip_senum(data: &[u8]) -> Vec<u8>; // 1
    fn roundtrip_sunion(data: &[u8]) -> Vec<u8>; // 2
    fn roundtrip_sunion_no_void(data: &[u8]) -> Vec<u8>; // 3
    fn roundtrip_uunion(data: &[u8]) -> Vec<u8>; // 4
    fn roundtrip_uunion_bool(data: &[u8]) -> Vec<u8>; // 5
    fn roundtrip_test_recursive(data: &[u8]) -> Vec<u8>; // 6
    fn roundtrip_bytestring(data: &[u8]) -> Vec<u8>; // 7
    fn roundtrip_sha256hash(data: &[u8]) -> Vec<u8>; // 8
    fn roundtrip_storage_delta(data: &[u8]) -> Vec<u8>; // 9
    fn roundtrip_storage_delta_vec(data: &[u8]) -> Vec<u8>; // 10
    fn roundtrip_i32(data: &[u8]) -> Vec<u8>; // 11
    fn roundtrip_u32(data: &[u8]) -> Vec<u8>; // 12
    fn roundtrip_library_exn_xdr(data: &[u8]) -> Vec<u8>; // 13
    fn roundtrip_opaque_vector(data: &[u8]) -> Vec<u8>; // 14
    fn roundtrip_opaque_slice(data: &[u8]) -> Vec<u8>; // 15
}
