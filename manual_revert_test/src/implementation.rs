use crate::interface::*;
use thruput_sdk::*;

struct ManualRevertTestContract;

#[contract]
impl ManualRevertTest for ManualRevertTestContract {
    fn test_initializer(
        expected_sender: ContractAddress,
        marker: u32,
        mode: u32,
    ) -> [u8; 1024] {
        let sender = thruput_sdk::get_caller();
        assert_eq!(sender, expected_sender);
        let mut calldata = thruput_sdk::internal::new_scratch();
        let auth_token_count = thruput_sdk::fetch_calldata_and_auth_tokens(&mut calldata);
        assert_eq!(auth_token_count, 0);
        let mut state = [0u8; 36];
        state[..32].copy_from_slice(sender.as_ref());
        state[32..].copy_from_slice(&marker.to_be_bytes());
        bytestring_set(&static_key!("initialized"), &state);
        if mode == 1 {
            throw(ManualRevertTestError::BasicRevert);
        }
        if mode == 2 {
            thruput_sdk::trap();
        }
        [0xAB; 1024]
    }

    fn test_basic_manual_revert(
        should_revert: bool,
    ) {
        let test_key: StorageKeyContractData = static_key!("test_basic_revert");
        bytestring_set(&test_key, b"state_before_revert");

        if should_revert {
            throw(ManualRevertTestError::BasicRevert);
        }
    }

    fn test_error_type_revert(
        error_type: u32,
    ) {
        if error_type != 0 {
            throw(ManualRevertTestError::ErrorTypeRevert);
        }
    }

    fn test_nested_manual_revert(
        child_should_revert: bool,
    ) {
        let parent_key: StorageKeyContractData = static_key!("parent_state");
        bytestring_set(&parent_key, b"parent_data");

        Self::internal_revert_method(child_should_revert);
    }

    fn test_storage_manual_revert(
        should_revert: bool,
    ) {
        let key1: StorageKeyContractData = static_key!("storage_test_1");
        let key2: StorageKeyContractData = static_key!("storage_test_2");
        let key3: StorageKeyContractData = static_key!("storage_test_3");

        bytestring_set(&key1, b"value1");
        bytestring_set(&key2, b"backup_data");
        bytestring_set(&key3, b"counter_value");

        if should_revert {
            throw(ManualRevertTestError::StorageRevert);
        }
    }

    fn internal_revert_method(
        should_revert: bool,
    ) {
        let internal_key: StorageKeyContractData =
            static_key!("internal_state");
        bytestring_set(&internal_key, b"internal_data");

        if should_revert {
            throw(ManualRevertTestError::NestedRevert);
        }
    }

    fn check_storage_exists() -> bool {
        let test_key: StorageKeyContractData = static_key!("test_basic_revert");
        bytestring_exists(&test_key)
    }
}
