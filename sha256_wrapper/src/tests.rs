use crate::{Sha256WrapperContract, Client};
use sdk::*;
use sha2::{Digest, Sha256};

use std::vec;

client_test! { Sha256WrapperContract =>

    fn test_sha256_wrapper(client: &mut Client) {
        let data = vec![1, 2, 3, 4, 5];
        let hash = client.sha256(&data);

        // Compute expected hash using sha2 crate
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let expect_bytes = hasher.finalize();
        let expect =
            SHA256Hash::from(<[u8; 32]>::try_from(&expect_bytes[..]).unwrap());
        assert_eq!(hash, expect);
    }
}
