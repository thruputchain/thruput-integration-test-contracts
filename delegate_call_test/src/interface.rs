use thruput_sdk::*;

#[contract(client = Client)]
pub trait DelegateCallTest {
    // Returns (current_address, msg_sender)
    fn test_delegate_call_context(
    ) -> (ContractAddress, ContractAddress);
    fn test_normal_invoke_context(
    ) -> (ContractAddress, ContractAddress);
    fn test_delegate_call(
        target_address: ContractAddress,
    );
    fn test_normal_invoke(
        target_address: ContractAddress,
    );
}
