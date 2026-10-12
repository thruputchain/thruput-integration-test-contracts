use crate::{ReadOnlyTestContract, ReadOnlyTest};
use sdk::*;
use sdk::xdr::cxs;

#[test]
fn test_read_only_bytestring_set() {
    let env = sdk::TestEnvironment::no_cost();

    let test_contract = env.register_mock(<ReadOnlyTestContract as ReadOnlyTest>::__mock);
    let contract_hash = test_contract.as_hash().clone();

    env.transact(|tx| {
        tx.call(&ContractAddress::named("read_only")).run(move || {
            test_read_only_bytestring_set_inner(&contract_hash)
        })
    })
    .assert_success();
}

fn test_read_only_bytestring_set_inner(
    contract_hash: &sdk::types::SHA256Hash,
) {
    let nonce = SHA256Hash::from([7; 32]);
    let deployed_addr = deploy_contract(&nonce, contract_hash);

    let mut outputs = core::mem::MaybeUninit::uninit();

    let flag = thruput_xdr::invoke_flag::InvokeFlag::ReadOnly.0.cast_unsigned();
    let calldata = cxs::encode(&true).unwrap();

    let entrypoint_id =
        crate::ReadOnlyTestEntrypoints::read_only_bytestring_set.into_u64();

    sdk::sys::uninit_Invoke(
        &deployed_addr,
        &calldata,
        &[], // auth
        &sdk::sys::CopyableInvokeInputs {
            entrypoint_id,
            gas_limit: u64::MAX, // gas limit
            flags: flag,
            reserved: 0,
        },
        &mut outputs,
    )
    .unwrap();
}
