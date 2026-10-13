use thruput_sdk::*;

#[contract(client = Client)]
pub trait NnintOutputFaultTest {
    fn test_fault_valid_first();
    fn test_fault_invalid_first();
    fn test_validation_failure();
}
