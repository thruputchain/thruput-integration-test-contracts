//! An implementation of Fibonacci

use crate::Sha256Wrapper;
use thruput_sdk::*;

pub struct Sha256WrapperContract;

// This implements the Fibonacci interface.  The `contract` macro provides
// an exported entrypoint for each method; these will be placed in a
// module with the specified name ("guest_exports").
#[contract]
impl Sha256Wrapper for Sha256WrapperContract {
    fn sha256(data: &[u8]) -> SHA256Hash {
        thruput_sdk::crypto::sha256(data)
    }

    fn sha256_repeated(n: u32, data: &[u8]) {
        for _ in 0..n {
            Self::sha256(data);
        }
    }
}

