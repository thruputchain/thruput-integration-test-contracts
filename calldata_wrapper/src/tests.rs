use crate::{CalldataWrapperContract, CalldataWrapper};
use crate::Client;
use crate::checks::check_echo;
use thruput_sdk::*;

#[test]
fn test_calldata_wrapper() {
    let env = thruput_sdk::TestEnvironment::new(None);

    let test_contract = env
        .register_mock(<CalldataWrapperContract as CalldataWrapper>::__mock);
    let contract_hash = test_contract.as_hash().clone();

    // thruput_sdk::TestEnvironment::enable_syscall_logging();

    env.transact(|tx| {
        tx.call(&ContractAddress::named("calldata_wrapper")).run(move || {
            test_calldata_wrapper_inner(&contract_hash)
        })
    })
    .assert_success();
}

fn test_calldata_wrapper_inner(
    contract_hash: &thruput_sdk::types::SHA256Hash,
) {
    let nonce = SHA256Hash::from([7; 32]);
    let deployed_addr = deploy_contract(&nonce, contract_hash);

    check_echo(&Client::new(&deployed_addr));
}
