use thruput_sdk::*;

#[contract(client = Client)]
pub trait DeployAndTransfer {
    fn deploy_and_transfer(
        nonce: &SHA256Hash,
        contract_hash: &SHA256Hash,
        amount: i128,
    ) -> ContractAddress;
}
