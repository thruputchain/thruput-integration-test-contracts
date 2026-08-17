use super::interface::*;
use sdk::*;
use groundhog_xdr::syscalls::SyscallWord;

// `groundhog_syscall::syscall_wrapper` is the single shared syscall-dispatch
// chokepoint: the SDK routes its own syscalls through it too, so the wasm
// linker sees exactly one `marmot.syscall` import (the meter caps function
// imports at 1). The thin `direct_syscall` shim keeps the existing call sites
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
    sdk::sys::SyscallNo::kNonnegativeIntGet.0 as SyscallWord;
const K_SYSCALL_GET_CALLDATA: SyscallWord = sdk::sys::SyscallNo::kGetCalldata.0 as SyscallWord;
const K_SYSCALL_LOG_EVENT: SyscallWord = sdk::sys::SyscallNo::kLogEvent.0 as SyscallWord;

const K_FAULT: i32 = 1; // SyscallErrorCode::kSyscallErrorCodeFault

fn assert_fault(actual: i32) {
    assert_eq!(actual, K_FAULT);
}

fn assert_not_fault(actual: i32) {
    assert_ne!(actual, K_FAULT);
}

pub struct OutputAliasTestContract;

#[contract(exports = "guest_exports")]
impl OutputAliasTest for OutputAliasTestContract {
    // Point both i128 output pointers (current_set and current_add) at the
    // exact same address. This is exact overlap and must fault.
    fn test_nnint_exact_overlap() {
        let key = static_key!("alias_test_key");
        let mut shared: i128 = 0xDEAD_BEEF;
        let mut opt: bool = false;

        let err_code = unsafe {
            direct_syscall(
                K_SYSCALL_NONNEGATIVE_INT_GET,
                &key as *const _ as SyscallWord,
                1, // force_load = true
                &mut shared as *mut i128 as SyscallWord,  // current_set
                &mut opt as *mut bool as SyscallWord,      // current_set_opt
                &mut shared as *mut i128 as SyscallWord,  // current_add (same as current_set!)
                0,
                0
            )
        };

        assert_fault(err_code);
        // Outputs must not have been written (disarmed on alias fault).
        assert_eq!(shared, 0xDEAD_BEEF, "aliased output was overwritten");
    }

    // Place current_set at buf+0 and current_add at buf+8.
    // i128 is 16 bytes, so [0,16) and [8,24) partially overlap.
    fn test_nnint_partial_overlap() {
        let key = static_key!("alias_test_key");
        let mut buf = [0xAAu8; 32];
        let mut opt: bool = false;

        let current_set_ptr = buf.as_mut_ptr() as u32;
        let current_add_ptr = (buf.as_mut_ptr() as u32) + 8;

        let err_code = unsafe {
            direct_syscall(
                K_SYSCALL_NONNEGATIVE_INT_GET,
                &key as *const _ as SyscallWord,
                1, // force_load = true
                current_set_ptr,                   // current_set at buf[0..16]
                &mut opt as *mut bool as SyscallWord,      // current_set_opt
                current_add_ptr,                   // current_add at buf[8..24] (overlaps!)
                0,
                0
            )
        };

        assert_fault(err_code);
        // Verify the buffer was not modified.
        assert_eq!(buf, [0xAAu8; 32], "overlapping output was overwritten");
    }

    // Normal non-overlapping outputs. Should succeed (no fault).
    fn test_nnint_no_overlap() {
        let key = static_key!("alias_test_key");
        let mut current_set: i128 = 0;
        let mut current_set_opt: bool = false;
        let mut current_add: i128 = 0;

        let err_code = unsafe {
            direct_syscall(
                K_SYSCALL_NONNEGATIVE_INT_GET,
                &key as *const _ as SyscallWord,
                1, // force_load = true
                &mut current_set as *mut i128 as SyscallWord,
                &mut current_set_opt as *mut bool as SyscallWord,
                &mut current_add as *mut i128 as SyscallWord,
                0,
                0
            )
        };

        assert_not_fault(err_code);
    }

