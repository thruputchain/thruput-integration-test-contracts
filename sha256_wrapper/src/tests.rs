use crate::implementation;
use crate::interface::client::Sha256WrapperClient;
use sdk::prelude::*;
use sha2::{Digest, Sha256};

use std::vec;

#[test]
fn test_sha256_wrapper() {
    let mut env = sdk::TestEnvironment::new();

    let test_contract = implementation::test_setup::register(&mut env);
    let contract_hash = env.setup_create_contract(&test_contract);

    sdk::TestEnvironment::enable_syscall_logging();

    let mut handler = env.prepare_test(move || -> () {
        match test_sha256_wrapper_inner(&contract_hash) {
            Ok(_) => (),
            Err(error) => panic!("test failed: {}", error),
        }
    });

    let res = handler.root_invoke();
    assert_eq!(res.result.panic, sdk::InvokeErrorCode::kInvokeNone as i32);
}

fn test_sha256_wrapper_inner(
    contract_hash: &sdk::types::SHA256Hash,
) -> Result<()> {
    let nonce = SHA256Hash::from([7; 32]);
    let deployed_addr = deploy_contract(&nonce, contract_hash)?;

    let mut client = Sha256WrapperClient::new(&deployed_addr);

    let data = vec![1, 2, 3, 4, 5];
    let hash = client.sha256(&data);

    // Compute expected hash using sha2 crate
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let expect_bytes = hasher.finalize();
    let expect =
        SHA256Hash::from(<[u8; 32]>::try_from(&expect_bytes[..]).unwrap());
    assert_eq!(hash, expect);

    Ok(())
}
