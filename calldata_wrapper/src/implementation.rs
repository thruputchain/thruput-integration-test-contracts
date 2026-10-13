//! An implementation of Fibonacci

use super::interface::CalldataWrapper;
use thruput_sdk::*;

use alloc::vec::Vec;

pub struct CalldataWrapperContract;

// This implements the Fibonacci interface.  The `contract` macro provides
// an exported entrypoint for each method; these will be placed in a
// module with the specified name ("guest_exports").
#[contract]
impl CalldataWrapper for CalldataWrapperContract {
    fn echo(data: &[u8]) -> Vec<u8> {
        data.into()
    }
}

