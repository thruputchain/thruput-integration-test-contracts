//! An implementation of Fibonacci

use crate::INTERNAL_ALWAYS_FAULT_EXPORT_NUM;

use super::interface::SyscallMiscTests;
use groundhog_xdr::syscall_error::SyscallErrorCode;
use sdk::*;

#[derive(Default)]
struct SyscallMiscTestsContract;

// This implements the Fibonacci interface.  The `contract` macro provides
// an exported entrypoint for each method; these will be placed in a
// module with the specified name ("guest_exports").
#[contract(exports = "guest_exports", storage = "false")]
impl SyscallMiscTests for SyscallMiscTestsContract {
    fn test_get_return_before_return_buffer_set(&self) {

        let mut buffer = [0; 1024];
        let mut written_len = 0;
        let res = groundhog_xdr::syscalls::raw_GetReturnedBuffer(buffer.as_mut_slice(), &mut written_len);
        match res {
            Ok(_) => {
                panic!("get_return_buffer should have failed");
            }
            Err(error) => {
                assert_eq!(error, SyscallErrorCode::kSyscallErrorCodeNoReturnBufferAvailable.into());
            }
        }
    }

    fn test_get_return_after_fault(&self) {
        let addr = sdk::get_current_address().unwrap();
        let mut return_buffer_len: RuntimeMemoryLength = 0;
        let entrypoint = u64::from_be_bytes(INTERNAL_ALWAYS_FAULT_EXPORT_NUM.into_inner());

        let res = groundhog_xdr::syscalls::raw_Invoke(&addr, entrypoint, 0, &[], 0, &mut return_buffer_len);
        match res {
            Ok(_) => {
                panic!("invoke should have failed");
            }
            Err(error) => {
                if error != SyscallErrorCode::kSyscallErrorCodeDeterministicWasm.into() {
                    panic!("invoke should have failed with deterministic wasm error");
                }
            }
        }

        let mut buffer = [0; 1024];
        let mut written_len = 0;
        let res = groundhog_xdr::syscalls::raw_GetReturnedBuffer(buffer.as_mut_slice(), &mut written_len);
        match res {
            Ok(_) => {
                panic!("get_return_buffer should have failed");
            }
            Err(error) => {
                assert_eq!(error, SyscallErrorCode::kSyscallErrorCodeNoReturnBufferAvailable.into());
            }
        }

    }

    fn internal_always_fault(&self) {
        panic!("internal_always_fault");
    }
}

