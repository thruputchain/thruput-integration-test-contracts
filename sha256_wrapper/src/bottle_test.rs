use crate::Client;
use alloc::vec;
use bottle_test_if::*;
use thruput_sdk::*;
use thruput_sdk::storage::Value;
use sha2::{Digest, Sha256};

#[derive(Debug)]
#[storage]
pub struct BT {
    target_addr: Value<ContractAddress>,
}

#[contract]
impl BottleTest for BT {

    fn setup(&mut self, target_contract_hash: &SHA256Hash) {
        let nonce: SHA256Hash = [7; 32].into();
        let target_addr = deploy_contract(&nonce, target_contract_hash);

        self.target_addr.set(&target_addr);
    }


    fn run(&mut self) {
        let target_addr = self.target_addr.try_get().none_if_missing().unwrap_or_else(||
            panic!("bottle-test: target_addr unset; please run setup() before run()"));

        let client = Client::new(&target_addr);

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
