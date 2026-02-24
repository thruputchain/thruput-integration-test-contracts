use sdk::*;

#[contract(client = Client)]
pub trait SimpleWallet {
    fn initialize(&mut self, owner: &Ed25519PublicKey);
    fn do_action(&self, signature: &Ed25519Signature, invocation_bytes: &[u8]);
}
