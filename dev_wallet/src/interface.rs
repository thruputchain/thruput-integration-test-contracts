use sdk::*;

#[contract(client = Client)]
pub trait DevWallet {
    fn initialize(&mut self, owner: &Ed25519PublicKey);
    fn do_action(invocation_bytes: &[u8]);
    fn deploy(
        nonce: &SHA256Hash,
        contract_hash: &SHA256Hash,
    ) -> ContractAddress;
    /// Deploy and initialize a wallet at its deterministic address.
    ///
    /// The caller must transfer enough HOG to the address derived from
    /// `(msg_sender, nonce, wallet_contract_hash)` before invoking this
    /// entrypoint. Wallet creation does not transfer HOG itself.
    fn create_wallet(
        nonce: &SHA256Hash,
        wallet_contract_hash: &SHA256Hash,
        owner: &Ed25519PublicKey,
    ) -> ContractAddress;
}
