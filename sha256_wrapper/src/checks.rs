//! Checks shared by the native unit tests (`src/tests.rs`) and the rv32
//! test (`tests/rv32.rs`), so the mock and the rv32 build are held to the
//! same assertions.

use crate::Client;
use sha2::{Digest, Sha256};

pub fn check_sha256(client: &Client) {
    let data = alloc::vec![1, 2, 3, 4, 5];
    let hash = client.sha256(&data);

    // Compute expected hash using sha2 crate
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let expect_bytes = hasher.finalize();
    let expect =
        thruput_sdk::SHA256Hash::from(<[u8; 32]>::try_from(&expect_bytes[..]).unwrap());
    assert_eq!(hash, expect);
}
