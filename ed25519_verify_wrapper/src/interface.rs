use sdk::prelude::*;

pub use sdk::prelude::{Error, Result};

#[contract(client = "Client")]
pub trait Ed25519VerifyWrapper {
    fn ed25519_verify(
        &self,
        message: &[u8],
        signature: &Ed25519Signature,
        public_key: &Ed25519PublicKey,
    ) -> bool;

    fn ed25519_verify_repeated(
        &self,
        n: u32,
        message: &[u8],
        signature: &Ed25519Signature,
        public_key: &Ed25519PublicKey,
    );
}
