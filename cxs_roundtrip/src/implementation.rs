//! An implementation of Fibonacci

use super::interface::CxsRoundtrip;
use sdk::prelude::*;

use alloc::vec::Vec;

#[derive(Default)]
struct CxsRoundtripContract;

use xdrs::serialize_cxs;
use xdrs::{ReadXDROwned, WriteXDR};

use sdk::prelude::log;
use alloc::format;

fn roundtrip<T>(data: &[u8]) -> Vec<u8> 
where
    T: ReadXDROwned + WriteXDR + core::fmt::Debug,
{
    log(format!("roundtrip: {:02X?}", data).as_bytes()).unwrap();
    let parsed: T = serialize_cxs::decode(data).unwrap_or_else(|e| {
        log(format!("decode error: {:?}", e).as_bytes()).unwrap();
        panic!("decode error")
    });
    log(format!("parsed as {:?}", parsed).as_bytes()).unwrap();
    let out = serialize_cxs::encode(&parsed).unwrap();
    log(format!("out: {:02X?}", out).as_bytes()).unwrap();
    out
}

// Macro to generate all roundtrip implementations
macro_rules! impl_roundtrip_methods {
    ($($method_name:ident: $type_name:ty),* $(,)?) => {
        #[contract(exports = "guest_exports", storage = "false")]
        impl CxsRoundtrip for CxsRoundtripContract {
            $(
                fn $method_name(&self, data: &[u8]) -> Vec<u8> {
                    roundtrip::<$type_name>(data)
                }
            )*
        }
    };
}

// Generate all implementations
impl_roundtrip_methods! {
    roundtrip_foo: crate::roundtrip::Foo,
    roundtrip_senum: crate::roundtrip::senum,
    roundtrip_sunion: crate::roundtrip::sunion,
    roundtrip_sunion_no_void: crate::roundtrip::sunion_no_void,
    roundtrip_uunion: crate::roundtrip::uunion,
    roundtrip_uunion_bool: crate::roundtrip::uunion_bool,
    roundtrip_test_recursive: crate::roundtrip::test_recursive,
    roundtrip_bytestring: crate::roundtrip::Bytestring,
    roundtrip_sha256hash: crate::roundtrip::SHA256Hash,
    roundtrip_storage_delta: crate::roundtrip::StorageDelta,
    roundtrip_storage_delta_vec: crate::roundtrip::StorageDeltaVec,
    roundtrip_i32: i32,
    roundtrip_u32: u32,
    roundtrip_error_xdr: sdk::contract_result::Error,
    roundtrip_contract_error_xdr: sdk::contract_result::ContractError,
    roundtrip_storage_err_xdr: sdk::storage::error::Error,
    roundtrip_result_xdr: sdk::contract_result::Result<i32>,
}

#[unsafe(export_name = "FFFFFFFFFFFFFFFF")]
pub extern "C" fn startup_null() -> () {
    ()
}

