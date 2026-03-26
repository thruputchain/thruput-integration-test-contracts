use sdk::*;

#[contract]
pub trait SimpleWalletAuthClaim {
    fn initialize(&mut self, owner: &Ed25519PublicKey);
    fn do_action(&self, invocation_bytes: &[u8]);
}
