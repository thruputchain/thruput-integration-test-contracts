use sdk::*;

#[contract(client = Client)]
pub trait GenesisRoot {
    fn distribute(
        nonce: &SHA256Hash,
        contract_hash: &SHA256Hash,
        initializer: &ContractEntrypointID,
        initializer_calldata: &[u8],
        root_funding: u128,
        allocation: u128,
    ) -> ContractAddress;
}
