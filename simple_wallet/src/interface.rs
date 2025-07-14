use sdk::prelude::*;

pub use sdk::prelude::{Error, Result};

#[contract(client = "client")]
pub trait SimpleWallet {
    fn initialize(maybe_self: Option<&mut Self>, owner: Ed25519PublicKey) -> Result<()>;
    fn do_action(&self, signature: &Ed25519Signature, invocation_bytes: &[u8]) -> Result<()>;
}
