use super::interface::*;
use sdk::*;
use sdk::sys::SyscallErrorCode;

// The raw `marmot.syscall` import must match the host ABI exactly: each slot is
// a `SyscallWord` (u32 on wasm32, u64 on native). Declaring it with the SDK's
// `SyscallWord` keeps this import byte-identical to the SDK's own `syscall`
// import so the wasm linker merges them into a single import (the meter caps
// function imports at 1). The `u64`-taking `direct_syscall` shim below lets the
// existing call sites keep passing plain `u64`s.
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
unsafe fn direct_syscall(
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

const K_SYSCALL_NONNEGATIVE_INT_GET: u64 = 16;
const K_OOB_PTR: u64 = u64::MAX - 1;

fn assert_fault(actual: i32) {
    assert_eq!(actual, SyscallErrorCode::kSyscallErrorCodeFault.0);
}

#[derive(Default)]
pub struct NnintOutputFaultTestContract;

#[contract(exports = "guest_exports", storage = false)]
impl NnintOutputFaultTest for NnintOutputFaultTestContract {
    fn test_fault_valid_first(&self) {
        let key = static_key!("fault_test_key");

        let mut current_set: i128 = 0xDEAD_BEEF;
        let mut current_set_opt: bool = true;

        let err_code = unsafe {
            direct_syscall(
                K_SYSCALL_NONNEGATIVE_INT_GET,
                &key as *const _ as u64,
                1, // force_load = true
                &mut current_set as *mut i128 as u64,
                &mut current_set_opt as *mut bool as u64,
                K_OOB_PTR, // invalid current_add pointer
                0,
                0
            )
        };

        assert_fault(err_code);
        assert_eq!(
            current_set, 0xDEAD_BEEF,
            "current_set was overwritten by faulting syscall"
        );
        assert_eq!(
            current_set_opt, true,
            "current_set_opt was overwritten by faulting syscall"
        );
    }

    fn test_fault_invalid_first(&self) {
        let key = static_key!("fault_test_key");

        let mut current_add: i128 = 0xCAFE;
        let mut current_set_opt: bool = true;

        let err_code = unsafe {
            direct_syscall(
                K_SYSCALL_NONNEGATIVE_INT_GET,
                &key as *const _ as u64,
                1, // force_load = true
                K_OOB_PTR, // invalid current_set pointer
                &mut current_set_opt as *mut bool as u64,
                &mut current_add as *mut i128 as u64,
                0,
                0
            )
        };

        assert_fault(err_code);
        assert_eq!(
            current_set_opt, true,
            "current_set_opt was overwritten by faulting syscall"
        );
        assert_eq!(
            current_add, 0xCAFE,
            "current_add was overwritten by faulting syscall"
        );
    }

    fn test_validation_failure(&self) {
        let key = static_key!("fault_test_key");

        let mut current_set: i128 = 0xDEAD_BEEF;
        let mut current_set_opt: bool = true;
        let mut current_add: i128 = 0xCAFE;

        let err_code = unsafe {
            direct_syscall(
                K_SYSCALL_NONNEGATIVE_INT_GET,
                &key as *const _ as u64,
                2, // invalid boolean
                &mut current_set as *mut i128 as u64,
                &mut current_set_opt as *mut bool as u64,
                &mut current_add as *mut i128 as u64,
                0,
                0
            )
        };
        assert_fault(err_code);
        assert_eq!(
            current_set_opt, true,
            "current_set_opt was overwritten by faulting syscall"
        );
        assert_eq!(
            current_set, 0xDEAD_BEEF,
            "current_set was overwritten by faulting syscall"
        );
        assert_eq!(
            current_add, 0xCAFE,
            "current_add was overwritten by faulting syscall"
        );
    }
}
