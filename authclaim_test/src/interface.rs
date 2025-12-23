use sdk::*;

pub use sdk::contract_result::*;

#[contract(client = Client)]
pub trait AuthClaimTest {
    /// Test kInvokedAddressSignature authclaim from index 0
    fn test_invoked_address_signature(&self);

    /// Test kInvokedAddressSignedWeight authclaim from index 1
    fn test_invoked_address_signed_weight(&self);

    /// Test kOtherAddressSignature authclaim from index 2
    fn test_other_address_signature(&self);

    /// Test kOtherAddressSignedWeight authclaim from index 3
    fn test_other_address_signed_weight(&self);
}

