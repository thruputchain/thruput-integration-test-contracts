use sdk::*;

#[contract(client = Client)]
pub trait DeployAndTransfer {
    fn deploy_and_transfer(
        &self,
        nonce: &SHA256Hash,
        contract_hash: &SHA256Hash,
        amount: u128,
    ) -> ContractAddress;
}
