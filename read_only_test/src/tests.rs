use crate::implementation;
use crate::interface::*;
use sdk::*;

#[test]
fn test_read_only_bytestring_set() {
    let mut env = sdk::TestEnvironment::new();

    let test_contract = implementation::test_setup::register(&mut env);
    let contract_hash = env.setup_create_contract(&test_contract);

    // sdk::TestEnvironment::enable_syscall_logging();

    let mut handler = env.prepare_test(move || -> () {
        match test_read_only_bytestring_set_inner(&contract_hash) {
            Ok(_) => (),
            Err(error) => panic!("test failed: {}", error),
        }
    });

    let res = handler.root_invoke();
    assert_eq!(res.status, sdk::InvokeErrorCode::kInvokeNone);
}

fn test_read_only_bytestring_set_inner(
    contract_hash: &sdk::types::SHA256Hash,
) -> Result<()> {
    let nonce = SHA256Hash::from([7; 32]);
    let deployed_addr = deploy_contract(&nonce, contract_hash)?;

    let mut return_len = 0;

    let flag: i32 =
        groundhog_xdr::transaction::InvokeFlag::kInvokeFlagReadOnly as i32;

    sdk::sys::uninit_Invoke(
        &deployed_addr,
        0xCE6746E15AF20000,
        u64::MAX,
        &[0, 0, 0, 1],
        flag,
        &mut return_len,
    )
    .unwrap();

    Ok(())
}
