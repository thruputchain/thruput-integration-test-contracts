use super::interface::*;
use sdk::*;
use sdk::sys::SyscallErrorCode;
use groundhog_xdr::syscalls::SyscallWord;

#[derive(Default)]
pub struct NoSyscallTestContract;

#[link(wasm_import_module = "marmot")]
unsafe extern "C" {
    #[link_name = "syscall"]
    fn syscall_wrapper_impl(
        syscall_no: SyscallWord,
        arg1: SyscallWord,
        arg2: SyscallWord,
        arg3: SyscallWord,
        arg4: SyscallWord,
        arg5: SyscallWord,
        arg6: SyscallWord,
        arg7: SyscallWord,
    ) -> i32;
}

#[contract(storage = false)]
impl NoSyscallTest for NoSyscallTestContract {

    fn run_test(&self, syscall: u64) {
        let res = unsafe { syscall_wrapper_impl(syscall as SyscallWord, 0, 0, 0, 0, 0, 0, 0) };
        if res != SyscallErrorCode::kSyscallErrorCodeNoSyscall.0 {
            panic!("syscall {} failed", syscall);
        }
    }
       
}

