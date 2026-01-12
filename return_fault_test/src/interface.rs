use sdk::*;

#[contract(client = Client)]
pub trait ReturnFaultTest {
    /// Test kInvokedAddressSignature authclaim from index 0
    fn run_test(&self) -> ();

    fn test_inner(&self) -> ();
}

