use crate::{Client, Sha256WrapperContract, Sha256WrapperTestSetup};
use sha2::{Digest, Sha256};

use std::vec;

#[test]
fn test_sha256_wrapper() {
    sdk::client_test(
        |env| env.register_mock::<Sha256WrapperContract, Sha256WrapperTestSetup>(),
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
