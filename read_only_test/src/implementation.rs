use sdk::*;
use crate::interface::*;

#[derive(Default)]
struct ReadOnlyTestContract;

#[contract(exports = "guest_exports", storage = "false")]
impl ReadOnlyTest for ReadOnlyTestContract {
    fn read_only_bytestring_set(&self, expect_error: bool) {
        let key : StorageKeyContractData = static_key!("test");
        let res = bytestring_set(&key, b"test");
        if expect_error {
            assert!(res.err().unwrap().is_code(SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput));
        } else {
            assert_eq!(res, Ok(()));
        }
    }
}
