use sdk::*;

#[contract(client = Client)]
pub trait DevWallet {
    fn initialize(&mut self, owner: &Ed25519PublicKey);
    fn do_action(&self, invocation_bytes: &[u8]);
    fn deploy(
        &self,
        nonce: &SHA256Hash,
        contract_hash: &SHA256Hash,
    ) -> ContractAddress;
}
