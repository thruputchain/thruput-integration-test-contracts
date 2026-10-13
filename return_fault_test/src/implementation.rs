use crate::{Client, ReturnFaultTest};
use thruput_sdk::*;

pub struct ReturnFaultTestContract;

#[contract]
impl ReturnFaultTest for ReturnFaultTestContract {

    fn run_test(callee_addr: &ContractAddress) {
        let client = Client::new(callee_addr);

        match client.try_test_inner() {
            Ok(_) => {
                panic!("test_inner should have failed");
            }
            Err(CallError::Fault(_)) => {
                // Expected: the inner frame ran and faulted on the
                // out-of-bounds Return.
            }
            Err(e) =>
                panic!("Expected the call to fault: {e:?}"),
        }
    }

    fn test_inner() {
        let ptr: *const u8 = 65536 as *const u8;
        let slice = unsafe { core::slice::from_raw_parts(ptr, 1) as &[u8] };

        // This function is expected to fail, so returning success is a test
        // failure: if ConsoleLog accepts the out-of-bounds slice, bail out
        // (the inner frame returns normally and run_test will catch it).
        if thruput_sdk::sys::raw_ConsoleLog(slice).is_ok() {
            return;
        }

        // this should fail, because slice is out of bounds
        thruput_sdk::sys::raw_Return(slice, false, &[]);
    }
}
