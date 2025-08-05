use sdk::prelude::*;

pub use sdk::prelude::{Error, Result};

#[contract(client = "Client")]
pub trait ManualRevertCaller {
    /// Initialize the contract with the address of the ManualRevertTest contract
    fn initialize(&mut self, address: &ContractAddress);
    
    /// Call the ManualRevertTest contract's test_basic_manual_revert method
    /// This tests cross-contract revert propagation
    fn call_basic_manual_revert(&mut self, should_revert: bool) -> Result<()>;
    
    /// Call the ManualRevertTest contract's test_nested_manual_revert method
    /// This tests nested revert scenarios across contracts
    fn call_nested_manual_revert(&mut self, child_should_revert: bool) -> Result<()>;
    
    /// Call the ManualRevertTest contract's test_error_type_revert method
    /// This tests different error types in cross-contract calls
    fn call_error_type_revert(&mut self, error_type: u32) -> Result<()>;
    
    /// Call the ManualRevertTest contract's test_storage_manual_revert method
    /// This tests storage operations with cross-contract revert
    fn call_storage_manual_revert(&mut self, should_revert: bool) -> Result<()>;
    
    /// Check if the ManualRevertTest contract's storage exists
    /// Used to verify that storage changes were properly reverted
    fn check_target_storage_exists(&mut self) -> bool;
    
    /// Write some state to this contract to verify it persists
    /// while the target contract's state gets reverted
    fn write_own_state(&mut self, data: &[u8]) -> Result<()>;
    
    /// Check if this contract's state exists
    fn check_own_state_exists(&mut self) -> bool;
} 