use crate::implementation;
use crate::Client;
use sdk::*;
use sha2::{Digest, Sha256};

use std::vec;

#[test]
fn test_sha256_wrapper() {
    let mut env = sdk::TestEnvironment::new(None);

    let test_contract = implementation::test_setup::register(&mut env);
    let contract_hash = env.setup_create_contract(&test_contract);

    // sdk::TestEnvironment::enable_syscall_logging();

    let mut handler = env.prepare_test(move || -> () {
        let nonce = SHA256Hash::from([7; 32]);
        let deployed_addr = deploy_contract(&nonce, &contract_hash).unwrap();
        let mut client = Client::new(&deployed_addr);
        match test_sha256_wrapper_inner(&mut client) {
            Ok(_) => (),
            Err(error) => panic!("test failed: {}", error),
        }
    });

    let res = handler.root_invoke();
    assert_eq!(res.status, sdk::InvokeErrorCode::kInvokeNone);
}

fn test_sha256_wrapper_inner(client: &mut Client) -> InvokeResult<()> {
    let data = vec![1, 2, 3, 4, 5];
    let hash = client.sha256(&data)?;

    // Compute expected hash using sha2 crate
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let expect_bytes = hasher.finalize();
    let expect =
        SHA256Hash::from(<[u8; 32]>::try_from(&expect_bytes[..]).unwrap());
    assert_eq!(hash, expect);

    Ok(())
}
