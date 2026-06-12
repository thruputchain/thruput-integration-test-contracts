use crate::interface::*;
use sdk::*;

#[sdk::storage]
struct ManualRevertTestContract;

#[contract]
impl ManualRevertTest for ManualRevertTestContract {
    fn test_basic_manual_revert(
        &self,
        should_revert: bool,
    ) {
        let test_key: StorageKeyContractData = static_key!("test_basic_revert");
        bytestring_set(&test_key, b"state_before_revert");

        if should_revert {
            throw(ManualRevertTestError::BasicRevert);
        }
    }

    fn test_error_type_revert(
        &self,
        error_type: u32,
    ) {
        if error_type != 0 {
            throw(ManualRevertTestError::ErrorTypeRevert);
        }
    }

    fn test_nested_manual_revert(
        &self,
        child_should_revert: bool,
    ) {
        let parent_key: StorageKeyContractData = static_key!("parent_state");
        bytestring_set(&parent_key, b"parent_data");

        self.internal_revert_method(child_should_revert);
    }

    fn test_storage_manual_revert(
        &self,
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
        &self,
        should_revert: bool,
    ) {
        let internal_key: StorageKeyContractData =
            static_key!("internal_state");
        bytestring_set(&internal_key, b"internal_data");

        if should_revert {
            throw(ManualRevertTestError::NestedRevert);
        }
    }

    fn check_storage_exists(&self) -> bool {
        let test_key: StorageKeyContractData = static_key!("test_basic_revert");
        bytestring_exists(&test_key)
    }
}
