use crate::interface::*;
use alloc::format;
use sdk::*;

//
// Implementation of ManualRevertTest contract demonstrating manual frame revert functionality.
//
// Manual revert in the SDK works through the Returnable trait:
// - When a contract method returns Result::Err, the SDK calls force_revert() which returns true
// - This triggers the Return syscall with revert_frame=true
// - State changes within the current frame are reverted
// - The transaction still commits successfully (unlike normal failures)
// - Error data is serialized and returned to the caller
//
// This behavior is similar to EVM's REVERT opcode, allowing "controlled failures"
// where you can signal an error condition while maintaining transaction validity.
//
#[derive(Default)]
struct ManualRevertTestContract;

#[contract(exports = "guest_exports", storage = "false", err_type = ManualRevertTestError)]
impl ManualRevertTest for ManualRevertTestContract {
    // Test basic manual revert functionality
    // Demonstrates how the Returnable trait automatically triggers manual revert on Err
    fn test_basic_manual_revert(
        &self,
        should_revert: bool,
    ) {
        // Write some state first
        let test_key: StorageKeyContractData = static_key!("test_basic_revert");
        bytestring_set(&test_key, b"state_before_revert");

        if should_revert {
            let syscall = sdk::sys::kBytestringSet;
            let syserr = SysErr::from(SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput);

            // When we abort, the SDK automatically:
            // 1. Triggers Return syscall with revert_frame=true
            // 2. Serializes error data and makes it available to parent
            // 3. Reverts state changes but commits transaction successfully
            sdk::syscall_abort2(syscall, syserr);
        }
    }

    // Test nested calls with manual revert
    fn test_nested_manual_revert(
        &self,
        child_should_revert: bool,
    ) -> Result<(), ManualRevertTestError> {
        // Parent writes state
        let parent_key: StorageKeyContractData = static_key!("parent_state");
        bytestring_set(&parent_key, b"parent_data")?;

        // Call internal method that may revert
        self.internal_revert_method(child_should_revert)
    }

    // Test storage operations with manual revert
    fn test_storage_manual_revert(
        &self,
        should_revert: bool,
    ) -> Result<(), ManualRevertTestError> {
        // Write multiple storage items
        let key1: StorageKeyContractData = static_key!("storage_test_1");
        let key2: StorageKeyContractData = static_key!("storage_test_2");
        let key3: StorageKeyContractData = static_key!("storage_test_3");

        bytestring_set(&key1, b"value1").unwrap();
        bytestring_set(&key2, b"backup_data").unwrap();
        bytestring_set(&key3, b"counter_value").unwrap();

        if should_revert {
            // All storage changes above will be reverted due to Returnable trait
            return internal_err("Storage operations manually reverted");
        }

        Ok(())
    }

    // Internal method for nested testing - manually reverts with data
    fn internal_revert_method(
        &self,
        should_revert: bool,
    ) -> Result<(), ManualRevertTestError> {
        // Write some internal state
        let internal_key: StorageKeyContractData =
            static_key!("internal_state");
        bytestring_set(&internal_key, b"internal_data")?;

        if should_revert {
            return Err(SysErr::from(
                SyscallErrorCode::kSyscallErrorCodeNoReturnBufferAvailable,
            )
            .into());
        }

        Ok(())
    }

    // Check if storage was actually reverted - returns true if key exists
    fn check_storage_exists(&self) -> Result<bool, ManualRevertTestError> {
        let test_key: StorageKeyContractData = static_key!("test_basic_revert");
        let res = bytestring_get(&test_key).is_ok();
        Ok(res)
    }
}
