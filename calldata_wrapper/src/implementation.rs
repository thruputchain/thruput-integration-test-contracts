//! An implementation of Fibonacci

use super::interface::CalldataWrapper;
use sdk::prelude::*;

use alloc::vec::Vec;

struct CalldataWrapperContract;

// This implements the Fibonacci interface.  The `contract` macro provides
// an exported entrypoint for each method; these will be placed in a
// module with the specified name ("guest_exports").
#[contract(exports = "guest_exports", storage = "false")]
impl CalldataWrapper for CalldataWrapperContract {
    fn echo(&self, data: &Vec<u8>) -> Vec<u8> {
        data.clone()
    }
}

#[unsafe(export_name = "FFFFFFFFFFFFFFFF")]
pub extern "C" fn startup_null() -> () {
    ()
}

