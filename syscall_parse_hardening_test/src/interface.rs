use sdk::*;

#[contract(client = Client)]
pub trait SyscallParseHardeningTest {
    fn test_verify_signature_noncanonical_scheme_fault();
    fn test_change_pubkey_optional_pointer_fault();
    fn test_get_deploy_address_optional_pointer_sentinel();
}
