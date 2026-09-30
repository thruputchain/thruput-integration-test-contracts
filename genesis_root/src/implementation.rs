use super::interface::*;
use alloc::vec::Vec;
use groundhog_erc20_if::ERC20Client;
use sdk::*;

pub struct GenesisRootContract;

const GENESIS_ROOT_ADDRESS: ContractAddress = ContractAddress::new([
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 2,
]);

#[contract]
impl GenesisRoot for GenesisRootContract {
    fn distribute(
        distributions: Vec<Distribution>,
        root_funding: i128,
        token_funding: i128,
    ) {
        let mut block_height: BlockHeight = 0.into();
        sdk::sys::raw_GetBlockHeight(&mut block_height).unwrap();
        assert!(block_height.into_inner() <= 1);

        let token_address = sdk::precompiles::ADDRESS_PRECOMPILED_HOG_TOKEN;
        let token = ERC20Client::new(&token_address);
        token.mint(&token_address, token_funding);
        token.mint(&GENESIS_ROOT_ADDRESS, root_funding);

        for distribution in distributions {
            let deployed = sdk::get_deploy_address(
                &distribution.nonce,
                &distribution.contract_hash,
            );
            token.mint(&deployed, distribution.allocation);
            let actual = sdk::deploy_contract(
                &distribution.nonce,
                &distribution.contract_hash,
            );
            assert_eq!(actual, deployed);

            let auth_handles = AuthHandleBuf::new();
            let mut return_buf = sdk::call::ReturnBuf::new();
            let mut returned_auth_handles = AuthHandleBuf::new();
            match sdk::call::contract_call(
                &deployed,
                distribution.initializer,
                u64::MAX,
                distribution.initializer_calldata.as_slice(),
                &auth_handles,
                0,
                &mut return_buf,
                &mut returned_auth_handles,
            ) {
                Ok(sdk::call::CallOutcome::Return) => {}
                Ok(sdk::call::CallOutcome::Abort) => {
                    sdk::contract_abort(return_buf.as_ref())
                }
                Err(_) => trap(),
            }
        }
    }
}
