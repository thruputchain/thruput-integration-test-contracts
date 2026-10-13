use thruput_sdk::*;

#[contract(client = Client)]
pub trait SyscallParseHardeningTest {
    fn test_verify_signature_noncanonical_scheme_fault();
    fn test_change_pubkey_optional_pointer_fault();
    fn test_get_deploy_address_optional_pointer_sentinel();
    fn test_verify_signature_wrong_size_signature();
    fn test_zero_len_span_oob_pointer_fault();
    fn test_span_end_u32_wrap_fault();
}
