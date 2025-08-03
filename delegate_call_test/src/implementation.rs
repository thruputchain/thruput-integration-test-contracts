//! An implementation of DelegateCallTest

use super::interface::DelegateCallTest;
use sdk::prelude::*;

#[derive(Default)]
struct DelegateCallTestContract;

#[contract(exports = "guest_exports", storage = "false")]
impl DelegateCallTest for DelegateCallTestContract {
    fn test_delegate_call_context(&self) -> Result<(ContractAddress, ContractAddress)> {
        let current_address = get_current_address()?;
        let msg_sender = get_caller()?;
        Ok((current_address, msg_sender))
    }

    fn test_normal_invoke_context(&self) -> Result<(ContractAddress, ContractAddress)> {
        let current_address = get_current_address()?;
        let msg_sender = get_caller()?;
        Ok((current_address, msg_sender))
    }

    fn test_delegate_call(&self, _target_address: ContractAddress) -> Result<()> {
        // TODO: Implement delegate call to target_address
        // This will be implemented when SDK supports WASM-to-WASM delegate calls
        Ok(())
    }

    fn test_normal_invoke(&self, _target_address: ContractAddress) -> Result<()> {
        // TODO: Implement normal invoke to target_address
        // This will be implemented when SDK supports WASM-to-WASM calls
        Ok(())
    }
}

#[unsafe(export_name = "FFFFFFFFFFFFFFFF")]
pub extern "C" fn startup_null() -> () {
    ()
} 