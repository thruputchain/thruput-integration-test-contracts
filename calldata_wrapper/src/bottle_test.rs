use crate::Client;
use alloc::vec;
use bottle_test_if::*;
use sdk::*;
use sdk::storage::Value;

#[derive(Debug)]
#[storage]
pub struct BT {
    target_addr: Value<ContractAddress>,
}

#[contract(version = "0.1.0")]
impl BottleTest for BT {
    fn setup(&mut self, target_contract_hash: &SHA256Hash) {
        let nonce: SHA256Hash = [7; 32].into();
        let target_addr = deploy_contract(&nonce, target_contract_hash);

        self.target_addr.set(&target_addr);
    }

    fn run(&mut self) {
        let target_addr = self.target_addr.try_get().unwrap_or_else(||
            panic!("bottle-test: target_addr unset; please run setup() before run()"));

        let client = Client::new(&target_addr);

        let data = vec![1, 2, 3, 4, 5];
        let echo = client.echo(&data);

        assert_eq!(echo, data);
    }
}
