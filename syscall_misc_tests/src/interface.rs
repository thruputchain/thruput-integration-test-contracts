use sdk::*;

#[contract(client = Client)]
pub trait SyscallMiscTests {
    // Existing tests
    //
    // Tests that drive one of the `internal_*` helpers below take a `target`
    // address rather than invoking themselves: the reentrance guard rejects a
    // contract that is already on the invocation stack, so the helper has to
    // live at a second deployment of this contract.
    fn test_get_return_before_return_buffer_set();
    fn test_get_return_after_fault(target: &ContractAddress);

    // Internal helpers (invoked via raw_Invoke from other test methods)
    fn internal_always_fault();
    fn internal_always_revert();
    fn internal_return_too_long();
    fn internal_return_exact_max();
    fn internal_try_bytestring_set();

    // Documented error conditions
    fn test_get_msg_sender_root();
    fn test_invoke_contract_not_found();
    fn test_invoke_frame_reverted(target: &ContractAddress);
    fn test_verify_sig_bad_scheme();
    fn test_verify_sig_bad_key_size();
    fn test_verify_sig_bad_signature();
    fn test_bs_get_len_key_not_exist();
    fn test_bs_get_len_wrong_type();
    fn test_bs_get_key_not_exist();
    fn test_bs_get_wrong_type();
    fn test_bs_set_too_long();
    fn test_nnint_get_key_not_exist();
    fn test_nnint_get_wrong_type();
    fn test_nnint_set_add_negative();
    fn test_rcq_insert_too_large();
    fn test_rcq_insert_over_bytestring();
    fn test_rcq_consume_empty();
    fn test_rcq_consume_wrong_type();
    fn test_log_event_data_too_long();
    fn test_log_event_noncontinuous_topics();
    fn test_sha256_update_no_buffer();
    fn test_sha256_update_too_many_buffers();
    fn test_sha256_update_buffer_too_large();
    fn test_sha256_finalize_no_buffer();
    fn test_sha256_finalize_double();
    fn test_get_tx_stats_invalid();
    fn test_change_pubkey_index_oob();
    fn test_deploy_contract_not_available();

    // Additional behavioral tests
    // `chain_limit` is the caller's `kConfigParamMaxReturnBufferLen`. These
    // two size buffers from a constant, since an array's length must be known
    // at compile time, so they assert the two agree rather than silently
    // probing the wrong boundary if the chain's default moves.
    fn test_return_buffer_too_long(target: &ContractAddress, chain_limit: u32);
    fn test_return_buffer_exact_max(target: &ContractAddress, chain_limit: u32);

    // Additional documented error conditions
    fn test_deploy_contract_already_deployed(contract_hash: &SHA256Hash);

    // Undocumented edge cases
    fn test_invoke_invalid_flags();
    fn test_write_syscall_in_read_only(target: &ContractAddress);

    // Verify a raw entrypoint that returns without calling the Return syscall
    fn test_invoke_no_return_syscall(target: &ContractAddress);

    // The other half of that pair: a panicking entrypoint faults instead,
    // with no payload of any kind.
    fn internal_always_panic();
    fn test_invoke_panic_faults(target: &ContractAddress);

    // Error-code precedence pins: the order of checks decides which code wins
    // when one input violates multiple constraints.
    fn internal_rcq_obs_cap_seed(count: u32);
    fn test_rcq_cap_precedence(count: u32);
    fn test_get_returned_buffer_no_buffer_beats_misaligned();
    fn test_invoke_unowned_token_loses_to_reentrance();
    fn test_invoke_self_is_reentrance();
    fn internal_bytestring_set_oob_key();
    fn test_readonly_fault_beats_readonly_rejection(target: &ContractAddress);

    // Return auth token edge-case internal helpers
    fn internal_return_misaligned_auth_tokens();
    fn internal_return_revert_with_auth_tokens();
    fn internal_return_with_empty_auth_tokens();
    fn internal_return_with_auth_token_ids();

    // Return auth token edge-case tests
    fn test_return_misaligned_auth_tokens(target: &ContractAddress);
    fn test_return_revert_with_auth_tokens(target: &ContractAddress);
    fn test_return_with_empty_auth_tokens(target: &ContractAddress);
    fn test_return_with_auth_token_ids(target: &ContractAddress);

    // Cross-invoke depth precedence pins. These need an rv32 caller: hogd's
    // native syscall entry takes a typed span of uint32_t token ids, whose
    // length is a count of ids, so it cannot express a misaligned byte span.
    fn test_invoke_misaligned_tokens_beat_depth(target: &ContractAddress);
    fn test_invoke_depth_refused(target: &ContractAddress);
}
