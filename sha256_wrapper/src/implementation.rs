//! An implementation of Fibonacci

use super::interface::Sha256Wrapper;
use sdk::*;

#[derive(Default)]
struct Sha256WrapperContract;

// This implements the Fibonacci interface.  The `contract` macro provides
// an exported entrypoint for each method; these will be placed in a
// module with the specified name ("guest_exports").
#[contract(exports = "guest_exports", storage = "false")]
impl Sha256Wrapper for Sha256WrapperContract {
    fn sha256(&self, data: &[u8]) -> SHA256Hash {
        sdk::crypto::sha_256(data).unwrap()
    }

    fn sha256_repeated(&self, n: u32, data: &[u8]) {
        for _ in 0..n {
            self.sha256(data);
        }
    }
}

#[unsafe(export_name = "FFFFFFFFFFFFFFFF")]
pub extern "C" fn startup_null() -> () {
    ()
}
