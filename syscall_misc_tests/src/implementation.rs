use crate::SyscallMiscTests;
use sdk::sys::{self, SyscallErrorCode};
use sdk::*;

#[link(wasm_import_module = "marmot")]
unsafe extern "C" {
    #[link_name = "syscall"]
    fn direct_syscall(
        no: u64,
        a1: u64,
        a2: u64,
        a3: u64,
        a4: u64,
        a5: u64,
        a6: u64,
        a7: u64,
        a8: u64,
        a9: u64,
        a10: u64,
    ) -> i32;
}

const K_SYSCALL_RETURN: u64 = 6;

#[derive(Default)]
struct SyscallMiscTestsContract;

fn expect_err(res: Result<(), sys::SysErr>, expected: SyscallErrorCode) {
    match res {
        Ok(_) => panic!("expected error {:?} but syscall succeeded", expected),
        Err(e) => assert_eq!(e, expected.into()),
    }
}

#[contract(storage = false)]
impl SyscallMiscTests for SyscallMiscTestsContract {
    // ── Existing tests ──────────────────────────────────────────────

    fn test_get_return_before_return_buffer_set(&self) {
        let mut buffer = [0; 1024];
        let mut written_len = 0;
        let mut auth_token_id_list_buf = [0u8; 1024];
        let mut auth_token_id_list_return_len: RuntimeMemoryLength = 0;
        let res = sys::raw_GetReturnedBuffer(
            buffer.as_mut_slice(),
            &mut written_len,
            &mut auth_token_id_list_buf.as_mut_slice(),
            &mut auth_token_id_list_return_len,
        );
        expect_err(
            res,
            SyscallErrorCode::kSyscallErrorCodeNoReturnBufferAvailable,
        );
    }

    fn test_get_return_after_fault(&self) {
        let addr = sdk::get_current_address();
        let mut return_buffer_len: RuntimeMemoryLength = 0;
        let mut auth_token_id_list_return_len: RuntimeMemoryLength = 0;
        let entrypoint =
            crate::SyscallMiscTestsEntrypoints::internal_always_fault
                .into_u64();

        let res = sys::raw_Invoke(
            &addr,
            entrypoint,
            u64::MAX,
            &[],
            0,
            &[],
            &mut return_buffer_len,
            &mut auth_token_id_list_return_len,
        );
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeDeterministicWasm);

