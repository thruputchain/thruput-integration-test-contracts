use sdk::*;

#[contract_exception]
#[derive(Copy, Clone)]
pub enum ManualRevertTestError {
    BasicRevert = 1,
    ErrorTypeRevert = 2,
    NestedRevert = 3,
    StorageRevert = 4,
}

#[contract(client = "Client", throws = ManualRevertTestError)]
pub trait ManualRevertTest {
    /// Tests basic manual revert functionality.
    /// When should_revert=true, writes storage then aborts (triggering manual revert).
    /// When should_revert=false, writes storage and returns normally.
    fn test_basic_manual_revert(
        should_revert: bool,
    );

    /// Tests different error types to verify they all trigger manual revert.
    /// All non-zero error_type values should trigger manual revert via abort.
    fn test_error_type_revert(
        error_type: u32,
    );

    /// Tests nested manual revert by calling internal_revert_method.
    /// Verifies that manual revert works correctly when triggered from a nested call.
    fn test_nested_manual_revert(
        child_should_revert: bool,
    );

    /// Tests manual revert behavior with storage operations.
    /// Writes multiple storage items, then conditionally reverts to test that
    /// all storage changes within the frame are properly reverted on manual revert.
    fn test_storage_manual_revert(
        should_revert: bool,
    );

    /// Internal helper method for testing nested manual revert scenarios.
    fn internal_revert_method(
        should_revert: bool,
    );

    /// Utility method to check if test storage exists.
    fn check_storage_exists() -> bool;
}
