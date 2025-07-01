#![no_std]

extern crate alloc;

use sdk::storage_prelude::{get_calldata, get_calldata_len};
use alloc::vec::Vec;
use alloc::vec;

pub fn calldata_len(count: u32) {
    for _ in 0..count {
        let _ = get_calldata_len().unwrap();
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
    //let len = get_calldata_len().unwrap();
    //let mut buf : Vec<u8> = vec![0; len as usize];
    //let mut written_len = 0;

    for _ in 0..count {
        let _ = get_calldata().unwrap();
       // sdk::sys::raw_GetCalldata(buf.as_mut_slice(), &mut written_len).unwrap();
    }
}

#[unsafe(export_name = "10")]
pub extern "C" fn calldata_raw_1() -> () {
    calldata(1);
}

#[unsafe(export_name = "11")]
pub extern "C" fn calldata_raw_100() -> () {
    calldata(100);
}

#[unsafe(export_name = "12")]
pub extern "C" fn calldata_raw_1000() -> () {
    calldata(1000);
}

#[unsafe(export_name = "FFFFFFFFFFFFFFFF")]
pub extern "C" fn startup_null() -> () {
    ()
}
