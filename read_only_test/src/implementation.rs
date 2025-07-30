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
        let key : StorageKeyContractData = static_key!("test");
        let res = bytestring_set(&key, b"test");
        if expect_error {
            assert!(res.err().unwrap().is_code(SyscallErrorCode::kInvalidSyscallInput));
        } else {
            assert_eq!(res, Ok(()));
        }
    }
}