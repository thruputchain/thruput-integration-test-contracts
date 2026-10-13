use super::interface::*;
use alloc::vec::Vec;
use thruput_erc20_if::ERC20Client;
use thruput_sdk::*;

pub struct GenesisRootContract;

const GENESIS_ROOT_ADDRESS: ContractAddress = ContractAddress::new([
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 2,
]);

#[contract]
impl GenesisRoot for GenesisRootContract {
    fn distribute(
        distributions: Vec<Distribution>,
        account_allocations: Vec<AccountAllocation>,
        root_funding: i128,
        token_funding: i128,
    ) {
        let mut block_height: BlockHeight = 0.into();
        thruput_sdk::sys::raw_GetBlockHeight(&mut block_height).unwrap();
        assert!(block_height.into_inner() <= 1);

        let token_address = thruput_sdk::precompiles::ADDRESS_PRECOMPILED_HOG_TOKEN;
        let token = ERC20Client::new(&token_address);
        token.mint(&token_address, token_funding);
        token.mint(&GENESIS_ROOT_ADDRESS, root_funding);

        for allocation in account_allocations {
            token.mint(&allocation.address, allocation.allocation);
        }

        for distribution in distributions {
            let deployed = thruput_sdk::get_deploy_address(
                &distribution.nonce,
                &distribution.contract_hash,
            );
            token.mint(&deployed, distribution.allocation);
            let actual = thruput_sdk::deploy_contract(
                &distribution.nonce,
                &distribution.contract_hash,
            );
            assert_eq!(actual, deployed);

            let auth_handles = AuthHandleBuf::new();
            let mut return_buf = thruput_sdk::call::ReturnBuf::new();
            let mut returned_auth_handles = AuthHandleBuf::new();
            match thruput_sdk::call::contract_call(
                &deployed,
                distribution.initializer,
                u64::MAX,
                distribution.initializer_calldata.as_slice(),
                &auth_handles,
                0,
                &mut return_buf,
                &mut returned_auth_handles,
            ) {
                Ok(thruput_sdk::call::CallOutcome::Return) => {}
                Ok(thruput_sdk::call::CallOutcome::Abort) => {
                    thruput_sdk::contract_abort(return_buf.as_ref())
                }
                Err(_) => trap(),
            }
        }
    }
}
