use super::interface::*;
use sdk::*;
use sdk::sys::SyscallErrorCode;
use groundhog_xdr::syscalls::SyscallWord;

#[derive(Default)]
pub struct NoSyscallTestContract;

// `groundhog_syscall::syscall_wrapper` is the single shared syscall-dispatch
// chokepoint: the SDK routes its own syscalls through it too, so the wasm
// linker sees exactly one `marmot.syscall` import (the meter caps function
// imports at 1). The thin `direct_syscall` shim keeps the existing call site
// (which passes `SyscallWord`-typed args) unchanged.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
unsafe fn direct_syscall(
    syscall_no: SyscallWord,
    arg1: SyscallWord,
    arg2: SyscallWord,
    arg3: SyscallWord,
    arg4: SyscallWord,
    arg5: SyscallWord,
    arg6: SyscallWord,
    arg7: SyscallWord,
) -> i32 {
    groundhog_syscall::syscall_wrapper(
        syscall_no, arg1, arg2, arg3, arg4, arg5, arg6, arg7,
    )
}

#[contract(storage = false)]
impl NoSyscallTest for NoSyscallTestContract {

    fn run_test(&self, syscall: u64) {
        let res = unsafe { direct_syscall(syscall as SyscallWord, 0, 0, 0, 0, 0, 0, 0) };
        if res != SyscallErrorCode::kSyscallErrorCodeNoSyscall.0 {
            panic!("syscall {} failed", syscall);
        }
    }
       
}

