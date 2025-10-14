use sdk::*;

pub use sdk::contract_result::*;

pub type ManualRevertTestError = ::sdk::contract_result::Error;

#[contract(client = "Client", err_type = ManualRevertTestError)]
pub trait ManualRevertTest {
    /// Tests basic manual revert functionality.
    /// When should_revert=true, writes storage then returns Err (triggering manual revert).
    /// When should_revert=false, writes storage and returns Ok (normal success).
    /// The manual revert should preserve error data while reverting storage changes.
    fn test_basic_manual_revert(
        &self,
        should_revert: bool,
    ) -> Result<(), ManualRevertTestError>;

    /// Tests different error types to verify they all trigger manual revert.
    /// Uses contract_error! and caller_error! macros to generate different error variants.
    /// All non-zero error_type values should trigger manual revert via Result::Err.
    fn test_error_type_revert(
        &self,
        error_type: u32,
    ) -> Result<(), ManualRevertTestError>;

    /// Tests nested manual revert by calling internal_revert_method.
    /// Verifies that manual revert works correctly when triggered from a nested call.
    /// The parent method should be able to handle the child's manual revert gracefully.
    fn test_nested_manual_revert(
        &self,
        child_should_revert: bool,
    ) -> Result<(), ManualRevertTestError>;

    /// Tests manual revert behavior with storage operations.
    /// Writes multiple storage items, then conditionally reverts to test that
    /// all storage changes within the frame are properly reverted on manual revert.
    fn test_storage_manual_revert(
        &self,
        should_revert: bool,
    ) -> Result<(), ManualRevertTestError>;

    /// Internal helper method for testing nested manual revert scenarios.
    /// Called by test_nested_manual_revert to simulate a child contract that may revert.
    /// Demonstrates how manual revert propagates through the call stack.
    fn internal_revert_method(
        &self,
        should_revert: bool,
    ) -> Result<(), ManualRevertTestError>;

    /// Utility method to check if test storage exists.
    /// Returns true if the test storage key exists, false otherwise.
    /// Used to verify that storage changes were properly reverted during manual revert.
    fn check_storage_exists(&self) -> Result<bool, ManualRevertTestError>;
}
