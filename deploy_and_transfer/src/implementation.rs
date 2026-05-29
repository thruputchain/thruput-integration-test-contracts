use super::interface::*;
use groundhog_erc20_if::ERC20Client;
use sdk::*;

#[derive(Default)]
pub struct DeployAndTransferContract;

#[contract(storage = false)]
impl DeployAndTransfer for DeployAndTransferContract {
    fn deploy_and_transfer(
        &self,
        nonce: &SHA256Hash,
        contract_hash: &SHA256Hash,
        amount: u128,
    ) -> ContractAddress {
        let deployed_address = sdk::get_deploy_address(nonce, contract_hash);
        let token_client =
            ERC20Client::new(&sdk::precompiles::ADDRESS_PRECOMPILED_HOG_TOKEN);
        token_client.transfer(&deployed_address, amount);
        sdk::deploy_contract(nonce, contract_hash)
    }
}
