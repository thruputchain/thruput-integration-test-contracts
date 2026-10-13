use thruput_sdk::*;

#[contract]
pub trait SimpleWalletAuthClaim {
    fn initialize(owner: &Ed25519PublicKey);
    fn do_action(invocation_bytes: &[u8]);
}
