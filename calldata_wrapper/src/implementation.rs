//! An implementation of Fibonacci

use super::interface::CalldataWrapper;
use sdk::*;

use alloc::vec::Vec;

#[derive(Default)]
pub struct CalldataWrapperContract;

// This implements the Fibonacci interface.  The `contract` macro provides
// an exported entrypoint for each method; these will be placed in a
// module with the specified name ("guest_exports").
#[contract(storage = false)]
impl CalldataWrapper for CalldataWrapperContract {
    fn echo(&self, data: &[u8]) -> Vec<u8> {
        data.into()
    }
}

#[unsafe(export_name = "FFFFFFFFFFFFFFFF")]
pub extern "C" fn startup_null() -> () {
    ()
}
