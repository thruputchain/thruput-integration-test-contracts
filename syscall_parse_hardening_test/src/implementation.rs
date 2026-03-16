use super::interface::*;
use sdk::*;
use sdk::sys::SyscallErrorCode;

#[link(wasm_import_module = "marmot")]
unsafe extern "C" {
    #[link_name = "syscall"]
    fn direct_syscall(
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

const K_SYSCALL_GET_DEPLOY_ADDRESS: u64 = 9;
const K_SYSCALL_VERIFY_SIGNATURE: u64 = 12;
const K_SYSCALL_CHANGE_PUBLIC_KEY_REGISTRATION: u64 = 23;
const K_OOB_PTR: u64 = u64::MAX - 1;

fn assert_code(actual: i32, expected: SyscallErrorCode) {
    assert_eq!(actual, expected.0);
}

#[derive(Default)]
pub struct SyscallParseHardeningTestContract;

#[contract(exports = "guest_exports", storage = "false")]
impl SyscallParseHardeningTest for SyscallParseHardeningTestContract {
    fn test_verify_signature_noncanonical_scheme_fault(&self) {
        let signature = [0u8; 64];
        let public_key = [0u8; 32];
        let message = [0u8; 1];

        // Sign-extended -1 is a canonical i32 encoding, so parse succeeds and
        // the inner syscall rejects the unknown scheme.
        let canonical_neg_one = unsafe {
            direct_syscall(
                K_SYSCALL_VERIFY_SIGNATURE,
                u64::MAX,
                signature.as_ptr() as u64,
                signature.len() as u64,
                public_key.as_ptr() as u64,
                public_key.len() as u64,
                message.as_ptr() as u64,
                message.len() as u64,
            )
        };
        assert_code(
            canonical_neg_one,
            SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput,
        );

        // Non-sign-extended encodings should fail in syscall argument parsing.
        let noncanonical_neg_one = unsafe {
            direct_syscall(
                K_SYSCALL_VERIFY_SIGNATURE,
                0x00000000FFFFFFFF,
                signature.as_ptr() as u64,
                signature.len() as u64,
                public_key.as_ptr() as u64,
                public_key.len() as u64,
                message.as_ptr() as u64,
                message.len() as u64,
            )
        };
        assert_code(noncanonical_neg_one, SyscallErrorCode::kSyscallErrorCodeFault);

        let noncanonical_overflow = unsafe {
            direct_syscall(
                K_SYSCALL_VERIFY_SIGNATURE,
                0x0000000080000000,
                signature.as_ptr() as u64,
                signature.len() as u64,
                public_key.as_ptr() as u64,
                public_key.len() as u64,
                message.as_ptr() as u64,
                message.len() as u64,
            )
        };
        assert_code(noncanonical_overflow, SyscallErrorCode::kSyscallErrorCodeFault);
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
                key_bytes.as_ptr() as u64,
                key_bytes.len() as u64,
                &key_weight as *const u32 as u64,
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
                &key_scheme as *const i32 as u64,
                key_bytes.as_ptr() as u64,
                key_bytes.len() as u64,
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

        let null_sentinel_ok = unsafe {
            direct_syscall(
                K_SYSCALL_GET_DEPLOY_ADDRESS,
                u64::MAX,
                &nonce as *const SHA256Hash as u64,
                &contract_hash as *const SHA256Hash as u64,
                &mut to_be_deployed as *mut ContractAddress as u64,
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
                &nonce as *const SHA256Hash as u64,
                &contract_hash as *const SHA256Hash as u64,
                &mut to_be_deployed as *mut ContractAddress as u64,
                0,
                0,
                0,
            )
        };
        assert_code(oob_optional_caller, SyscallErrorCode::kSyscallErrorCodeFault);
    }
}
