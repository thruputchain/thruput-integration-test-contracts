//! An implementation of Ed25519VerifyWrapper

use crate::interface::Ed25519VerifyWrapper;
use thruput_sdk::*;

#[derive(Default)]
pub struct Ed25519VerifyWrapperContract;

// This implements the Fibonacci interface.  The `contract` macro provides
// an exported entrypoint for each method; these will be placed in a
// module with the specified name ("guest_exports").
#[contract]
impl Ed25519VerifyWrapper for Ed25519VerifyWrapperContract {
    fn ed25519_verify(
        message: &[u8],
        signature: &Ed25519Signature,
        public_key: &Ed25519PublicKey,
    ) -> bool {
        thruput_sdk::crypto::verify_signature_ed25519(public_key, signature, message)
    }

    fn ed25519_verify_repeated(
        n: u32,
        message: &[u8],
        signature: &Ed25519Signature,
        public_key: &Ed25519PublicKey,
    ) {
        for _ in 0..n {
            assert!(Self::ed25519_verify(message, signature, public_key));
        }
    }
}
