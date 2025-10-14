use crate::interface::*;
use alloc::format;
use manual_revert_test::Client as ManualRevertTestClient;
use sdk::*;

//
// Implementation of ManualRevertCaller contract that tests cross-contract revert propagation.
//
// This contract calls into the ManualRevertTest contract to test various revert scenarios:
// - Basic manual revert across contract boundaries
// - Nested revert scenarios
// - Different error types
// - Storage operations with revert
//
// The tests verify that:
// 1. This contract's state persists when target contract reverts
// 2. Target contract's state is properly reverted
// 3. Error data flows correctly through cross-contract calls
// 4. Complex nested scenarios work correctly
//

#[storage]
pub struct ManualRevertCallerContract {
    target_address: ContractAddress,
}

impl ManualRevertCallerContract {
    fn check_result(
        result: &ClientResult<(), sdk::contract_result::Error>,
        should_revert: bool,
    ) {
        log(format!(
            "doing check_result: should_revert {:?} res {:?}",
            should_revert, result
        )
        .as_bytes())
        .unwrap();
        match result {
            Ok(()) => {
                if should_revert {
                    panic!("Manual revert should have failed");
                } else {
                    return;
                }
            }
            Err(e) => match e {
                ClientError::AbortDecodeFailed => {
                    panic!("AbortDecodeFailed");
                }
                ClientError::Invoke(_) => {
                    panic!("Invoke");
                }
                ClientError::Aborted(_) => {
                    if should_revert {
                        return;
                    } else {
                        panic!("Manual revert should have failed");
                    }
                }
            },
        }
    }
}

#[contract(exports = "guest_exports")]
impl ManualRevertCaller for ManualRevertCallerContract {
    /// Initialize the contract with the target ManualRevertTest contract address
    fn initialize(&mut self, address: &ContractAddress) {
        self.target_address = *address;
    }

    /// Call the target contract's test_basic_manual_revert method
    /// Tests basic cross-contract revert propagation
    fn call_basic_manual_revert(&mut self, should_revert: bool) -> () {
        // Write some state to this contract first
        let own_key: StorageKeyContractData = static_key!("caller_state");
        bytestring_set(&own_key, b"caller_data_before_call").unwrap();

        // Create client for target contract using the stored target_address
        let mut client = ManualRevertTestClient::new(&self.target_address);

        // Call the target contract's method
        let result: ClientResult<(), sdk::contract_result::Error> =
            client.test_basic_manual_revert(should_revert);

        log(format!("result as {:?}", result).as_bytes()).unwrap();

        Self::check_result(&result, should_revert);
    }

    /// Call the target contract's test_nested_manual_revert method
    /// Tests nested revert scenarios across contracts
    fn call_nested_manual_revert(&mut self, child_should_revert: bool) -> () {
        // Write some state to this contract
        let own_key: StorageKeyContractData =
            static_key!("caller_nested_state");
        bytestring_set(&own_key, b"caller_nested_data").unwrap();

        // Create client for target contract
        let mut client = ManualRevertTestClient::new(&self.target_address);

        // Call the target contract's nested method
        let result = client.test_nested_manual_revert(child_should_revert);

        Self::check_result(&result, child_should_revert);
    }

    /// Call the target contract's test_error_type_revert method
    /// Tests different error types in cross-contract calls
    fn call_error_type_revert(&mut self, error_type: u32) -> () {
        // Write some state to this contract
        let own_key: StorageKeyContractData =
            static_key!("caller_error_state");
        bytestring_set(&own_key, b"caller_error_data").unwrap();

        // Create client for target contract
        let mut client = ManualRevertTestClient::new(&self.target_address);

        // Call the target contract's error type method
        let result: ClientResult<(), sdk::contract_result::Error> =
            client.test_error_type_revert(error_type);

        Self::check_result(&result, true);
    }

    /// Call the target contract's test_storage_manual_revert method
    /// Tests storage operations with cross-contract revert
    fn call_storage_manual_revert(&mut self, should_revert: bool) -> () {
        // Write some state to this contract
        let own_key: StorageKeyContractData =
            static_key!("caller_storage_state");
        bytestring_set(&own_key, b"caller_storage_data").unwrap();

        // Create client for target contract
        let mut client = ManualRevertTestClient::new(&self.target_address);

        // Call the target contract's storage method
        let result: ClientResult<(), sdk::contract_result::Error> =
            client.test_storage_manual_revert(should_revert);

        Self::check_result(&result, should_revert);
    }

    /// Check if the target contract's storage exists
    /// Used to verify that storage changes were properly reverted
    fn check_target_storage_exists(&mut self) -> bool {
        let mut client = ManualRevertTestClient::new(&self.target_address);
        client.check_storage_exists().unwrap_or(false)
    }

    /// Write some state to this contract to verify it persists
    /// while the target contract's state gets reverted
    fn write_own_state(&mut self, data: &[u8]) -> () {
        let own_key: StorageKeyContractData =
            static_key!("caller_persistent_state");
        bytestring_set(&own_key, data).unwrap();
    }

    /// Check if this contract's state exists
    fn check_own_state_exists(&mut self) -> bool {
        let own_key: StorageKeyContractData =
            static_key!("caller_persistent_state");
        bytestring_get(&own_key).is_ok()
    }
}
