//! An implementation of Fibonacci

use crate::interface::TxsRoundtrip;
use alloc::format;
use alloc::vec::Vec;
use sdk::*;
use sdk::__xdrs::txs;
use sdk::{ReadXDR, WriteXDR};

#[sdk::storage]
pub struct TxsRoundtripContract;

fn roundtrip<'a, T>(data: &'a [u8]) -> Vec<u8>
where
    T: ReadXDR<'a> + WriteXDR + core::fmt::Debug,
{
    log(format!("roundtrip: {:02X?}", data).as_bytes());
    let parsed: T = txs::decode(data).unwrap_or_else(|e| {
        log(format!("decode error: {:?}", e).as_bytes());
        panic!("decode error")
    });
    log(format!("parsed as {:?}", parsed).as_bytes());
    let out = txs::encode(&parsed).unwrap();
    log(format!("out: {:02X?}", out).as_bytes());
    out
}

// Macro to generate all roundtrip implementations
macro_rules! impl_roundtrip_methods {
    ($($method_name:ident: $type_name:ty),* $(,)?) => {
        #[contract]
        impl TxsRoundtrip for TxsRoundtripContract {
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
    roundtrip_storage_err_xdr: sdk::storage::error::StorageError,
    roundtrip_opaque_vector: Vec<u8>,
    roundtrip_opaque_slice: &[u8],
}

#[unsafe(export_name = "FFFFFFFFFFFFFFFF")]
pub extern "C" fn startup_null() -> () {
    ()
}
