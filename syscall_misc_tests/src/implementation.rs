use crate::SyscallMiscTests;
use sdk::sys::{self, SyscallErrorCode};
use sdk::transaction::InvocationFrameStatusCode;
use sdk::*;
use groundhog_xdr::syscalls::SyscallWord;

// `groundhog_syscall::syscall_wrapper` is the single shared syscall-dispatch
// chokepoint: the SDK routes its own syscalls through it too, so a guest
// artifact carries exactly one syscall entry sequence. The thin `direct_syscall` shim keeps the existing call sites
// (which pass `SyscallWord`-typed args) unchanged.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
unsafe fn direct_syscall(
    no: SyscallWord,
    a1: SyscallWord,
    a2: SyscallWord,
    a3: SyscallWord,
    a4: SyscallWord,
    a5: SyscallWord,
    a6: SyscallWord,
    a7: SyscallWord,
) -> i32 {
    groundhog_syscall::syscall_wrapper(no, a1, a2, a3, a4, a5, a6, a7)
}

const K_SYSCALL_RETURN: SyscallWord = sys::SyscallNo::kReturn.0 as SyscallWord;

/// Key derivation shared with the C++ side's make_obs_cap_key (rcq observation
/// cap tests): 'o','c','k' prefix, index LE at bytes [4, 8).
fn obs_cap_key(index: u32) -> StorageKeyContractData {
    let mut key = [0u8; 32];
    key[0] = b'o';
    key[1] = b'c';
    key[2] = b'k';
    key[4..8].copy_from_slice(&index.to_le_bytes());
    key.into()
}

struct SyscallMiscTestsContract;

fn expect_err(res: Result<(), sys::SysErr>, expected: SyscallErrorCode) {
    match res {
        Ok(_) => panic!("expected error {:?} but syscall succeeded", expected),
        Err(e) => assert_eq!(e, expected.into()),
    }
}

/// Assert that an `Invoke` actually ran a child frame (the syscall returned
/// `Ok`) and that the child frame's outcome matches `expected`. With the new
/// invoke ABI, a child fault or manual revert is no longer surfaced as a
/// syscall error code; the outcome is reported in `outputs.frame_status`.
fn expect_frame_status(
    res: Result<(), sys::SysErr>,
    outputs: &sys::CopyableInvokeOutputs,
    expected: InvocationFrameStatusCode,
) {
    match res {
        Ok(()) => assert_eq!(outputs.frame_status, expected),
        Err(e) => panic!(
            "expected frame status {:?} but syscall returned error {:?}",
            expected, e
        ),
    }
}

/// Sentinel `frame_status` we seed the outputs with before an `Invoke`. It is
/// distinct from every real `InvocationFrameStatusCode` (Return/Fault/Abort),
/// so a `frame_status` assertion only passes if the syscall genuinely wrote
/// the outcome over this placeholder rather than leaving it untouched.
const UNWRITTEN_FRAME_STATUS: InvocationFrameStatusCode =
    InvocationFrameStatusCode(-1);

#[contract]
impl SyscallMiscTests for SyscallMiscTestsContract {
    // ── Existing tests ──────────────────────────────────────────────

