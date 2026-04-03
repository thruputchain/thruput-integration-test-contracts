use super::interface::*;
use sdk::*;
use sdk::sys::SyscallErrorCode;

#[derive(Default)]
pub struct NoSyscallTestContract;

#[link(wasm_import_module = "marmot")]
unsafe extern "C" {
    #[link_name = "syscall"]
    fn syscall_wrapper_impl(
        syscall_no: u64,
        arg1: u64,
        arg2: u64,
        arg3: u64,
        arg4: u64,
        arg5: u64,
        arg6: u64,
        arg7: u64,
    ) -> i32;
}

#[contract(storage = false)]
impl NoSyscallTest for NoSyscallTestContract {

    fn run_test(&self, syscall: u64) {
        let res = unsafe { syscall_wrapper_impl(syscall, 0, 0, 0, 0, 0, 0, 0) };
        if res != SyscallErrorCode::kSyscallErrorCodeNoSyscall.0 {
            panic!("syscall {} failed", syscall);
        }
    }
       
}

