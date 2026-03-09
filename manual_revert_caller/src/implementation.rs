use crate::interface::*;
use alloc::format;
use manual_revert_test::ManualRevertTestError;
use manual_revert_test::Client as ManualRevertTestClient;
use sdk::*;
use sdk::call::{CallResult, CallError};

//
// Implementation of ManualRevertCaller contract that tests cross-contract revert propagation.
//

#[storage]
pub struct ManualRevertCallerContract {
    target_address: ContractAddress,
}

impl ManualRevertCallerContract {
    fn check_result(
        result: &CallResult<(), ManualRevertTestError>,
        should_revert: bool,
    ) {
        log(format!(
            "doing check_result: should_revert {:?} res {:?}",
            should_revert, result
        )
        .as_bytes());
        match result {
            Ok(()) => {
                if should_revert {
                    panic!("Manual revert should have failed");
                }
            }
            Err(e) => match e {
                CallError::Fault(_) => {
                    panic!("Fault");
                }
                CallError::Abort(_) => {
                    if !should_revert {
                        panic!("Manual revert should not have failed");
                    }
                }
            },
        }
    }
}

#[contract]
impl ManualRevertCaller for ManualRevertCallerContract {
    /// Initialize the contract with the target ManualRevertTest contract address
    #[init]
    fn initialize(&mut self, address: &ContractAddress) {
        self.target_address = *address;
    }

    /// Call the target contract's test_basic_manual_revert method
    fn call_basic_manual_revert(&mut self, should_revert: bool) {
        let own_key: StorageKeyContractData = static_key!("caller_state");
        bytestring_set(&own_key, b"caller_data_before_call");

        let mut client = ManualRevertTestClient::new(&self.target_address);
        let result = client.try_test_basic_manual_revert(should_revert);

        log(format!("result as {:?}", result).as_bytes());

        Self::check_result(&result, should_revert);
    }

    /// Call the target contract's test_nested_manual_revert method
    fn call_nested_manual_revert(&mut self, child_should_revert: bool) {
        let own_key: StorageKeyContractData =
            static_key!("caller_nested_state");
        bytestring_set(&own_key, b"caller_nested_data");

        let mut client = ManualRevertTestClient::new(&self.target_address);
        let result = client.try_test_nested_manual_revert(child_should_revert);

        Self::check_result(&result, child_should_revert);
    }

    /// Call the target contract's test_error_type_revert method
    fn call_error_type_revert(&mut self, error_type: u32) {
        let own_key: StorageKeyContractData =
            static_key!("caller_error_state");
        bytestring_set(&own_key, b"caller_error_data");

        let mut client = ManualRevertTestClient::new(&self.target_address);
        let result = client.try_test_error_type_revert(error_type);

        Self::check_result(&result, true);
    }

    /// Call the target contract's test_storage_manual_revert method
    fn call_storage_manual_revert(&mut self, should_revert: bool) {
        let own_key: StorageKeyContractData =
            static_key!("caller_storage_state");
        bytestring_set(&own_key, b"caller_storage_data");

        let mut client = ManualRevertTestClient::new(&self.target_address);
        let result = client.try_test_storage_manual_revert(should_revert);

        Self::check_result(&result, should_revert);
    }

    /// Check if the target contract's storage exists
    fn check_target_storage_exists(&mut self) -> bool {
        let mut client = ManualRevertTestClient::new(&self.target_address);
        client.check_storage_exists()
    }

    /// Write some state to this contract
    fn write_own_state(&mut self, data: &[u8]) {
        let own_key: StorageKeyContractData =
            static_key!("caller_persistent_state");
        bytestring_set(&own_key, data);
    }

    /// Check if this contract's state exists
    fn check_own_state_exists(&mut self) -> bool {
        let own_key: StorageKeyContractData =
            static_key!("caller_persistent_state");
        bytestring_exists(&own_key)
    }
}
