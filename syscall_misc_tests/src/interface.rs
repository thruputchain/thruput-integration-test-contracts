use sdk::*;

#[contract(client = Client)]
pub trait SyscallMiscTests {
    // Existing tests
    fn test_get_return_before_return_buffer_set();
    fn test_get_return_after_fault();

    // Internal helpers (invoked via raw_Invoke from other test methods)
    fn internal_always_fault();
    fn internal_always_revert();
    fn internal_return_too_long();
    fn internal_try_bytestring_set();

    // Documented error conditions
    fn test_get_msg_sender_root();
    fn test_invoke_contract_not_found();
    fn test_invoke_frame_reverted();
    fn test_verify_sig_bad_scheme();
    fn test_verify_sig_bad_key_size();
    fn test_verify_sig_bad_signature();
    fn test_bs_get_len_key_not_exist();
    fn test_bs_get_len_wrong_type();
    fn test_bs_get_key_not_exist();
    fn test_bs_get_wrong_type();
    fn test_bs_set_too_long();
    fn test_bs_set_invalid_flags();
    fn test_nnint_get_key_not_exist();
    fn test_nnint_get_wrong_type();
    fn test_nnint_set_add_negative();
    fn test_nnint_set_add_invalid_flags();
    fn test_delete_invalid_flags();
    fn test_rcq_insert_too_large();
    fn test_rcq_insert_wrong_type();
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
    fn test_return_buffer_too_long();

    // Additional documented error conditions
    fn test_deploy_contract_already_deployed(contract_hash: &SHA256Hash);

    // Undocumented edge cases
    fn test_invoke_invalid_flags();
    fn test_write_syscall_in_read_only();

    // Verify wasm function that returns without calling Return syscall
    fn test_invoke_no_return_syscall(target: &ContractAddress);

    // Return auth token edge-case internal helpers
    fn internal_return_misaligned_auth_tokens();
    fn internal_return_revert_with_auth_tokens();
    fn internal_return_with_empty_auth_tokens();
    fn internal_return_with_auth_token_ids();

    // Return auth token edge-case tests
    fn test_return_misaligned_auth_tokens();
    fn test_return_revert_with_auth_tokens();
    fn test_return_with_empty_auth_tokens();
    fn test_return_with_auth_token_ids();
}
