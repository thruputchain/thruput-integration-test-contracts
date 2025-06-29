use sdk::prelude::*;

pub use sdk::prelude::{Error, Result};

use alloc::vec::Vec;

#[contract(client = "client")]
pub trait Ed25519VerifyWrapper {
    fn ed25519_verify(
        &self,
        message: &Vec<u8>,
        signature: &Ed25519Signature,
        public_key: &Ed25519PublicKey,
    ) -> bool;
}
