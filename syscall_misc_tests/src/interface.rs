use sdk::*;

#[contract(client = Client)]
pub trait SyscallMiscTests {
    // Existing tests
    fn test_get_return_before_return_buffer_set(&self);
    fn test_get_return_after_fault(&self);

    // Internal helpers (invoked via raw_Invoke from other test methods)
    fn internal_always_fault(&self);
    fn internal_always_revert(&self);
    fn internal_return_too_long(&self);
    fn internal_try_bytestring_set(&self);

    // Documented error conditions
    fn test_get_msg_sender_root(&self);
    fn test_invoke_contract_not_found(&self);
    fn test_invoke_frame_reverted(&self);
    fn test_verify_sig_bad_scheme(&self);
    fn test_verify_sig_bad_key_size(&self);
    fn test_verify_sig_bad_signature(&self);
    fn test_bs_get_len_key_not_exist(&self);
    fn test_bs_get_len_wrong_type(&self);
    fn test_bs_get_key_not_exist(&self);
    fn test_bs_get_wrong_type(&self);
    fn test_bs_set_too_long(&self);
    fn test_bs_set_invalid_flags(&self);
    fn test_nnint_get_key_not_exist(&self);
    fn test_nnint_get_wrong_type(&self);
    fn test_nnint_set_add_negative(&self);
    fn test_nnint_set_add_invalid_flags(&self);
    fn test_delete_invalid_flags(&self);
    fn test_rcq_insert_too_large(&self);
    fn test_rcq_insert_wrong_type(&self);
    fn test_rcq_consume_empty(&self);
    fn test_rcq_consume_wrong_type(&self);
    fn test_log_event_data_too_long(&self);
    fn test_log_event_noncontinuous_topics(&self);
    fn test_get_auth_claim_oob(&self);
    fn test_sha256_update_no_buffer(&self);
    fn test_sha256_update_too_many_buffers(&self);
    fn test_sha256_update_buffer_too_large(&self);
    fn test_sha256_finalize_no_buffer(&self);
    fn test_sha256_finalize_double(&self);
    fn test_get_tx_stats_invalid(&self);
    fn test_change_pubkey_index_oob(&self);
    fn test_gas_perms_non_initial_segment(&self);
    fn test_deploy_contract_not_available(&self);

    // Additional behavioral tests
    fn test_return_buffer_too_long(&self);

    // Additional documented error conditions
    fn test_deploy_contract_already_deployed(&self, contract_hash: &SHA256Hash);

    // Undocumented edge cases
    fn test_invoke_invalid_flags(&self);
    fn test_write_syscall_in_read_only(&self);
}
