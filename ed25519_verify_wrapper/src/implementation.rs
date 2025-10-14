//! An implementation of Ed25519VerifyWrapper

use super::interface::Ed25519VerifyWrapper;
use sdk::*;

#[derive(Default)]
struct Ed25519VerifyWrapperContract;

// This implements the Fibonacci interface.  The `contract` macro provides
// an exported entrypoint for each method; these will be placed in a
// module with the specified name ("guest_exports").
#[contract(exports = "guest_exports", storage = "false")]
impl Ed25519VerifyWrapper for Ed25519VerifyWrapperContract {
    fn ed25519_verify(
        &self,
        message: &[u8],
        signature: &Ed25519Signature,
        public_key: &Ed25519PublicKey,
    ) -> bool {
        sdk::crypto::verify_signature_ed25519(public_key, signature, &message)
            .is_ok()
    }

    fn ed25519_verify_repeated(
        &self,
        n: u32,
        message: &[u8],
        signature: &Ed25519Signature,
        public_key: &Ed25519PublicKey,
    ) {
        for _ in 0..n {
            assert!(self.ed25519_verify(message, signature, public_key));
        }
    }
}

#[unsafe(export_name = "FFFFFFFFFFFFFFFF")]
pub extern "C" fn startup_null() -> () {
    ()
}
