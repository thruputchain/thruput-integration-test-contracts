use super::interface::*;
use groundhog_erc20_if::ERC20Client;
use sdk::*;

pub struct DeployAndTransferContract;

#[contract]
impl DeployAndTransfer for DeployAndTransferContract {
    fn deploy_and_transfer(
        nonce: &SHA256Hash,
        contract_hash: &SHA256Hash,
        amount: i128,
    ) -> ContractAddress {
        let deployed_address = sdk::get_deploy_address(nonce, contract_hash);
        let token_client =
            ERC20Client::new(&sdk::precompiles::ADDRESS_PRECOMPILED_HOG_TOKEN);
        token_client.transfer(&deployed_address, amount);
        sdk::deploy_contract(nonce, contract_hash)
    }
}
