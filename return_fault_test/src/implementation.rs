use super::interface::*;
use sdk::*;

pub use sdk::contract_result::*;
use sdk::sys::raw_Return;

#[derive(Default)]
pub struct ReturnFaultTestContract;

#[contract(exports = "guest_exports", storage = "false")]
impl ReturnFaultTest for ReturnFaultTestContract {

    fn run_test(&self) {
        let addr = sdk::get_current_address().unwrap();
        let mut client = Client::new(&addr);
        let expected_err_code : SysErr = SysErr(SyscallErrorCode::kSyscallErrorCodeDeterministicWasm as i32);
        match client.test_inner() {
            Ok(_) => {
                panic!("test_inner should have failed");
            }
            Err(e) => {
                match e {
                   InvokeError::Syscall(x) => {
                    assert_eq!(x, expected_err_code);
                    return;
                   },
                   _ => {
                    panic!("test_inner should have failed: {:?}", e);
                   }
                }
            }
        }
    }

    fn test_inner(&self) {
        let ptr: *const u8 = 65536 as *const u8;
        let slice = unsafe { core::slice::from_raw_parts(ptr, 1) as &[u8] };
        // this function is expected to fail
        // so returning success is a test failure
        let fault_res = sdk::log(slice);
        if fault_res.is_ok() {
            return;
        }
        if !fault_res.err().unwrap().is_code(SyscallErrorCode::kSyscallErrorCodeFault) {
            return;
        }
        // this should fail, because slice is out of bounds
        raw_Return(slice, false);
    }
}

