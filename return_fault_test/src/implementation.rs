use crate::{ReturnFaultTest, Client};
use sdk::sys::SyscallErrorCode;
use sdk::*;

#[derive(Default)]
pub struct ReturnFaultTestContract;

#[contract(storage = false)]
impl ReturnFaultTest for ReturnFaultTestContract {

    fn run_test(&self) {
        let addr = sdk::get_current_address();
        let mut client = Client::new(&addr);

        match client.try_test_inner() {
            Ok(_) => {
                panic!("test_inner should have failed");
            }
            Err(sdk::call::CallError::Fault(_)) => {
                // Expected: inner contract faulted due to out-of-bounds memory access
            }
            Err(e) =>
                panic!("Expected the call to fault: {e:?}"),
        }
    }

    fn test_inner(&self) {
        let ptr: *const u8 = 65536 as *const u8;
        let slice = unsafe { core::slice::from_raw_parts(ptr, 1) as &[u8] };

        // this function is expected to fail
        // so returning success is a test failure
        match sdk::sys::raw_ConsoleLog(slice) {
            Ok(_) => {
                // Unexpected
                return;
            }
            Err(syserr) => {
                if syserr.is_code(SyscallErrorCode::kSyscallErrorCodeDeterministicWasm) {
                    // Unexpected
                    return;
                }
            }
        }

        // this should fail, because slice is out of bounds
        sdk::sys::raw_Return(slice, false);
    }
}
