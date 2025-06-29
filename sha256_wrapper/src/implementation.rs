//! An implementation of Fibonacci

use super::interface::Sha256Wrapper;
use sdk::prelude::*;

use alloc::vec::Vec;

struct Sha256WrapperContract;


// This implements the Fibonacci interface.  The `contract` macro provides
// an exported entrypoint for each method; these will be placed in a
// module with the specified name ("guest_exports").
#[contract(exports = "guest_exports", storage = "false")]
impl Sha256Wrapper for Sha256WrapperContract {
    fn sha256(&self, data: &Vec<u8>) -> SHA256Hash {
        sdk::crypto::sha_256(data).unwrap()
    }
}