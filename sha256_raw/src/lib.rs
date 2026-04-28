#![no_std]

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use sdk::*;

#[unsafe(export_name = "0")]
pub extern "C" fn calldata_len_raw_1() -> () {
    let len = get_calldata_len();
    let mut buf: Vec<u8> = vec![0; len as usize];
    let mut written_len = 0;
    let mut auth_token_count = 0;

    sdk::sys::raw_GetCalldata(buf.as_mut_slice(), &mut written_len, &mut [], &mut auth_token_count).unwrap();

    let mut hash: SHA256Hash = SHA256Hash::default();

    sdk::sys::raw_SHA256(buf.as_slice(), &mut hash).unwrap();
}

#[unsafe(export_name = "FFFFFFFFFFFFFFFF")]
pub extern "C" fn startup_null() -> () {
    ()
}
