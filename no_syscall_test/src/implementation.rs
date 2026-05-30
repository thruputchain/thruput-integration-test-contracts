use super::interface::*;
use sdk::*;
use sdk::sys::SyscallErrorCode;

#[derive(Default)]
pub struct NoSyscallTestContract;

// The raw `marmot.syscall` import must match the host ABI exactly: each slot is
// a `SyscallWord` (u32 on wasm32, u64 on native). Declaring it with the SDK's
// `SyscallWord` keeps this import byte-identical to the SDK's own `syscall`
// import so the wasm linker merges them into a single import (the meter caps
// function imports at 1). The `u64`-taking `syscall_wrapper_impl` shim below
// lets the existing call site keep passing plain `u64`s.
#[link(wasm_import_module = "marmot")]
unsafe extern "C" {
    #[link_name = "syscall"]
    fn raw_marmot_syscall(
        syscall_no: sdk::sys::SyscallWord,
        arg1: sdk::sys::SyscallWord,
        arg2: sdk::sys::SyscallWord,
        arg3: sdk::sys::SyscallWord,
        arg4: sdk::sys::SyscallWord,
        arg5: sdk::sys::SyscallWord,
        arg6: sdk::sys::SyscallWord,
        arg7: sdk::sys::SyscallWord,
    ) -> i32;
}

#[inline(always)]
#[allow(clippy::too_many_arguments)]
unsafe fn syscall_wrapper_impl(
    syscall_no: u64,
    arg1: u64,
    arg2: u64,
    arg3: u64,
    arg4: u64,
    arg5: u64,
    arg6: u64,
    arg7: u64,
) -> i32 {
    unsafe {
        raw_marmot_syscall(
            syscall_no as sdk::sys::SyscallWord,
            arg1 as sdk::sys::SyscallWord,
            arg2 as sdk::sys::SyscallWord,
            arg3 as sdk::sys::SyscallWord,
            arg4 as sdk::sys::SyscallWord,
            arg5 as sdk::sys::SyscallWord,
            arg6 as sdk::sys::SyscallWord,
            arg7 as sdk::sys::SyscallWord,
        )
    }
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