        let mut buffer = [0; 1024];
        let mut written_len = 0;
        let mut auth_token_id_list_buf = [0u8; 1024];
        let mut auth_token_id_list_return_len: RuntimeMemoryLength = 0;
        let res = sys::raw_GetReturnedBuffer(
            buffer.as_mut_slice(),
            &mut written_len,
            &mut auth_token_id_list_buf.as_mut_slice(),
            &mut auth_token_id_list_return_len,
        );
        expect_err(
            res,
            SyscallErrorCode::kSyscallErrorCodeNoReturnBufferAvailable,
        );
    }

    // ── Internal helpers (invoked by other tests via raw_Invoke) ────

    fn internal_always_fault(&self) {
        panic!("internal_always_fault");
    }

    fn internal_always_revert(&self) {
        sys::Return(&[], true);
    }

    fn internal_return_too_long(&self) {
        let buffer = [0u8; 1025]; // > kMaxReturnBufferLen (1024)
        sys::Return(&buffer, false);
    }

    fn internal_try_bytestring_set(&self) {
        let key = static_key!("test_readonly_key");
        let res = sys::raw_BytestringSet(&key, b"data", 0);
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #1: GetMsgSender → kIsRootTransaction ───────────────────────

    fn test_get_msg_sender_root(&self) {
        let mut sender = ContractAddress::from([0u8; 32]);
        let res = sys::raw_GetMsgSender(&mut sender);
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeIsRootTransaction);
    }

    // ── #2: Invoke → kInvalidSyscallInput (contract not found) ──────

    fn test_invoke_contract_not_found(&self) {
        let nonexistent = ContractAddress::from([0xFFu8; 32]);
        let mut return_buffer_len: RuntimeMemoryLength = 0;
        let mut auth_token_id_list_return_len: RuntimeMemoryLength = 0;
        let res = sys::raw_Invoke(
            &nonexistent,
            0,
            u64::MAX,
            &[],
            0,
            &[],
            &mut return_buffer_len,
            &mut auth_token_id_list_return_len,
        );
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #3: Invoke → kFrameReverted ─────────────────────────────────

    fn test_invoke_frame_reverted(&self) {
        let addr = sdk::get_current_address();
        let mut return_buffer_len: RuntimeMemoryLength = 0;
        let mut auth_token_id_list_return_len: RuntimeMemoryLength = 0;
        let entrypoint =
            crate::SyscallMiscTestsEntrypoints::internal_always_revert
                .into_u64();
        let res = sys::raw_Invoke(
            &addr,
            entrypoint,
            u64::MAX,
            &[],
            0,
            &[],
            &mut return_buffer_len,
            &mut auth_token_id_list_return_len,
        );
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeFrameReverted);
    }

    // ── #4: VerifySignature → kInvalidSyscallInput (bad scheme) ─────

    fn test_verify_sig_bad_scheme(&self) {
        let sig = [0u8; 64];
        let key = [0u8; 32];
        let res = sys::raw_VerifySignature(99, &sig, &key, b"test");
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #5: VerifySignature → kInvalidSyscallInput (bad key size) ───

    fn test_verify_sig_bad_key_size(&self) {
        let sig = [0u8; 64];
        let key = [0u8; 10]; // Ed25519 expects 32
        let res = sys::raw_VerifySignature(0, &sig, &key, b"test");
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #6: VerifySignature → kInvalidSyscallInput (wrong key) ──────

    fn test_verify_sig_bad_signature(&self) {
        let sig = [0xABu8; 64]; // valid size, invalid signature
        let key = [0xCDu8; 32]; // valid size, invalid key
        let res = sys::raw_VerifySignature(0, &sig, &key, b"test");
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #7: BytestringGetLen → kStorageKeyNotExist ──────────────────

    fn test_bs_get_len_key_not_exist(&self) {
        let key = static_key!("nonexistent_bs_get_len");
        let mut length: RuntimeMemoryLength = 0;
        let res = sys::raw_BytestringGetLen(&key, &mut length);
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeStorageKeyNotExist);
    }

    // ── #8: BytestringGetLen → kInvalidSyscallInput (wrong type) ────

    fn test_bs_get_len_wrong_type(&self) {
        let key = static_key!("bs_get_len_wrong_type");
        // Write as NNInt
        let set_val: i128 = 0;
        let add_val: i128 = 0;
        sys::raw_NonnegativeIntSetAdd(&key, Some(&set_val), &add_val, 0)
            .unwrap();
        // Try to get bytestring length
        let mut length: RuntimeMemoryLength = 0;
        let res = sys::raw_BytestringGetLen(&key, &mut length);
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #9: BytestringGet → kStorageKeyNotExist ─────────────────────

    fn test_bs_get_key_not_exist(&self) {
        let key = static_key!("nonexistent_bs_get");
        let mut buf = [0u8; 1024];
        let mut written_len: RuntimeMemoryLength = 0;
        let res =
            sys::raw_BytestringGet(&key, buf.as_mut_slice(), &mut written_len);
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeStorageKeyNotExist);
    }

    // ── #10: BytestringGet → kInvalidSyscallInput (wrong type) ──────

    fn test_bs_get_wrong_type(&self) {
        let key = static_key!("bs_get_wrong_type");
        // Write as NNInt
        let set_val: i128 = 0;
        let add_val: i128 = 0;
        sys::raw_NonnegativeIntSetAdd(&key, Some(&set_val), &add_val, 0)
            .unwrap();
        // Try to get bytestring
        let mut buf = [0u8; 1024];
        let mut written_len: RuntimeMemoryLength = 0;
        let res =
            sys::raw_BytestringGet(&key, buf.as_mut_slice(), &mut written_len);
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #11: BytestringSet → kInvalidSyscallInput (too long) ────────

    fn test_bs_set_too_long(&self) {
        let key = static_key!("bs_set_too_long");
        let data = [0u8; 1025]; // > kBytestringMaxLen (1024)
        let res = sys::raw_BytestringSet(&key, &data, 0);
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #12: BytestringSet → kInvalidSyscallInput (invalid flags) ───

    fn test_bs_set_invalid_flags(&self) {
        let key = static_key!("bs_set_invalid_flags");
        let res = sys::raw_BytestringSet(&key, b"data", 0x2);
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #13: NonnegativeIntGet → kStorageKeyNotExist ────────────────

    fn test_nnint_get_key_not_exist(&self) {
        let key = static_key!("nnint_get_key_not_exist");
        let mut current_set: i128 = 0;
        let mut current_set_opt: bool = false;
        let mut current_add: i128 = 0;
        let res = sys::raw_NonnegativeIntGet(
            &key,
            true,
            &mut current_set,
            &mut current_set_opt,
            &mut current_add,
        );
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeStorageKeyNotExist);
    }

    // ── #14: NonnegativeIntGet → kInvalidSyscallInput (wrong type) ──

    fn test_nnint_get_wrong_type(&self) {
        let key = static_key!("nnint_get_wrong_type");
        // Write as bytestring
        sys::raw_BytestringSet(&key, b"data", 0).unwrap();
        // Try to get as NNInt with force_load=true
        let mut current_set: i128 = 0;
        let mut current_set_opt: bool = false;
        let mut current_add: i128 = 0;
        let res = sys::raw_NonnegativeIntGet(
            &key,
            true,
            &mut current_set,
            &mut current_set_opt,
            &mut current_add,
        );
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #15: NonnegativeIntSetAdd → kInvalidSyscallInput (negative) ─

    fn test_nnint_set_add_negative(&self) {
        let key = static_key!("nnint_set_add_negative");
        let set_val: i128 = 1;
        let add_val: i128 = -2; // 1 + (-2) = -1, always negative
        let res =
            sys::raw_NonnegativeIntSetAdd(&key, Some(&set_val), &add_val, 0);
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #16: NonnegativeIntSetAdd → kInvalidSyscallInput (flags) ────

    fn test_nnint_set_add_invalid_flags(&self) {
        let key = static_key!("nnint_set_add_invalid_flags");
        let add_val: i128 = 0;
        let res = sys::raw_NonnegativeIntSetAdd(
            &key, None, &add_val, 0x2, // only 0x0 and 0x1 valid
        );
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #17: DeleteObject → kInvalidSyscallInput (invalid flags) ────

    fn test_delete_invalid_flags(&self) {
        let key = static_key!("delete_invalid_flags");
        let res = sys::raw_DeleteObject(&key, 0x2);
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #18: RetrocausalQueueInsert → kInvalidSyscallInput (too large)

    fn test_rcq_insert_too_large(&self) {
        let key = static_key!("rcq_insert_too_large");
        let data = [0u8; 257]; // > kQueueEntryMaxLen (256)
        let res = sys::raw_RetrocausalQueueInsert(&key, &data);
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #19: RetrocausalQueueInsert → kInvalidSyscallInput (wrong type)

    fn test_rcq_insert_wrong_type(&self) {
        let key = static_key!("rcq_insert_wrong_type");
        // Write bytestring first
        sys::raw_BytestringSet(&key, b"data", 0).unwrap();
        // Try queue insert at same key
        let res = sys::raw_RetrocausalQueueInsert(&key, b"entry");
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #20: RetrocausalQueueConsume → kNoRetrocausalQueueEntry ─────

    fn test_rcq_consume_empty(&self) {
        let key = static_key!("rcq_consume_empty");
        let mut buf = [0u8; 256];
        let mut written_len: RuntimeMemoryLength = 0;
        let res = sys::raw_RetrocausalQueueConsume(
            &key,
            false,
            buf.as_mut_slice(),
            &mut written_len,
        );
        expect_err(
            res,
            SyscallErrorCode::kSyscallErrorCodeNoRetrocausalQueueEntry,
        );
    }

    // ── #21: RetrocausalQueueConsume → kInvalidSyscallInput (wrong type)

    fn test_rcq_consume_wrong_type(&self) {
        let key = static_key!("rcq_consume_wrong_type");
        // Write bytestring first
        sys::raw_BytestringSet(&key, b"data", 0).unwrap();
        // Try queue consume at same key
        let mut buf = [0u8; 256];
        let mut written_len: RuntimeMemoryLength = 0;
        let res = sys::raw_RetrocausalQueueConsume(
            &key,
            false,
            buf.as_mut_slice(),
            &mut written_len,
        );
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #22: LogEvent → kInvalidSyscallInput (data too long) ────────

    fn test_log_event_data_too_long(&self) {
        let data = [0u8; 1025]; // > kMaxEventLength (1024)
        let res = sys::raw_LogEvent(None, None, None, &data);
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #23: LogEvent → kInvalidSyscallInput (noncontinuous topics) ─

    fn test_log_event_noncontinuous_topics(&self) {
        let topic0 = [0u8; 32];
        let topic2 = [1u8; 32];
        // topic0=Some, topic1=None, topic2=Some → gap
        let res = sys::raw_LogEvent(Some(&topic0), None, Some(&topic2), &[]);
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #24: GetAuthClaim → kInvalidSyscallInput (oob) ──────────────

    fn test_get_auth_claim_oob(&self) {
        // index=0 when count=0 → out of bounds
        let mut auth_claim = unsafe {
            core::mem::zeroed::<groundhog_xdr::auth_claim::AuthClaim>()
        };
        let res = sys::raw_GetAuthClaim(0, &mut auth_claim);
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #25: SHA256Update → kInvalidSyscallInput (no buffer) ────────

    fn test_sha256_update_no_buffer(&self) {
        // reset=false on non-existent buffer
        let res = sys::raw_SHA256Update(0, &[0u8; 1], false);
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #26: SHA256Update → kInvalidSyscallInput (too many buffers) ─

    fn test_sha256_update_too_many_buffers(&self) {
        // Open 8 buffers (max = kMaxSHA256Buffers = 8)
        for i in 0u32..8 {
            sys::raw_SHA256Update(i, &[0u8; 1], true).unwrap();
        }
        // 9th buffer should fail
        let res = sys::raw_SHA256Update(8, &[0u8; 1], true);
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #27: SHA256Update → kInvalidSyscallInput (buffer too large) ─

    fn test_sha256_update_buffer_too_large(&self) {
        // Fill buffer to exactly the limit (4096 bytes)
        let data = [0u8; 4096];
        sys::raw_SHA256Update(0, &data, true).unwrap();
        // One more byte should exceed kSHA256BufferMaxSize (4096)
        let res = sys::raw_SHA256Update(0, &[0u8; 1], false);
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #28: SHA256Finalize → kInvalidSyscallInput (no buffer) ──────

    fn test_sha256_finalize_no_buffer(&self) {
        let mut output = SHA256Hash::default();
        let res = sys::raw_SHA256Finalize(0, &mut output);
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #29: SHA256Finalize → kInvalidSyscallInput (double finalize)

    fn test_sha256_finalize_double(&self) {
        // Create and finalize buffer
        sys::raw_SHA256Update(0, b"test", true).unwrap();
        let mut output = SHA256Hash::default();
        sys::raw_SHA256Finalize(0, &mut output).unwrap();
        // Second finalize should fail (buffer was removed)
        let res = sys::raw_SHA256Finalize(0, &mut output);
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #30: GetTransactionStatistics → kInvalidSyscallInput ────────

    fn test_get_tx_stats_invalid(&self) {
        // Call the raw syscall import directly to pass an invalid statistic
        // value (9999) without triggering UB from transmuting an invalid enum.
        // GetTransactionStatistics is syscall #22.
        let mut result: u64 = 0;
        let err_code = unsafe {
            #[link(wasm_import_module = "marmot")]
            extern "C" {
                #[link_name = "syscall"]
                fn direct_syscall(
                    no: u64,
                    a1: u64,
                    a2: u64,
                    a3: u64,
                    a4: u64,
                    a5: u64,
                    a6: u64,
                    a7: u64,
                    a8: u64,
                    a9: u64,
                    a10: u64,
                ) -> i32;
            }
            direct_syscall(
                22,                             // syscall number
                9999,                           // invalid statistic
                &mut result as *mut u64 as u64, // result pointer
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            )
        };
        let err = sys::SysErr(err_code);
        assert_eq!(
            err,
            SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput.into()
        );
    }

    // ── #31: ChangePublicKeyRegistration → kInvalidSyscallInput (oob)

    fn test_change_pubkey_index_oob(&self) {
        let scheme: i32 = 0; // Ed25519
        let weight: u32 = 1;
        let res = sys::raw_ChangePublicKeyRegistration(
            21, // >= kMaxSigners (21)
            Some(&scheme),
            &[0u8; 32],
            Some(&weight),
        );
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #32: WriteAddressGasPermissionConfigs → kInvalidSyscallInput

    fn test_gas_perms_non_initial_segment(&self) {
        let config = groundhog_xdr::address_auth::GasPermissionConfig {
            limit: 0,
            accepted_keys: 0,
            required_weight: 0,
        };
        // permission_1 supplied without permission_0
        let res = sys::raw_WriteAddressGasPermissionConfigs(
            None,
            Some(&config),
            None,
            None,
            None,
        );
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #33: DeployContract → kInvalidSyscallInput (not available) ──

    fn test_deploy_contract_not_available(&self) {
        let nonce = SHA256Hash::default();
        let contract_hash = SHA256Hash::from([0xFFu8; 32]);
        let mut deployed_address = ContractAddress::from([0u8; 32]);
        let res = sys::raw_DeployContract(
            &nonce,
            &contract_hash,
            &mut deployed_address,
        );
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #34: Return with too-large buffer → fault ───────────────────

    fn test_return_buffer_too_long(&self) {
        let addr = sdk::get_current_address();
        let mut return_buffer_len: RuntimeMemoryLength = 0;
        let mut auth_token_id_list_return_len: RuntimeMemoryLength = 0;
        let entrypoint =
            crate::SyscallMiscTestsEntrypoints::internal_return_too_long
                .into_u64();

        let res = sys::raw_Invoke(
            &addr,
            entrypoint,
            u64::MAX,
            &[],
            0,
            &[],
            &mut return_buffer_len,
            &mut auth_token_id_list_return_len,
        );
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeDeterministicWasm);

        // Buffer should NOT be retrievable after a fault
        let mut buffer = [0u8; 1024];
        let mut written_len: RuntimeMemoryLength = 0;
        let mut auth_token_id_list_return_len: RuntimeMemoryLength = 0;
        let mut auth_token_id_list_buf = [0u8; 1024];
        let res = sys::raw_GetReturnedBuffer(
            buffer.as_mut_slice(),
            &mut written_len,
            &mut auth_token_id_list_buf.as_mut_slice(),
            &mut auth_token_id_list_return_len,
        );
        expect_err(
            res,
            SyscallErrorCode::kSyscallErrorCodeNoReturnBufferAvailable,
        );
    }

    // ── #35: DeployContract → kContractAlreadyDeployed ──────────────

    fn test_deploy_contract_already_deployed(
        &self,
        contract_hash: &SHA256Hash,
    ) {
        let nonce = SHA256Hash::default();
        let mut deployed_address = ContractAddress::from([0u8; 32]);

        // First deploy should succeed
        sys::raw_DeployContract(&nonce, contract_hash, &mut deployed_address)
            .unwrap();

        // Second deploy with same nonce+hash → same address → already deployed
        let mut deployed_address2 = ContractAddress::from([0u8; 32]);
        let res = sys::raw_DeployContract(
            &nonce,
            contract_hash,
            &mut deployed_address2,
        );
        expect_err(
            res,
            SyscallErrorCode::kSyscallErrorCodeContractAlreadyDeployed,
        );
    }

    // ── #36: Invoke with garbage flags → kInvalidSyscallInput ───────

    fn test_invoke_invalid_flags(&self) {
        let addr = sdk::get_current_address();
        let mut return_buffer_len: RuntimeMemoryLength = 0;
        let mut auth_token_id_list_return_len: RuntimeMemoryLength = 0;
        let res = sys::raw_Invoke(
            &addr,
            0,
            u64::MAX,
            &[],
            0xFFFF, // garbage flags
            &[],
            &mut return_buffer_len,
            &mut auth_token_id_list_return_len,
        );
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #36: Write syscall in read-only frame → kInvalidSyscallInput

    fn test_write_syscall_in_read_only(&self) {
        let addr = sdk::get_current_address();
        let mut return_buffer_len: RuntimeMemoryLength = 0;
        let mut auth_token_id_list_return_len: RuntimeMemoryLength = 0;
        let entrypoint =
            crate::SyscallMiscTestsEntrypoints::internal_try_bytestring_set
                .into_u64();

        // Invoke with read-only flag (0x2 = ReadOnly)
        // internal_try_bytestring_set will verify that BytestringSet
        // returns kInvalidSyscallInput, then return normally
        let res = sys::raw_Invoke(
            &addr,
            entrypoint,
            u64::MAX,
            &[],
            0x2,
            &[],
            &mut return_buffer_len,
            &mut auth_token_id_list_return_len,
        );
        match res {
            Ok(_) => {} // sub-frame verified the error and returned
            Err(e) => {
                panic!("invoke should have succeeded, got error: {:?}", e)
            }
        }
    }

    // ── No-Return-syscall test ────────────────────────────────────────

    fn test_invoke_no_return_syscall(&self, target: &ContractAddress) {
        let mut return_buffer_len: RuntimeMemoryLength = 0;
        let mut auth_token_id_list_return_len: RuntimeMemoryLength = 0;
        // Invoke the raw export at entrypoint 0xFF on a separate contract.
        // That export just returns without calling the Return syscall.
        let res = sys::raw_Invoke(
            target,
            0xFF,
            u64::MAX,
            &[],
            0,
            &[],
            &mut return_buffer_len,
            &mut auth_token_id_list_return_len,
        );
        assert!(res.is_ok(), "invoke of noop should succeed");
        assert_eq!(return_buffer_len, 0, "return buffer should be empty");

        let mut buffer = [0u8; 1024];
        let mut written_len: RuntimeMemoryLength = 0;
        let mut auth_token_id_list_buf = [0u8; 1024];
        let mut auth_token_id_list_result_len: RuntimeMemoryLength = 0;
        let res = sys::raw_GetReturnedBuffer(
            buffer.as_mut_slice(),
            &mut written_len,
            &mut auth_token_id_list_buf.as_mut_slice(),
            &mut auth_token_id_list_result_len,
        );
        assert!(res.is_ok(), "GetReturnedBuffer should succeed");
        assert_eq!(written_len, 0, "returned buffer should be empty");
    }

    // ── Return auth token edge-case internal helpers ────────────────

    fn internal_return_misaligned_auth_tokens(&self) {
        let auth_tokens: [u8; 3] = [0, 0, 0]; // not a multiple of sizeof(u32)
        unsafe {
            direct_syscall(
                K_SYSCALL_RETURN,
                0,
                0, // empty buffer
                0, // revert_frame = false
                auth_tokens.as_ptr() as u64,
                auth_tokens.len() as u64,
                0,
                0,
                0,
                0,
                0,
            );
        }
        unreachable!()
    }

    fn internal_return_revert_with_auth_tokens(&self) {
        let auth_tokens: [u32; 1] = [0];
        let ptr = auth_tokens.as_ptr() as *const u8;
        let len = core::mem::size_of_val(&auth_tokens);
        unsafe {
            direct_syscall(
                K_SYSCALL_RETURN,
                0,
                0, // empty buffer
                1, // revert_frame = true
                ptr as u64,
                len as u64,
                0,
                0,
                0,
                0,
                0,
            );
        }
        unreachable!()
    }

    fn internal_return_with_empty_auth_tokens(&self) {
        let buf = [42u8; 4];
        unsafe {
            direct_syscall(
                K_SYSCALL_RETURN,
                buf.as_ptr() as u64,
                buf.len() as u64,
                0, // revert_frame = false
                0, // null ptr
                0, // len = 0
                0,
                0,
                0,
                0,
                0,
            );
        }
        unreachable!()
    }

    fn internal_return_with_auth_token_ids(&self) {
        let buf = [99u8; 4];
        let auth_tokens: [u32; 2] = [0, 1];
        let ptr = auth_tokens.as_ptr() as *const u8;
        let len = core::mem::size_of_val(&auth_tokens);
        unsafe {
            direct_syscall(
                K_SYSCALL_RETURN,
                buf.as_ptr() as u64,
                buf.len() as u64,
                0, // revert_frame = false
                ptr as u64,
                len as u64,
                0,
                0,
                0,
                0,
                0,
            );
        }
        unreachable!()
    }

    // ── Return auth token edge-case tests ───────────────────────────

    fn test_return_misaligned_auth_tokens(&self) {
        let addr = sdk::get_current_address();
        let mut return_buffer_len: RuntimeMemoryLength = 0;
        let mut auth_token_id_list_return_len: RuntimeMemoryLength = 0;
        let entrypoint =
            crate::SyscallMiscTestsEntrypoints::internal_return_misaligned_auth_tokens
                .into_u64();
        let res = sys::raw_Invoke(
            &addr,
            entrypoint,
            u64::MAX,
            &[],
            0,
            &[],
            &mut return_buffer_len,
            &mut auth_token_id_list_return_len,
        );
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeDeterministicWasm);
    }

    fn test_return_revert_with_auth_tokens(&self) {
        let addr = sdk::get_current_address();
        let mut return_buffer_len: RuntimeMemoryLength = 0;
        let mut auth_token_id_list_return_len: RuntimeMemoryLength = 0;
        let entrypoint =
            crate::SyscallMiscTestsEntrypoints::internal_return_revert_with_auth_tokens
                .into_u64();
        let res = sys::raw_Invoke(
            &addr,
            entrypoint,
            u64::MAX,
            &[],
            0,
            &[],
            &mut return_buffer_len,
            &mut auth_token_id_list_return_len,
        );
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeDeterministicWasm);
    }

    fn test_return_with_empty_auth_tokens(&self) {
        let addr = sdk::get_current_address();
        let mut return_buffer_len: RuntimeMemoryLength = 0;
        let mut auth_token_id_list_return_len: RuntimeMemoryLength = 0;
        let entrypoint =
            crate::SyscallMiscTestsEntrypoints::internal_return_with_empty_auth_tokens
                .into_u64();
        let res = sys::raw_Invoke(
            &addr,
            entrypoint,
            u64::MAX,
            &[],
            0,
            &[],
            &mut return_buffer_len,
            &mut auth_token_id_list_return_len,
        );
        assert!(res.is_ok());

        let mut buffer = [0u8; 1024];
        let mut written_len: RuntimeMemoryLength = 0;
        let mut auth_token_id_list_buf = [0u8; 1024];
        let mut auth_token_id_list_result_len: RuntimeMemoryLength = 0;
        let res = sys::raw_GetReturnedBuffer(
            buffer.as_mut_slice(),
            &mut written_len,
            &mut auth_token_id_list_buf.as_mut_slice(),
            &mut auth_token_id_list_result_len,
        );
        assert!(res.is_ok());
        assert_eq!(written_len, 4);
        assert_eq!(&buffer[..4], &[42, 42, 42, 42]);
    }

    fn test_return_with_auth_token_ids(&self) {
        let addr = sdk::get_current_address();
        let mut return_buffer_len: RuntimeMemoryLength = 0;
        let mut auth_token_id_list_return_len: RuntimeMemoryLength = 0;
        let entrypoint =
            crate::SyscallMiscTestsEntrypoints::internal_return_with_auth_token_ids
                .into_u64();
        let res = sys::raw_Invoke(
            &addr,
            entrypoint,
            u64::MAX,
            &[],
            0,
            &[],
            &mut return_buffer_len,
            &mut auth_token_id_list_return_len,
        );
        // The child doesn't own any auth tokens, but the Return syscall
        // currently doesn't validate ownership — it just stores the IDs.
        // This should succeed (or fail if ownership validation is added).
        assert!(res.is_ok());

        let mut buffer = [0u8; 1024];
        let mut written_len: RuntimeMemoryLength = 0;
        let mut auth_token_id_list_buf = [0u8; 1024];
        let mut auth_token_id_list_result_len: RuntimeMemoryLength = 0;
        let res = sys::raw_GetReturnedBuffer(
            buffer.as_mut_slice(),
            &mut written_len,
            &mut auth_token_id_list_buf.as_mut_slice(),
            &mut auth_token_id_list_result_len,
        );
        assert!(res.is_ok());
        assert_eq!(written_len, 4);
        assert_eq!(&buffer[..4], &[99, 99, 99, 99]);
    }
}
