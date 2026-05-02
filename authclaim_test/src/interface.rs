use sdk::*;

#[contract(client = Client)]
pub trait AuthClaimTest {
    fn test_get_auth_claim(&self);

    fn test_set_address_required_weight(&mut self);

    fn test_add_address_public_key(&mut self);

    fn test_remove_address_public_key(&mut self);

    fn test_add_pubkey_index_oob(&self);
}