    fn test_get_return_before_return_buffer_set() {
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

    fn test_get_return_after_fault(target: &ContractAddress) {
        let entrypoint =
            crate::SyscallMiscTestsEntrypoints::internal_always_fault
                .into_u64();

        let mut outputs = sys::CopyableInvokeOutputs { return_buffer_len: 0, returned_auth_token_len: 0, frame_status: UNWRITTEN_FRAME_STATUS };
        let res = sys::raw_Invoke(
            target,
            &[],
            &[],
            &sys::CopyableInvokeInputs { entrypoint_id: entrypoint, gas_limit: u64::MAX, flags: 0 },
            &mut outputs,
        );
        expect_frame_status(res, &outputs, InvocationFrameStatusCode::Fault);

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

    fn internal_always_fault() {
        panic!("internal_always_fault");
    }

    fn internal_always_revert() {
        sys::Return(&[], true, &[]);
    }

    fn internal_return_too_long() {
        let buffer = [0u8; (sdk::transaction::kMaxReturnBufferLen + 1) as usize]; // > kMaxReturnBufferLen (1024)
        sys::Return(&buffer, false, &[]);
    }

    fn internal_return_exact_max() {
        let mut buffer = [0u8; sdk::transaction::kMaxReturnBufferLen as usize];
        let mut i: usize = 0;
        while i < buffer.len() {
            buffer[i] = (i % 251) as u8;
            i += 1;
        }
        sys::Return(&buffer, false, &[]);
    }

    fn internal_try_bytestring_set() {
        let key = static_key!("test_readonly_key");
        let res = sys::raw_BytestringSet(&key, b"data", 0);
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #1: GetMsgSender → kIsRootTransaction ───────────────────────

    fn test_get_msg_sender_root() {
        let mut sender = ContractAddress::from([0u8; 32]);
        let res = sys::raw_GetMsgSender(&mut sender);
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeIsRootTransaction);
    }

    // ── #2: Invoke → kInvalidSyscallInput (contract not found) ──────

    fn test_invoke_contract_not_found() {
        let nonexistent = ContractAddress::from([0xFFu8; 32]);
        let res = sys::raw_Invoke(
            &nonexistent,
            &[],
            &[],
            &sys::CopyableInvokeInputs { entrypoint_id: 0, gas_limit: u64::MAX, flags: 0 },
            &mut sys::CopyableInvokeOutputs { return_buffer_len: 0, returned_auth_token_len: 0, frame_status: UNWRITTEN_FRAME_STATUS },
        );
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #3: Invoke → frame_status Abort (manual revert) ─────────────

    fn test_invoke_frame_reverted(target: &ContractAddress) {
        let entrypoint =
            crate::SyscallMiscTestsEntrypoints::internal_always_revert
                .into_u64();
        let mut outputs = sys::CopyableInvokeOutputs { return_buffer_len: 0, returned_auth_token_len: 0, frame_status: UNWRITTEN_FRAME_STATUS };
        let res = sys::raw_Invoke(
            target,
            &[],
            &[],
            &sys::CopyableInvokeInputs { entrypoint_id: entrypoint, gas_limit: u64::MAX, flags: 0 },
            &mut outputs,
        );
        expect_frame_status(res, &outputs, InvocationFrameStatusCode::Abort);
    }

    // ── #4: VerifySignature → kInvalidSyscallInput (bad scheme) ─────

    fn test_verify_sig_bad_scheme() {
        let sig = [0u8; 64];
        let key = [0u8; 32];
        let res = sys::raw_VerifySignature(99, &sig, &key, b"test");
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #5: VerifySignature → kInvalidSyscallInput (bad key size) ───

    fn test_verify_sig_bad_key_size() {
        let sig = [0u8; 64];
        let key = [0u8; 10]; // Ed25519 expects 32
        let res = sys::raw_VerifySignature(0, &sig, &key, b"test");
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #6: VerifySignature → kInvalidSyscallInput (wrong key) ──────

    fn test_verify_sig_bad_signature() {
        let sig = [0xABu8; 64]; // valid size, invalid signature
        let key = [0xCDu8; 32]; // valid size, invalid key
        let res = sys::raw_VerifySignature(0, &sig, &key, b"test");
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #7: BytestringGetLen → kStorageKeyNotExist ──────────────────

    fn test_bs_get_len_key_not_exist() {
        let key = static_key!("nonexistent_bs_get_len");
        let mut length: RuntimeMemoryLength = 0;
        let res = sys::raw_BytestringGetLen(&key, &mut length);
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeStorageKeyNotExist);
    }

    // ── #8: BytestringGetLen → kInvalidSyscallInput (wrong type) ────

    fn test_bs_get_len_wrong_type() {
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

    fn test_bs_get_key_not_exist() {
        let key = static_key!("nonexistent_bs_get");
        let mut buf = [0u8; 1024];
        let mut written_len: RuntimeMemoryLength = 0;
        let res =
            sys::raw_BytestringGet(&key, buf.as_mut_slice(), &mut written_len);
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeStorageKeyNotExist);
    }

    // ── #10: BytestringGet → kInvalidSyscallInput (wrong type) ──────

    fn test_bs_get_wrong_type() {
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

    fn test_bs_set_too_long() {
        let key = static_key!("bs_set_too_long");
        let data = [0u8; 1025]; // > kBytestringMaxLen (1024)
        let res = sys::raw_BytestringSet(&key, &data, 0);
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #12: BytestringSet → kInvalidSyscallInput (invalid flags) ───

    fn test_bs_set_invalid_flags() {
        let key = static_key!("bs_set_invalid_flags");
        let res = sys::raw_BytestringSet(&key, b"data", 0x2);
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #13: NonnegativeIntGet → kStorageKeyNotExist ────────────────

    fn test_nnint_get_key_not_exist() {
        let key = static_key!("nnint_get_key_not_exist");
        // Sentinel-seeded outputs: an inner-syscall error must leave every
        // output unwritten (the wrapper disarms staged outputs on error).
        let mut current_set: i128 = 0x5EED_5EED;
        let mut current_set_opt: bool = true;
        let mut current_add: i128 = 0x5EED_5EED;
        let res = sys::raw_NonnegativeIntGet(
            &key,
            true,
            &mut current_set,
            &mut current_set_opt,
            &mut current_add,
        );
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeStorageKeyNotExist);
        assert!(current_set == 0x5EED_5EED);
        assert!(current_set_opt);
        assert!(current_add == 0x5EED_5EED);
    }

    // ── #14: NonnegativeIntGet → kInvalidSyscallInput (wrong type) ──

    fn test_nnint_get_wrong_type() {
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

    fn test_nnint_set_add_negative() {
        let key = static_key!("nnint_set_add_negative");
        let set_val: i128 = 1;
        let add_val: i128 = -2; // 1 + (-2) = -1, always negative
        let res =
            sys::raw_NonnegativeIntSetAdd(&key, Some(&set_val), &add_val, 0);
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #16: NonnegativeIntSetAdd → kInvalidSyscallInput (flags) ────

    fn test_nnint_set_add_invalid_flags() {
        let key = static_key!("nnint_set_add_invalid_flags");
        let add_val: i128 = 0;
        let res = sys::raw_NonnegativeIntSetAdd(
            &key, None, &add_val, 0x2, // only 0x0 and 0x1 valid
        );
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #17: DeleteObject → kInvalidSyscallInput (invalid flags) ────

    fn test_delete_invalid_flags() {
        let key = static_key!("delete_invalid_flags");
        let res = sys::raw_DeleteObject(&key, 0x2);
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #18: RetrocausalQueueInsert → kInvalidSyscallInput (too large)

    fn test_rcq_insert_too_large() {
        let key = static_key!("rcq_insert_too_large");
        let data = [0u8; 257]; // > kQueueEntryMaxLen (256)
        let res = sys::raw_RetrocausalQueueInsert(&key, &data);
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #19: RetrocausalQueueInsert → kInvalidSyscallInput (wrong type)

    fn test_rcq_insert_wrong_type() {
        let key = static_key!("rcq_insert_wrong_type");
        // Write bytestring first
        sys::raw_BytestringSet(&key, b"data", 0).unwrap();
        // Try queue insert at same key
        let res = sys::raw_RetrocausalQueueInsert(&key, b"entry");
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #20: RetrocausalQueueConsume → kNoRetrocausalQueueEntry ─────

    fn test_rcq_consume_empty() {
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

    fn test_rcq_consume_wrong_type() {
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

    fn test_log_event_data_too_long() {
        let data = [0u8; 1025]; // > kMaxEventLength (1024)
        let res = sys::raw_LogEvent(None, None, None, &data);
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #23: LogEvent → kInvalidSyscallInput (noncontinuous topics) ─

    fn test_log_event_noncontinuous_topics() {
        let topic0 = [0u8; 32];
        let topic2 = [1u8; 32];
        // topic0=Some, topic1=None, topic2=Some → gap
        let res = sys::raw_LogEvent(Some(&topic0), None, Some(&topic2), &[]);
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #25: SHA256Update → kInvalidSyscallInput (no buffer) ────────

    fn test_sha256_update_no_buffer() {
        // reset=false on non-existent buffer
        let res = sys::raw_SHA256Update(0, &[0u8; 1], false);
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #26: SHA256Update → kInvalidSyscallInput (too many buffers) ─

    fn test_sha256_update_too_many_buffers() {
        // Open 8 buffers (max = kMaxSHA256Buffers = 8)
        for i in 0u32..8 {
            sys::raw_SHA256Update(i, &[0u8; 1], true).unwrap();
        }
        // 9th buffer should fail
        let res = sys::raw_SHA256Update(8, &[0u8; 1], true);
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #27: SHA256Update → kInvalidSyscallInput (buffer too large) ─

    fn test_sha256_update_buffer_too_large() {
        // Fill buffer to exactly the limit (4096 bytes)
        let data = [0u8; 4096];
        sys::raw_SHA256Update(0, &data, true).unwrap();
        // One more byte should exceed kSHA256BufferMaxSize (4096)
        let res = sys::raw_SHA256Update(0, &[0u8; 1], false);
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #28: SHA256Finalize → kInvalidSyscallInput (no buffer) ──────

    fn test_sha256_finalize_no_buffer() {
        // Sentinel-seeded output: the inner error must leave it unwritten.
        let mut output = SHA256Hash::from([0xAB; 32]);
        let res = sys::raw_SHA256Finalize(0, &mut output);
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
        assert!(output == SHA256Hash::from([0xAB; 32]));
    }

    // ── #29: SHA256Finalize → kInvalidSyscallInput (double finalize)

    fn test_sha256_finalize_double() {
        // Create and finalize buffer
        sys::raw_SHA256Update(0, b"test", true).unwrap();
        let mut output = SHA256Hash::default();
        sys::raw_SHA256Finalize(0, &mut output).unwrap();
        // Second finalize should fail (buffer was removed)
        let res = sys::raw_SHA256Finalize(0, &mut output);
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #30: GetTransactionStatistics → kInvalidSyscallInput ────────

    fn test_get_tx_stats_invalid() {
        // Call the syscall dispatch shim directly to pass an invalid statistic
        // value (9999) without triggering UB from transmuting an invalid enum.
        // GetTransactionStatistics is syscall #22. Routes through the shared
        // `direct_syscall` shim (groundhog_syscall::syscall_wrapper) so the rv32
        // ELF resolves `syscall` the same way every other call site does.
        let mut result: u64 = 0;
        let err_code = unsafe {
            direct_syscall(
                sys::SyscallNo::kGetTransactionStatistics.0 as SyscallWord, // syscall number
                9999,                           // invalid statistic
                &mut result as *mut u64 as SyscallWord, // result pointer
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

    fn test_change_pubkey_index_oob() {
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

    // ── #33: DeployContract → kInvalidSyscallInput (not available) ──

    fn test_deploy_contract_not_available() {
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

    fn test_return_buffer_too_long(target: &ContractAddress) {
        let entrypoint =
            crate::SyscallMiscTestsEntrypoints::internal_return_too_long
                .into_u64();

        let mut outputs = sys::CopyableInvokeOutputs { return_buffer_len: 0, returned_auth_token_len: 0, frame_status: UNWRITTEN_FRAME_STATUS };
        let res = sys::raw_Invoke(
            target,
            &[],
            &[],
            &sys::CopyableInvokeInputs { entrypoint_id: entrypoint, gas_limit: u64::MAX, flags: 0 },
            &mut outputs,
        );
        expect_frame_status(res, &outputs, InvocationFrameStatusCode::Fault);

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

    // ── #34b: Return with exactly kMaxReturnBufferLen bytes → Return ─

    fn test_return_buffer_exact_max(target: &ContractAddress) {
        let entrypoint =
            crate::SyscallMiscTestsEntrypoints::internal_return_exact_max
                .into_u64();

        let mut outputs = sys::CopyableInvokeOutputs { return_buffer_len: 0, returned_auth_token_len: 0, frame_status: UNWRITTEN_FRAME_STATUS };
        let res = sys::raw_Invoke(
            target,
            &[],
            &[],
            &sys::CopyableInvokeInputs { entrypoint_id: entrypoint, gas_limit: u64::MAX, flags: 0 },
            &mut outputs,
        );
        expect_frame_status(res, &outputs, InvocationFrameStatusCode::Return);
        assert!(outputs.return_buffer_len == sdk::transaction::kMaxReturnBufferLen);

        let mut buffer = [0u8; sdk::transaction::kMaxReturnBufferLen as usize];
        let mut written_len: RuntimeMemoryLength = 0;
        let mut auth_token_id_list_return_len: RuntimeMemoryLength = 0;
        let mut auth_token_id_list_buf = [0u8; 1024];
        sys::raw_GetReturnedBuffer(
            buffer.as_mut_slice(),
            &mut written_len,
            &mut auth_token_id_list_buf.as_mut_slice(),
            &mut auth_token_id_list_return_len,
        )
        .unwrap();
        assert!(written_len == sdk::transaction::kMaxReturnBufferLen);
        let mut i: usize = 0;
        while i < buffer.len() {
            assert!(buffer[i] == (i % 251) as u8);
            i += 1;
        }
    }

    // ── #35: DeployContract → kContractAlreadyDeployed ──────────────

    fn test_deploy_contract_already_deployed(
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

    fn test_invoke_invalid_flags() {
        let addr = sdk::get_current_address();
        let mut outputs = sys::CopyableInvokeOutputs { return_buffer_len: u32::MAX, returned_auth_token_len: u32::MAX, frame_status: UNWRITTEN_FRAME_STATUS };
        let res = sys::raw_Invoke(
            &addr,
            &[],
            &[],
            &sys::CopyableInvokeInputs { entrypoint_id: 0, gas_limit: u64::MAX, flags: 0xFFFF }, // garbage flags
            &mut outputs,
        );
        // outputs should not be written on syscall error
        assert!(outputs.return_buffer_len == u32::MAX);
        assert!(outputs.returned_auth_token_len == u32::MAX);
        assert!(outputs.frame_status == UNWRITTEN_FRAME_STATUS);

        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #36: Write syscall in read-only frame → kInvalidSyscallInput

    fn test_write_syscall_in_read_only(target: &ContractAddress) {
        let entrypoint =
            crate::SyscallMiscTestsEntrypoints::internal_try_bytestring_set
                .into_u64();

        // Invoke with read-only flag (0x2 = ReadOnly)
        // internal_try_bytestring_set will verify that BytestringSet
        // returns kInvalidSyscallInput, then return normally.
        //
        // The frame status has to be checked, not just `res`: a child that
        // panics comes back as Ok with frame_status Fault, so matching on
        // Ok/Err alone would pass even when the child's assertion failed --
        // i.e. even when the frame was not read-only and the write succeeded.
        let mut outputs = sys::CopyableInvokeOutputs { return_buffer_len: 0, returned_auth_token_len: 0, frame_status: UNWRITTEN_FRAME_STATUS };
        let res = sys::raw_Invoke(
            target,
            &[],
            &[],
            &sys::CopyableInvokeInputs { entrypoint_id: entrypoint, gas_limit: u64::MAX, flags: 0x2 },
            &mut outputs,
        );
        expect_frame_status(res, &outputs, InvocationFrameStatusCode::Return);
    }

    // ── #37: precedence: RCQ read-cap beats empty-queue, loses to wrong-type

    fn internal_rcq_obs_cap_seed(count: u32) {
        for i in 0..count {
            let key = obs_cap_key(i);
            sys::raw_RetrocausalQueueInsert(&key, b"e").unwrap();
        }
    }

    fn test_rcq_cap_precedence(count: u32) {
        // A bytestring-occupied key, written before the cap is reached.
        let bs_key = static_key!("rcq_cap_wrong_type");
        sys::raw_BytestringSet(&bs_key, b"data", 0).unwrap();

        // Observe `count` seeded entries, exactly filling the per-tx
        // observation cap.
        for i in 0..count {
            let key = obs_cap_key(i);
            let mut len: RuntimeMemoryLength = 0;
            sys::raw_RetrocausalQueuePeekLen(&key, &mut len).unwrap();
            assert!(len == 1);
        }

        // A cap-crossing read of a real entry trips the read limit.
        let mut len: RuntimeMemoryLength = 0;
        let res = sys::raw_RetrocausalQueuePeekLen(&obs_cap_key(count), &mut len);
        expect_err(
            res,
            SyscallErrorCode::kSyscallErrorCodeRetrocausalQueueReadLimitReached,
        );

        // Once tripped, an empty-queue read reports the cap, not emptiness:
        // the read-limit check runs before the empty-queue check.
        let empty_key = static_key!("rcq_cap_empty_key");
        let mut len: RuntimeMemoryLength = 0;
        let res = sys::raw_RetrocausalQueuePeekLen(&empty_key, &mut len);
        expect_err(
            res,
            SyscallErrorCode::kSyscallErrorCodeRetrocausalQueueReadLimitReached,
        );

        // ...but a wrong-type key reports the type error: the item-type
        // dispatch runs before the read-limit check.
        let mut buf = [0u8; 8];
        let mut written: RuntimeMemoryLength = 0;
        let res = sys::raw_RetrocausalQueueConsume(
            &bs_key,
            false,
            buf.as_mut_slice(),
            &mut written,
        );
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
    }

    // ── #38: precedence: no-return-buffer beats misaligned token span ──

    fn test_get_returned_buffer_no_buffer_beats_misaligned() {
        let mut buffer = [0; 64];
        let mut written_len = 0;
        let mut token_buf = [0u8; 3]; // len % 4 != 0
        let mut token_written: RuntimeMemoryLength = 0;
        let res = sys::raw_GetReturnedBuffer(
            buffer.as_mut_slice(),
            &mut written_len,
            &mut token_buf.as_mut_slice(),
            &mut token_written,
        );
        expect_err(
            res,
            SyscallErrorCode::kSyscallErrorCodeNoReturnBufferAvailable,
        );
    }

    // ── #39: precedence: unowned forwarded token beats reentrance ──

    fn test_invoke_unowned_token_beats_reentrance() {
        let addr = sdk::get_current_address();
        let fabricated_token_id: [u8; 4] = 0xDEADu32.to_le_bytes();
        let mut outputs = sys::CopyableInvokeOutputs { return_buffer_len: u32::MAX, returned_auth_token_len: u32::MAX, frame_status: UNWRITTEN_FRAME_STATUS };
        let res = sys::raw_Invoke(
            &addr,
            &[],
            &fabricated_token_id,
            &sys::CopyableInvokeInputs { entrypoint_id: 0, gas_limit: u64::MAX, flags: 0 },
            &mut outputs,
        );
        // The token-ownership check runs before the reentrance check.
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);
        assert!(outputs.return_buffer_len == u32::MAX);
        assert!(outputs.returned_auth_token_len == u32::MAX);
        assert!(outputs.frame_status == UNWRITTEN_FRAME_STATUS);
    }

    // ── #40: reentrance beats resolution: self-invoke reports kReentrance
    // even when the frame address has no deployed contract (default-wallet
    // fallback execution at an undeployed address).

    fn test_invoke_self_is_reentrance() {
        let addr = sdk::get_current_address();
        let mut outputs = sys::CopyableInvokeOutputs { return_buffer_len: u32::MAX, returned_auth_token_len: u32::MAX, frame_status: UNWRITTEN_FRAME_STATUS };
        let res = sys::raw_Invoke(
            &addr,
            &[],
            &[],
            &sys::CopyableInvokeInputs { entrypoint_id: 0, gas_limit: u64::MAX, flags: 0 },
            &mut outputs,
        );
        expect_err(res, SyscallErrorCode::kSyscallErrorCodeReentrance);
        assert!(outputs.frame_status == UNWRITTEN_FRAME_STATUS);
    }

    // ── #41: precedence: argument parse fault beats read-only rejection ──

    fn internal_bytestring_set_oob_key() {
        // BytestringSet with an out-of-bounds key pointer, issued raw so the
        // pointer never passes through the SDK. The parse-fault check runs
        // before the read-only-frame check, so this is Fault (not
        // InvalidSyscallInput) even inside a read-only frame.
        let err_code = unsafe {
            direct_syscall(
                sys::SyscallNo::kBytestringSet.0 as SyscallWord,
                0x00FF_0000, // key pointer far beyond guest memory
                0,           // value pointer
                0,           // value length
                0,           // flags
                0,
                0,
                0,
            )
        };
        assert!(err_code == SyscallErrorCode::kSyscallErrorCodeFault.0);
    }

    fn test_readonly_fault_beats_readonly_rejection(target: &ContractAddress) {
        let entrypoint =
            crate::SyscallMiscTestsEntrypoints::internal_bytestring_set_oob_key
                .into_u64();
        let mut outputs = sys::CopyableInvokeOutputs { return_buffer_len: 0, returned_auth_token_len: 0, frame_status: UNWRITTEN_FRAME_STATUS };
        let res = sys::raw_Invoke(
            target,
            &[],
            &[],
            &sys::CopyableInvokeInputs { entrypoint_id: entrypoint, gas_limit: u64::MAX, flags: 0x2 }, // ReadOnly
            &mut outputs,
        );
        expect_frame_status(res, &outputs, InvocationFrameStatusCode::Return);
    }

    // ── No-Return-syscall test ────────────────────────────────────────

    fn test_invoke_no_return_syscall(target: &ContractAddress) {
        let mut outputs = sys::CopyableInvokeOutputs { return_buffer_len: 0, returned_auth_token_len: 0, frame_status: UNWRITTEN_FRAME_STATUS };
        // Invoke the raw entrypoint 0xFF on a separate contract. Its body
        // returns without calling the Return syscall (see lib.rs). The two
        // runtimes define different semantics for that, asserted per target
        // below.
        let res = sys::raw_Invoke(
            target,
            &[],
            &[],
            &sys::CopyableInvokeInputs { entrypoint_id: 0xFF, gas_limit: u64::MAX, flags: 0 },
            &mut outputs,
        );

        // rv32: the SDK's `_start` dispatcher ebreaks when a wrapper returns
        // without the Return syscall, so the child frame faults
        // deterministically.
        #[cfg(target_arch = "riscv32")]
        expect_frame_status(res, &outputs, InvocationFrameStatusCode::Fault);

        // Host mock: a bare return maps to a successful Return with an
        // empty return buffer.
        #[cfg(not(target_arch = "riscv32"))]
        {
            assert!(res.is_ok(), "invoke of noop should succeed");
            assert_eq!(outputs.return_buffer_len, 0, "return buffer should be empty");

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
    }

    // ── Return auth token edge-case internal helpers ────────────────

    fn internal_return_misaligned_auth_tokens() {
        let auth_tokens: [u8; 3] = [0, 0, 0]; // not a multiple of sizeof(u32)
        unsafe {
            direct_syscall(
                K_SYSCALL_RETURN,
                0,
                0, // empty buffer
                0, // revert_frame = false
                auth_tokens.as_ptr() as SyscallWord,
                auth_tokens.len() as SyscallWord,
                0,
                0,
            );
        }
        unreachable!()
    }

    fn internal_return_revert_with_auth_tokens() {
        let auth_tokens: [u32; 1] = [0];
        let ptr = auth_tokens.as_ptr() as *const u8;
        let len = core::mem::size_of_val(&auth_tokens);
        unsafe {
            direct_syscall(
                K_SYSCALL_RETURN,
                0,
                0, // empty buffer
                1, // revert_frame = true
                ptr as SyscallWord,
                len as SyscallWord,
                0,
                0,
            );
        }
        unreachable!()
    }

    fn internal_return_with_empty_auth_tokens() {
        let buf = [42u8; 4];
        unsafe {
            direct_syscall(
                K_SYSCALL_RETURN,
                buf.as_ptr() as SyscallWord,
                buf.len() as SyscallWord,
                0, // revert_frame = false
                0, // null ptr
                0, // len = 0
                0,
                0,
            );
        }
        unreachable!()
    }

    fn internal_return_with_auth_token_ids() {
        let buf = [99u8; 4];
        let auth_tokens: [u32; 2] = [0, 1];
        let ptr = auth_tokens.as_ptr() as *const u8;
        let len = core::mem::size_of_val(&auth_tokens);
        unsafe {
            direct_syscall(
                K_SYSCALL_RETURN,
                buf.as_ptr() as SyscallWord,
                buf.len() as SyscallWord,
                0, // revert_frame = false
                ptr as SyscallWord,
                len as SyscallWord,
                0,
                0,
            );
        }
        unreachable!()
    }

    // ── Return auth token edge-case tests ───────────────────────────

    fn test_return_misaligned_auth_tokens(target: &ContractAddress) {
        let entrypoint =
            crate::SyscallMiscTestsEntrypoints::internal_return_misaligned_auth_tokens
                .into_u64();
        let mut outputs = sys::CopyableInvokeOutputs { return_buffer_len: 0, returned_auth_token_len: 0, frame_status: UNWRITTEN_FRAME_STATUS };
        let res = sys::raw_Invoke(
            target,
            &[],
            &[],
            &sys::CopyableInvokeInputs { entrypoint_id: entrypoint, gas_limit: u64::MAX, flags: 0 },
            &mut outputs,
        );
        expect_frame_status(res, &outputs, InvocationFrameStatusCode::Fault);
    }

    fn test_return_revert_with_auth_tokens(target: &ContractAddress) {
        let entrypoint =
            crate::SyscallMiscTestsEntrypoints::internal_return_revert_with_auth_tokens
                .into_u64();
        let mut outputs = sys::CopyableInvokeOutputs { return_buffer_len: 0, returned_auth_token_len: 0, frame_status: UNWRITTEN_FRAME_STATUS };
        let res = sys::raw_Invoke(
            target,
            &[],
            &[],
            &sys::CopyableInvokeInputs { entrypoint_id: entrypoint, gas_limit: u64::MAX, flags: 0 },
            &mut outputs,
        );
        expect_frame_status(res, &outputs, InvocationFrameStatusCode::Fault);
    }

    fn test_return_with_empty_auth_tokens(target: &ContractAddress) {
        let entrypoint =
            crate::SyscallMiscTestsEntrypoints::internal_return_with_empty_auth_tokens
                .into_u64();
        let mut outputs = sys::CopyableInvokeOutputs { return_buffer_len: 0, returned_auth_token_len: 0, frame_status: UNWRITTEN_FRAME_STATUS };
        let res = sys::raw_Invoke(
            target,
            &[],
            &[],
            &sys::CopyableInvokeInputs { entrypoint_id: entrypoint, gas_limit: u64::MAX, flags: 0 },
            &mut outputs,
        );
        expect_frame_status(res, &outputs, InvocationFrameStatusCode::Return);

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

    fn test_return_with_auth_token_ids(target: &ContractAddress) {
        let entrypoint =
            crate::SyscallMiscTestsEntrypoints::internal_return_with_auth_token_ids
                .into_u64();
        let mut outputs = sys::CopyableInvokeOutputs { return_buffer_len: 0, returned_auth_token_len: 0, frame_status: UNWRITTEN_FRAME_STATUS };
        let res = sys::raw_Invoke(
            target,
            &[],
            &[],
            &sys::CopyableInvokeInputs { entrypoint_id: entrypoint, gas_limit: u64::MAX, flags: 0 },
            &mut outputs,
        );
        // The child claims to return token IDs it doesn't own.
        // Ownership validation causes a deterministic exit (fault).
        expect_frame_status(res, &outputs, InvocationFrameStatusCode::Fault);
    }
}
