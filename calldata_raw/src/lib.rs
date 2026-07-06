#![no_std]

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use sdk::{fetch_calldata_and_auth_tokens, get_calldata_len};

pub fn calldata_len(count: u32) {
    for _ in 0..count {
        let _ = get_calldata_len();
    }
}

#[unsafe(export_name = "0")]
pub extern "C" fn calldata_len_raw_1() -> () {
    calldata_len(1);
}

#[unsafe(export_name = "1")]
pub extern "C" fn calldata_len_raw_100() -> () {
    calldata_len(100);
}

#[unsafe(export_name = "2")]
pub extern "C" fn calldata_len_raw_1000() -> () {
    calldata_len(1000);
}

pub fn calldata(count: u32) {
    let mut len: usize = 0;

    for _ in 0..count {
        // Fresh buffers per iteration: each fetch is independent, matching
        // the pre-existing semantics of one fetch per call.
        let mut buf = sdk::internal::new_scratch();
        let mut auth_ids = sdk::contract_op::AuthTokenIdBuf::new();
        fetch_calldata_and_auth_tokens(&mut buf, &mut auth_ids);
        let x_len = buf.len();
        if len > 0 {
            assert_eq!(x_len, len);
        }
        len = x_len;
    }
}

#[unsafe(export_name = "10")]
pub extern "C" fn calldata_1() -> () {
    calldata(1);
}

#[unsafe(export_name = "11")]
pub extern "C" fn calldata_100() -> () {
    calldata(100);
}

#[unsafe(export_name = "12")]
pub extern "C" fn calldata_1000() -> () {
    calldata(1000);
}

pub fn calldata_raw(count: u32) {
    let len = get_calldata_len();
    let mut buf: Vec<u8> = vec![0; len as usize];
    let mut written_len = 0;
    let mut auth_token_count = 0;
    for _ in 0..count {
        sdk::sys::raw_GetCalldata(buf.as_mut_slice(), &mut written_len, &mut [], &mut auth_token_count)
            .unwrap();
    }
}

#[unsafe(export_name = "20")]
pub extern "C" fn calldata_raw_1() -> () {
    calldata_raw(1);
}

#[unsafe(export_name = "21")]
pub extern "C" fn calldata_raw_100() -> () {
    calldata_raw(100);
}

#[unsafe(export_name = "22")]
pub extern "C" fn calldata_raw_1000() -> () {
    calldata_raw(1000);
}

#[unsafe(export_name = "FFFFFFFFFFFFFFFF")]
pub extern "C" fn startup_null() -> () {
    ()
}
