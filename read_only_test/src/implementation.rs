use crate::interface::*;
use sdk::*;

#[derive(Default)]
struct ReadOnlyTestContract;

#[contract(exports = "guest_exports", storage = "false")]
impl ReadOnlyTest for ReadOnlyTestContract {
    fn read_only_bytestring_set(&self, expect_error: bool) {
        let key: StorageKeyContractData = static_key!("test");
        let res = bytestring_set(&key, b"test");
        sdk::debug_log!("res: {:?}", res);
        if expect_error {
            assert!(res.err().unwrap().is_code(
                SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput
            ));
        } else {
            assert_eq!(res, Ok(()));
        }
    }

    fn read_only_log_event(&self, expect_error: bool) {
        let data = b"test event";
        let res = groundhog_xdr::syscalls::raw_LogEvent(None, None, None, data);
        sdk::debug_log!("log_event res: {:?}", res);
        if expect_error {
            assert!(res.err().unwrap().is_code(
                SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput
            ));
        } else {
            assert_eq!(res, Ok(()));
        }
    }
}
