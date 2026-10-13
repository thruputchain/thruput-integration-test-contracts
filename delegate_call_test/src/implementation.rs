//! An implementation of DelegateCallTest

use super::interface::*;
use thruput_sdk::*;

struct DelegateCallTestContract;

#[contract]
impl DelegateCallTest for DelegateCallTestContract {
    fn test_delegate_call_context(
    ) -> (ContractAddress, ContractAddress) {
        let current_address = get_current_address();
        let msg_sender = get_caller();
        (current_address, msg_sender)
    }

    fn test_normal_invoke_context(
    ) -> (ContractAddress, ContractAddress) {
        let current_address = get_current_address();
        let msg_sender = get_caller();
        (current_address, msg_sender)
    }

    fn test_delegate_call(
        _target_address: ContractAddress,
    ) {
        // TODO: Implement delegate call to target_address
        // This will be implemented when the SDK supports guest-to-guest delegate calls
    }

    fn test_normal_invoke(
        _target_address: ContractAddress,
    ) {
        // TODO: Implement normal invoke to target_address
        // This will be implemented when the SDK supports guest-to-guest calls
    }
}

