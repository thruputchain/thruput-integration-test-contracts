use sdk::*;

#[contract(client = Client)]
pub trait DevWallet {
    fn initialize(owner: &Ed25519PublicKey);
    fn do_action(invocation_bytes: &[u8]);
    fn deploy(
        nonce: &SHA256Hash,
        contract_hash: &SHA256Hash,
    ) -> ContractAddress;
}
