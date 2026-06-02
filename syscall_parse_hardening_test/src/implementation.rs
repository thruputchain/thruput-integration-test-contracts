use super::interface::*;
use sdk::*;
use sdk::sys::SyscallErrorCode;
use groundhog_xdr::syscalls::SyscallWord;

// `groundhog_syscall::syscall_wrapper` is the single shared syscall-dispatch
// chokepoint: the SDK routes its own syscalls through it too, so the wasm
// linker sees exactly one `marmot.syscall` import (the meter caps function
// imports at 1). The thin `direct_syscall` shim keeps the existing call sites
// (which pass `SyscallWord`-typed args) unchanged.
//
// Note: on wasm32 these slots are 32 bits, so the 64-bit "non-canonical -1"
// values exercised below are truncated by the host ABI; the full 64-bit width
// is still exercised on the native test path, where `SyscallWord` is `u64`.
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

const K_SYSCALL_GET_DEPLOY_ADDRESS: SyscallWord =
    groundhog_xdr::SyscallNo::kGetDeployAddress.0 as SyscallWord;
const K_SYSCALL_VERIFY_SIGNATURE: SyscallWord =
    groundhog_xdr::SyscallNo::kVerifySignature.0 as SyscallWord;
const K_SYSCALL_CHANGE_PUBLIC_KEY_REGISTRATION: SyscallWord =
    groundhog_xdr::SyscallNo::kChangePublicKeyRegistration.0 as SyscallWord;
// An offset that is out of bounds for the contract's linear memory, but is not
// the all-ones optional-pointer null sentinel.
const K_OOB_PTR: SyscallWord = SyscallWord::MAX - 1;

fn assert_code(actual: i32, expected: SyscallErrorCode) {
    assert_eq!(actual, expected.0);
}

#[derive(Default)]
pub struct SyscallParseHardeningTestContract;

#[contract(exports = "guest_exports", storage = false)]
impl SyscallParseHardeningTest for SyscallParseHardeningTestContract {
    fn test_verify_signature_noncanonical_scheme_fault(&self) {
        let signature = [0u8; 64];
        let public_key = [0u8; 32];
        let message = [0u8; 1];

        // Under the wasm32 ABI each value slot is delivered as a bare 32-bit
        // word, so every bit pattern is a valid i32 scheme: there is no
        // "non-canonical" (improperly sign-extended) or out-of-range encoding to
        // reject at argument-parsing time, and a value wider than 32 bits cannot
        // even be expressed in an i32 wasm argument. The scheme is therefore
        // always parsed successfully; an unknown scheme (here -1) is rejected by
        // the inner syscall with kSyscallErrorCodeInvalidSyscallInput.
        let unknown_scheme = unsafe {
            direct_syscall(
                K_SYSCALL_VERIFY_SIGNATURE,
                SyscallWord::MAX, // -1: a valid i32 bit pattern, but unknown scheme
                signature.as_ptr() as SyscallWord,
                signature.len() as SyscallWord,
                public_key.as_ptr() as SyscallWord,
                public_key.len() as SyscallWord,
                message.as_ptr() as SyscallWord,
                message.len() as SyscallWord,
            )
        };
        assert_code(
            unknown_scheme,
            SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput,
        );
    }

    fn test_change_pubkey_optional_pointer_fault(&self) {
        let key_bytes = [0u8; 32];
        let key_scheme: i32 = 0;
        let key_weight: u32 = 1;

        // OOB optional key scheme pointer must fault in syscall parsing.
        let oob_optional_scheme = unsafe {
            direct_syscall(
                K_SYSCALL_CHANGE_PUBLIC_KEY_REGISTRATION,
                0,
                K_OOB_PTR,
                key_bytes.as_ptr() as SyscallWord,
                key_bytes.len() as SyscallWord,
                &key_weight as *const u32 as SyscallWord,
                0,
                0,
            )
        };
        assert_code(oob_optional_scheme, SyscallErrorCode::kSyscallErrorCodeFault);

        // OOB optional key weight pointer must also fault in syscall parsing.
        let oob_optional_weight = unsafe {
            direct_syscall(
                K_SYSCALL_CHANGE_PUBLIC_KEY_REGISTRATION,
                0,
                &key_scheme as *const i32 as SyscallWord,
                key_bytes.as_ptr() as SyscallWord,
                key_bytes.len() as SyscallWord,
                K_OOB_PTR,
                0,
                0,
            )
        };
        assert_code(oob_optional_weight, SyscallErrorCode::kSyscallErrorCodeFault);
    }

    fn test_get_deploy_address_optional_pointer_sentinel(&self) {
        let nonce = SHA256Hash::default();
        let contract_hash = SHA256Hash::from([7u8; 32]);
        let mut to_be_deployed = ContractAddress::from([0u8; 32]);

        // The all-ones value is the explicit null sentinel for an omitted
        // optional pointer (SyscallWord::MAX == u32::MAX on wasm32), so the
        // optional caller_address is treated as None and the call succeeds.
        let null_sentinel_ok = unsafe {
            direct_syscall(
                K_SYSCALL_GET_DEPLOY_ADDRESS,
                SyscallWord::MAX,
                &nonce as *const SHA256Hash as SyscallWord,
                &contract_hash as *const SHA256Hash as SyscallWord,
                &mut to_be_deployed as *mut ContractAddress as SyscallWord,
                0,
                0,
                0,
            )
        };
        assert_code(
            null_sentinel_ok,
            SyscallErrorCode::kSyscallErrorCodeNone,
        );

        let oob_optional_caller = unsafe {
            direct_syscall(
                K_SYSCALL_GET_DEPLOY_ADDRESS,
                K_OOB_PTR,
                &nonce as *const SHA256Hash as SyscallWord,
                &contract_hash as *const SHA256Hash as SyscallWord,
                &mut to_be_deployed as *mut ContractAddress as SyscallWord,
                0,
                0,
                0,
            )
        };
        assert_code(oob_optional_caller, SyscallErrorCode::kSyscallErrorCodeFault);
    }
}
