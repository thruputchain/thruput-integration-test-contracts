use super::interface::*;
use thruput_erc20_if::ERC20Client;
use thruput_sdk::*;

pub struct DeployAndTransferContract;

#[contract]
impl DeployAndTransfer for DeployAndTransferContract {
    fn deploy_and_transfer(
        nonce: &SHA256Hash,
        contract_hash: &SHA256Hash,
        amount: i128,
    ) -> ContractAddress {
        let deployed_address = thruput_sdk::get_deploy_address(nonce, contract_hash);
        let token_client =
            ERC20Client::new(&thruput_sdk::precompiles::ADDRESS_PRECOMPILED_HOG_TOKEN);
        token_client.transfer(&deployed_address, amount);
        thruput_sdk::deploy_contract(nonce, contract_hash)
    }
}
