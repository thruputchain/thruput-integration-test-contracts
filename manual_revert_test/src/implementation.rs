use sdk::prelude::*;
use sdk::macros::static_key;

use crate::interface::*;

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

#[contract(exports = "guest_exports", storage = "false")]
impl ManualRevertTest for ManualRevertTestContract {
    
    /// Test basic manual revert functionality
    /// Demonstrates how the Returnable trait automatically triggers manual revert on Err
    fn test_basic_manual_revert(&self, should_revert: bool) -> Result<()> {
        // Write some state first
        let test_key: StorageKeyContractData = static_key!("test_basic_revert");
        bytestring_set(&test_key, b"state_before_revert").unwrap();
        
        if should_revert {
            // When we return Err, the SDK's Returnable trait automatically:
            // 1. Calls force_revert() which returns true for Result::Err
            // 2. Triggers Return syscall with revert_frame=true
            // 3. Serializes error data and makes it available to parent
            // 4. Reverts state changes but commits transaction successfully
            return Err(SysErr::from(SyscallErrorCode::kInvalidSyscallInput).into());
        }
        
        Ok(())
    }
    
    /// Test manual revert with different error types
    fn test_error_type_revert(&self, error_type: u32) -> Result<()> {
        // Write test state
        let key: StorageKeyContractData = static_key!("error_type_test");
        bytestring_set(&key, b"test_data")?;
        
        match error_type {
            1 => {
                // Syscall error triggers manual revert
                Err(SysErr::from(SyscallErrorCode::kStorageKeyNotExist).into())
            },
            2 => {
                // Different syscall error triggers manual revert
                Err(SysErr::from(SyscallErrorCode::kInsufficientGasForInvocation).into())
            },
            3 => {
                // Custom contract error triggers manual revert
                internal_err("Test custom contract error")
            },
            4 => {
                // Caller error triggers manual revert
                caller_error("Test caller error")
            },
            _ => {
                Ok(())
            }
        }
    }
    
    /// Test nested calls with manual revert
    fn test_nested_manual_revert(&self, child_should_revert: bool) -> Result<()> {
        // Parent writes state
        let parent_key: StorageKeyContractData = static_key!("parent_state");
        bytestring_set(&parent_key, b"parent_data")?;
        
        // Call internal method that may revert
        let result = self.internal_revert_method(child_should_revert);
        
        match result {
            Ok(()) => {
                // Child succeeded
                Ok(())
            },
            Err(_e) => {
                // Child manually reverted - in a real system, the parent would receive
                // the child's error data even though child's state was reverted
                // For this test, we'll just succeed to show parent handles child revert
                Ok(())
            }
        }
    }
    
    /// Test storage operations with manual revert
    fn test_storage_manual_revert(&self, should_revert: bool) -> Result<()> {
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
    
    /// Internal method for nested testing - manually reverts with data
    fn internal_revert_method(&self, should_revert: bool) -> Result<()> {
        // Write some internal state
        let internal_key: StorageKeyContractData = static_key!("internal_state");
        bytestring_set(&internal_key, b"internal_data")?;
        
        if should_revert {
            // This triggers manual revert via Returnable trait
            return Err(SysErr::from(SyscallErrorCode::kCalldataParseError).into());
        }
        
        Ok(())
    }
    
    /// Check if storage was actually reverted - returns true if key exists
    fn check_storage_exists(&self) -> bool {
        let test_key: StorageKeyContractData = static_key!("test_basic_revert");
        bytestring_get(&test_key).is_ok()
    }
}
