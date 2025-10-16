use crate::Client;
use bottle_test_if::*;
use sdk::*;
use sha2::{Digest, Sha256};

#[derive(Copy, Clone, Default)]
pub struct BT;

#[contract(storage = "false")]
impl BottleTest for BT {
    fn run(&mut self, target_contract_hash: &SHA256Hash) {
        let nonce = SHA256Hash::from([7; 32]);
        let deployed_addr = deploy_contract(&nonce, target_contract_hash).unwrap();

        let mut client = Client::new(&deployed_addr);

        let data = vec![1, 2, 3, 4, 5];
        let hash = client.sha256(&data).unwrap();

        // Compute expected hash using sha2 crate
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let expect_bytes = hasher.finalize();
        let expect =
            SHA256Hash::from(<[u8; 32]>::try_from(&expect_bytes[..]).unwrap());
        assert_eq!(hash, expect);
    }
}
