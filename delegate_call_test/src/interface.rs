use sdk::prelude::*;

pub use sdk::contract_result::*;

#[contract(client = Client)]
pub trait DelegateCallTest {
    // Returns (current_address, msg_sender)
    fn test_delegate_call_context(&self) -> Result<(ContractAddress, ContractAddress)>;
    fn test_normal_invoke_context(&self) -> Result<(ContractAddress, ContractAddress)>;
    fn test_delegate_call(&self, target_address: ContractAddress) -> Result<()>;
    fn test_normal_invoke(&self, target_address: ContractAddress) -> Result<()>;
} 
