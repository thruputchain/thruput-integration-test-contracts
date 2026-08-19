use super::interface::*;
use sdk::*;
use sdk::sys::SyscallErrorCode;
use groundhog_xdr::syscalls::SyscallWord;

// `groundhog_syscall::syscall_wrapper` is the single shared syscall-dispatch
// chokepoint: the SDK routes its own syscalls through it too, so a guest
// artifact carries exactly one syscall entry sequence. The thin `direct_syscall` shim keeps the existing call sites
// (which pass `SyscallWord`-typed args) unchanged.
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

const K_SYSCALL_NONNEGATIVE_INT_GET: SyscallWord =
    groundhog_xdr::SyscallNo::kNonnegativeIntGet.0 as SyscallWord;
const K_OOB_PTR: SyscallWord = SyscallWord::MAX - 1;

fn assert_fault(actual: i32) {
    assert_eq!(actual, SyscallErrorCode::kSyscallErrorCodeFault.0);
}

pub struct NnintOutputFaultTestContract;

#[contract(exports = "guest_exports")]
impl NnintOutputFaultTest for NnintOutputFaultTestContract {
    fn test_fault_valid_first() {
        let key = static_key!("fault_test_key");

        let mut current_set: i128 = 0xDEAD_BEEF;
        let mut current_set_opt: bool = true;

        let err_code = unsafe {
            direct_syscall(
                K_SYSCALL_NONNEGATIVE_INT_GET,
                &key as *const _ as SyscallWord,
                1, // force_load = true
                &mut current_set as *mut i128 as SyscallWord,
                &mut current_set_opt as *mut bool as SyscallWord,
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

    fn test_fault_invalid_first() {
        let key = static_key!("fault_test_key");

        let mut current_add: i128 = 0xCAFE;
        let mut current_set_opt: bool = true;

        let err_code = unsafe {
            direct_syscall(
                K_SYSCALL_NONNEGATIVE_INT_GET,
                &key as *const _ as SyscallWord,
                1, // force_load = true
                K_OOB_PTR, // invalid current_set pointer
                &mut current_set_opt as *mut bool as SyscallWord,
                &mut current_add as *mut i128 as SyscallWord,
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

    fn test_validation_failure() {
        let key = static_key!("fault_test_key");

        let mut current_set: i128 = 0xDEAD_BEEF;
        let mut current_set_opt: bool = true;
        let mut current_add: i128 = 0xCAFE;

        let err_code = unsafe {
            direct_syscall(
                K_SYSCALL_NONNEGATIVE_INT_GET,
                &key as *const _ as SyscallWord,
                2, // invalid boolean
                &mut current_set as *mut i128 as SyscallWord,
                &mut current_set_opt as *mut bool as SyscallWord,
                &mut current_add as *mut i128 as SyscallWord,
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
