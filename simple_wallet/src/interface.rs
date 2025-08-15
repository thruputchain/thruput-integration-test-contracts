use sdk::prelude::*;

pub use sdk::contract_result::*;

#[contract(client = Client)]
pub trait SimpleWallet {
    fn initialize(&mut self, owner: &Ed25519PublicKey);
    fn do_action(&self, signature: &Ed25519Signature, invocation_bytes: &[u8]);
}
