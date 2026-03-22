use crate::implementation;
use crate::Client;
use sdk::*;
use sdk::transaction::InvocationFrameStatusCode;
use std::vec;

#[test]
fn test_calldata_wrapper() {
    let mut env = sdk::TestEnvironment::new(None);

    let test_contract = implementation::test_setup::register(&mut env);
    let contract_hash = env.setup_create_contract(&test_contract);

    // sdk::TestEnvironment::enable_syscall_logging();

    let mut handler = env.prepare_test(move || -> () {
        test_calldata_wrapper_inner(&contract_hash)
    });

    let res = handler.root_invoke();
    assert_eq!(res.output.status_code(), InvocationFrameStatusCode::Return);
}

fn test_calldata_wrapper_inner(
    contract_hash: &sdk::types::SHA256Hash,
) {
    let nonce = SHA256Hash::from([7; 32]);
    let deployed_addr = deploy_contract(&nonce, contract_hash);

    let mut client = Client::new(&deployed_addr);

    let data = vec![1, 2, 3, 4, 5];
    let echo = client.echo(&data);

    assert_eq!(echo, data);
}
