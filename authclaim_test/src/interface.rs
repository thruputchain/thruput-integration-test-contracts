use thruput_sdk::*;

#[contract(client = Client)]
pub trait AuthClaimTest {
    fn test_get_auth_claim();

    fn test_set_address_required_weight();

    fn test_add_address_public_key();

    fn test_remove_address_public_key();

    fn test_add_pubkey_index_oob();
}

