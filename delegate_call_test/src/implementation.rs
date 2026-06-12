//! An implementation of DelegateCallTest

use super::interface::*;
use sdk::*;

#[sdk::storage]
struct DelegateCallTestContract;

#[contract]
impl DelegateCallTest for DelegateCallTestContract {
    fn test_delegate_call_context(
        &self,
    ) -> (ContractAddress, ContractAddress) {
        let current_address = get_current_address();
        let msg_sender = get_caller();
        (current_address, msg_sender)
    }

    fn test_normal_invoke_context(
        &self,
    ) -> (ContractAddress, ContractAddress) {
        let current_address = get_current_address();
        let msg_sender = get_caller();
        (current_address, msg_sender)
    }

    fn test_delegate_call(
        &self,
        _target_address: ContractAddress,
    ) {
        // TODO: Implement delegate call to target_address
        // This will be implemented when SDK supports WASM-to-WASM delegate calls
    }

    fn test_normal_invoke(
        &self,
        _target_address: ContractAddress,
    ) {
        // TODO: Implement normal invoke to target_address
        // This will be implemented when SDK supports WASM-to-WASM calls
    }
}

#[unsafe(export_name = "FFFFFFFFFFFFFFFF")]
pub extern "C" fn startup_null() -> () {
    ()
}
