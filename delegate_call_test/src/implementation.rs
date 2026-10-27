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
        // Deliberately a no-op: the SDK has no guest-side delegate call, and
        // only the `*_context` methods are driven by tests.
    }

    fn test_normal_invoke(
        _target_address: ContractAddress,
    ) {
        // Deliberately a no-op, like `test_delegate_call`: only the
        // `*_context` methods are driven by tests.
    }
}