    // Place the output_written_len pointer inside the output span.
    // GetCalldata: (span_ptr, span_len, written_len_ptr, ...)
    // If written_len_ptr falls within [span_ptr, span_ptr+span_len),
    // it's span-writable overlap and must fault.
    fn test_calldata_span_writable_overlap() {
        let mut buf = [0xBBu8; 64];
        // Place output_written_len inside the span
        let span_ptr = buf.as_mut_ptr() as u32;
        let span_len: SyscallWord = 64;
        let written_len_ptr = span_ptr + 16; // inside [span_ptr, span_ptr+64)

        let err_code = unsafe {
            direct_syscall(
                K_SYSCALL_GET_CALLDATA,
                span_ptr,
                span_len,
                written_len_ptr, // overlaps with span
                0,
                0,
                0,
                0
            )
        };

        assert_fault(err_code);
        assert_eq!(buf, [0xBBu8; 64], "overlapping output was overwritten");
    }

    // Non-overlapping span and writable output. Should succeed.
    fn test_calldata_no_overlap() {
        let mut buf = [0u8; 64];
        let mut written_len: u32 = 0xDEAD;

        let err_code = unsafe {
            direct_syscall(
                K_SYSCALL_GET_CALLDATA,
                buf.as_mut_ptr() as SyscallWord,
                buf.len() as SyscallWord,
                &mut written_len as *mut u32 as SyscallWord,
                0,
                0,
                0,
                0
            )
        };

        assert_not_fault(err_code);
    }

    // Two optional input pointers at the same address must fault.
    // LogEvent: (topic0_ptr, topic1_ptr, topic2_ptr, data_ptr, data_len, 0, 0)
    fn test_log_event_input_input_alias() {
        let topic = [0u8; 32];
        let topic_ptr = &topic as *const _ as u32;

        // Pass the same topic pointer for topic0 and topic2 (topic1=None).
        let err_code = unsafe {
            direct_syscall(
                K_SYSCALL_LOG_EVENT,
                topic_ptr,         // topic0
                SyscallWord::MAX,  // topic1 = None
                topic_ptr,         // topic2 = same address as topic0!
                0,                 // data ptr (empty)
                0,                 // data len
                0,
                0
            )
        };

        assert_fault(err_code);
    }

    // Zero-length span aliasing under the half-open overlap predicate: a
    // zero-length data span whose pointer lands strictly inside topic0's
    // 32-byte range faults; at the topic's exact start or one-past-end it
    // does not alias and the event logs.
    fn test_log_event_zero_len_data_alias() {
        let topic = [0u8; 32];
        let topic_ptr = &topic as *const _ as u32;

        let inside = unsafe {
            direct_syscall(
                K_SYSCALL_LOG_EVENT,
                topic_ptr,
                SyscallWord::MAX, // topic1 = None
                SyscallWord::MAX, // topic2 = None
                topic_ptr + 8,    // zero-len data inside topic0's range
                0,
                0,
                0,
            )
        };
        assert_fault(inside);

        let at_start = unsafe {
            direct_syscall(
                K_SYSCALL_LOG_EVENT,
                topic_ptr,
                SyscallWord::MAX,
                SyscallWord::MAX,
                topic_ptr, // zero-len data at topic0's start: no alias
                0,
                0,
                0,
            )
        };
        assert_not_fault(at_start);

        let at_end = unsafe {
            direct_syscall(
                K_SYSCALL_LOG_EVENT,
                topic_ptr,
                SyscallWord::MAX,
                SyscallWord::MAX,
                topic_ptr + 32, // zero-len data one-past topic0: no alias
                0,
                0,
                0,
            )
        };
        assert_not_fault(at_end);
    }

    fn test_nnint_input_alias_output() {

        let mut buf = [0xAAu8; 64];
        let mut opt: bool = false;

        let current_set_ptr = buf.as_mut_ptr() as u32;
        let current_add_ptr = (buf.as_mut_ptr() as u32) + 32;
        let key_ptr = buf.as_mut_ptr() as u32;

        let err_code = unsafe {
            direct_syscall(
                K_SYSCALL_NONNEGATIVE_INT_GET,
                key_ptr,
                1, // force_load = true
                current_set_ptr,                   // current_set at buf[0..16] (overlaps input key)
                &mut opt as *mut bool as SyscallWord,      // current_set_opt
                current_add_ptr,                   // current_add at buf[32..48] (no overlap)
                0,
                0
            )
        };

        assert_fault(err_code);
        // Verify the buffer was not modified.
        assert_eq!(buf, [0xAAu8; 64], "overlapping output was overwritten");

    }
}
