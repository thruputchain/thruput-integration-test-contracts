use crate::implementation;
use crate::interface::*;
use sdk::*;
use sdk::transaction::InvocationFrameStatusCode;

#[test]
fn test_read_only_bytestring_set() {
    let mut env = sdk::TestEnvironment::new(None);

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
    assert_eq!(res.output.status_code(), InvocationFrameStatusCode::kInvocationFrameStatusCodeReturn as i32);
}

fn test_read_only_bytestring_set_inner(
    contract_hash: &sdk::types::SHA256Hash,
) -> Result<()> {
    use crate::implementation::guest_exports::READ_ONLY_BYTESTRING_SET_EXPORT_NUM;

    let nonce = SHA256Hash::from([7; 32]);
    let deployed_addr = deploy_contract(&nonce, contract_hash)?;

    let mut return_len: core::mem::MaybeUninit<u32> = core::mem::MaybeUninit::uninit();

    let flag: i32 =
        groundhog_xdr::transaction::InvokeFlag::kInvokeFlagReadOnly as i32;

    let entrypoint_id =
        u64::from_be_bytes(READ_ONLY_BYTESTRING_SET_EXPORT_NUM.into_inner());

    sdk::sys::uninit_Invoke(
        &deployed_addr,
        entrypoint_id,
        u64::MAX,
        &[0, 0, 0, 1],
        flag,
        &mut return_len,
    )
    .unwrap();

    Ok(())
}
