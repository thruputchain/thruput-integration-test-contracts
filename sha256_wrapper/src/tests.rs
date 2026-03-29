use crate::{Client, Sha256WrapperContract, Sha256Wrapper};
use crate::interface::ClientError;
use sha2::{Digest, Sha256};

use std::vec;

#[test]
fn test_sha256_wrapper() {
    sdk::client_test(
        |env| env.register_mock(<Sha256WrapperContract as Sha256Wrapper>::__mock),
        client_test,
    )
}

fn client_test(client: &mut Client) {
    let data = vec![1, 2, 3, 4, 5];
    let hash = client.sha256(&data);

    // Compute expected hash using sha2 crate
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let expect_bytes = hasher.finalize();
    let expect =
        sdk::SHA256Hash::from(<[u8; 32]>::try_from(&expect_bytes[..]).unwrap());
    assert_eq!(hash, expect);
}

#[test]
fn test_gas_limit_does_not_propagate() {
    sdk::client_test(
        |env| env.register_mock(<Sha256WrapperContract as Sha256Wrapper>::__mock),
        gas_limit_does_not_propagate,
    )
}

fn gas_limit_does_not_propagate(client: &mut Client) {
    let data = vec![1, 2, 3, 4, 5];

    // Call with tiny gas limit — should fail with OOG
    let result = client.with_gas_limit(1).try_sha256(&data);
    assert!(
        matches!(result, Err(ClientError::Fault(_))),
        "Expected OOG fault with gas_limit=1, got: {:?}",
        result
    );

    // Subsequent call without explicit gas limit should use default (unlimited).
    // This is the key assertion: with_gas_limit must NOT have mutated `client`.
    let hash = client
        .try_sha256(&data)
        .expect("gas limit should not propagate to subsequent calls");

    let mut hasher = Sha256::new();
    hasher.update(&data);
    let expect_bytes = hasher.finalize();
    let expect =
        sdk::SHA256Hash::from(<[u8; 32]>::try_from(&expect_bytes[..]).unwrap());
    assert_eq!(hash, expect);
}
