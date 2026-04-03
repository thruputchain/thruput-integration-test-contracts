use crate::{ReadOnlyTestContract, ReadOnlyTest};
use sdk::*;

#[test]
fn test_read_only_bytestring_set() {
    let env = sdk::TestEnvironment::no_cost();

    let test_contract = env.register_mock(<ReadOnlyTestContract as ReadOnlyTest>::__mock);
    let contract_hash = test_contract.as_hash().clone();

    contract_exec(env, move ||
        test_read_only_bytestring_set_inner(&contract_hash)
    ).unwrap();
}

fn test_read_only_bytestring_set_inner(
    contract_hash: &sdk::types::SHA256Hash,
) {
    let nonce = SHA256Hash::from([7; 32]);
    let deployed_addr = deploy_contract(&nonce, contract_hash);

    let mut return_len: core::mem::MaybeUninit<u32> = core::mem::MaybeUninit::uninit();

    let flag: i32 =
        groundhog_xdr::transaction::InvokeFlag::ReadOnly.0;
    let calldata = cxs::encode(&true).unwrap();

    let entrypoint_id = u64::from_be_bytes(
        ::sdk::contract::ContractEntrypointID::from(
            crate::ReadOnlyTestEntrypoints::read_only_bytestring_set,
        )
        .into_inner(),
    );

    sdk::sys::uninit_Invoke(
        &deployed_addr,
        entrypoint_id,
        u64::MAX,
        &calldata,
        flag,
        &mut return_len,
    )
    .unwrap();
}
