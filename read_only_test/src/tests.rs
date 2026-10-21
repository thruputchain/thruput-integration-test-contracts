use crate::checks::check_read_only_bytestring_set;
use crate::{ReadOnlyTestContract, ReadOnlyTest};
use thruput_sdk::*;

#[test]
fn test_read_only_bytestring_set() {
    let env = thruput_sdk::TestEnvironment::no_cost();

    let test_contract = env.register_mock(<ReadOnlyTestContract as ReadOnlyTest>::__mock);
    let contract_hash = test_contract.as_hash().clone();

    env.transact(|tx| {
        tx.call(&ContractAddress::named("read_only")).run(move || {
            check_read_only_bytestring_set(&contract_hash)
        })
    })
    .assert_success();
}
