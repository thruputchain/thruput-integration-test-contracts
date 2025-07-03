#![no_std]

extern crate alloc;

use sdk::storage_prelude::{get_calldata, get_calldata_len};
use sdk::storage_prelude::SHA256Hash;
use alloc::vec::Vec;
use alloc::vec;

#[unsafe(export_name = "0")]
pub extern "C" fn calldata_len_raw_1() -> () {
    let len = get_calldata_len().unwrap();
    let mut buf : Vec<u8> = vec![0; len as usize];
    let mut written_len = 0;
   
    sdk::sys::raw_GetCalldata(buf.as_mut_slice(), &mut written_len).unwrap();

    let mut hash : SHA256Hash = SHA256Hash::default();

    sdk::sys::raw_SHA256(buf.as_slice(), &mut hash).unwrap();
}

#[unsafe(export_name = "FFFFFFFFFFFFFFFF")]
pub extern "C" fn startup_null() -> () {
    ()
}
