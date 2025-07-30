use sdk::prelude::*;

pub use sdk::prelude::{Error, Result};
use sdk::macros::static_key;

use crate::interface::ReadOnlyTest;

use alloc::format;

#[derive(Default)]
struct ReadOnlyTestContract;

#[contract(exports = "guest_exports", storage = "false")]
impl ReadOnlyTest for ReadOnlyTestContract {
    fn read_only_bytestring_set(&self, expect_error: bool) {
        log(format!("read_only_bytestring_set: {:?}", expect_error).as_bytes()).unwrap();
       /*let key : StorageKeyContractData = static_key!("test");
        let res = bytestring_set(&key, b"test");
        if expect_error {
            assert!(res.err().unwrap_or_else(|| {
                log(format!("expected error").as_bytes()).unwrap();
                panic!("expected error")
            }).is_code(SyscallErrorCode::kInvalidSyscallInput));
        } else {
            assert_eq!(res, Ok(()));
        }
        log(format!("done read_only_bytestring_set: {:?}", expect_error).as_bytes()).unwrap(); */
    }
}