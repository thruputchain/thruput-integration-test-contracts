use crate::implementation;
use crate::interface::CalldataWrapperClient;
use sdk::prelude::*;

use std::vec;

#[test]
fn test_calldata_wrapper() {
    let mut env = sdk::TestEnvironment::new();

    let test_contract = implementation::test_setup::register(&mut env);
    let contract_hash = env.setup_create_contract(&test_contract);

    sdk::TestEnvironment::enable_syscall_logging();

    let mut handler = env.prepare_test(move || -> () {
        match test_calldata_wrapper_inner(&contract_hash) {
            Ok(_) => (),
            Err(error) => panic!("test failed: {}", error),
        }
    });

    let res = handler.root_invoke();
    assert_eq!(res.result.panic, sdk::InvokeErrorCode::kInvokeNone as i32);
}

fn test_calldata_wrapper_inner(
    contract_hash: &sdk::types::SHA256Hash,
) -> Result<()> {
    let nonce = SHA256Hash::from([7; 32]); 
    let deployed_addr = deploy_contract(&nonce, contract_hash)?;

    let mut client = CalldataWrapperClient::new(&deployed_addr);

    let data = vec![1, 2, 3, 4, 5];
    let echo = client.echo(&data);

    assert_eq!(echo, data);

    Ok(())
}
