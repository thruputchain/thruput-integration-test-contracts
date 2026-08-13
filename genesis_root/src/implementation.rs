use super::interface::*;
use groundhog_erc20_if::ERC20Client;
use sdk::*;

pub struct GenesisRootContract;

const GENESIS_ROOT_ADDRESS: ContractAddress = ContractAddress::new([
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
]);

#[contract]
impl GenesisRoot for GenesisRootContract {
    fn distribute(
        nonce: &SHA256Hash,
        contract_hash: &SHA256Hash,
        initializer: &ContractEntrypointID,
        initializer_calldata: &[u8],
        root_funding: u128,
        allocation: u128,
    ) -> ContractAddress {
        let token = ERC20Client::new(&sdk::precompiles::ADDRESS_PRECOMPILED_HOG_TOKEN);
        token.mint(&GENESIS_ROOT_ADDRESS, root_funding);

        let deployed = sdk::get_deploy_address(nonce, contract_hash);
        token.mint(&deployed, allocation);
        let actual = sdk::deploy_contract(nonce, contract_hash);
        assert_eq!(actual, deployed);

        let auth_handles = AuthHandleBuf::new();
        let mut return_buf = sdk::call::ReturnBuf::new();
        let mut returned_auth_handles = AuthHandleBuf::new();
        sdk::call::contract_call(
            &deployed,
            *initializer,
            u64::MAX,
            initializer_calldata,
            &auth_handles,
            0,
            &mut return_buf,
            &mut returned_auth_handles,
        )
        .unwrap();
        deployed
    }
}
