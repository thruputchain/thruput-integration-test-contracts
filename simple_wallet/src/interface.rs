use sdk::prelude::*;

pub use sdk::prelude::{Error, Result};

#[contract(client = SimpleWalletClient)]
pub trait SimpleWallet {
    fn initialize(&mut self, owner: &Ed25519PublicKey) -> Result<()>;
    fn do_action(&self, signature: &Ed25519Signature, invocation_bytes: &[u8]) -> Result<()>;
}
