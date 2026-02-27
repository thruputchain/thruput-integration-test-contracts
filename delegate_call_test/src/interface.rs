use sdk::*;

#[contract(client = Client)]
pub trait DelegateCallTest {
    // Returns (current_address, msg_sender)
    fn test_delegate_call_context(
        &self,
    ) -> (ContractAddress, ContractAddress);
    fn test_normal_invoke_context(
        &self,
    ) -> (ContractAddress, ContractAddress);
    fn test_delegate_call(
        &self,
        target_address: ContractAddress,
    );
    fn test_normal_invoke(
        &self,
        target_address: ContractAddress,
    );
}
