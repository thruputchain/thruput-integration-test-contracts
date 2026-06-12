use crate::{ReturnFaultTest, Client, ClientError};
use sdk::*;

#[sdk::storage]
pub struct ReturnFaultTestContract;

#[contract]
impl ReturnFaultTest for ReturnFaultTestContract {

    fn run_test(&self) {
        let addr = sdk::get_current_address();
        let client = Client::new(&addr);

        match client.try_test_inner() {
            Ok(_) => {
                panic!("test_inner should have failed");
            }
            Err(ClientError::Fault(_)) => {
                // Expected: inner contract faulted due to out-of-bounds memory access
            }
            Err(e) =>
                panic!("Expected the call to fault: {e:?}"),
        }
    }

    fn test_inner(&self) {
        let ptr: *const u8 = 65536 as *const u8;
        let slice = unsafe { core::slice::from_raw_parts(ptr, 1) as &[u8] };

        // This function is expected to fail, so returning success is a test
        // failure: if ConsoleLog accepts the out-of-bounds slice, bail out
        // (the inner frame returns normally and run_test will catch it).
        if sdk::sys::raw_ConsoleLog(slice).is_ok() {
            return;
        }

        // this should fail, because slice is out of bounds
        sdk::sys::raw_Return(slice, false, &[]);
    }
}
