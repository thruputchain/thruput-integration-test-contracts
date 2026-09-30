use crate::Client;
use bottle_test_if::*;
use sdk::*;
use sdk::storage::Value;

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
        test_fibonacci_inner(&client);
    }
}

fn test_fibonacci_inner(client: &Client) {
    assert_eq!(client.fibonacci(0), 0);
    assert_eq!(client.fibonacci(1), 1);
    assert_eq!(client.fibonacci(2), 1);
    assert_eq!(client.fibonacci(3), 2);
    assert_eq!(client.fibonacci(4), 3);
    assert_eq!(client.fibonacci(5), 5);
    assert_eq!(client.fibonacci(6), 8);

    assert_eq!(client.slow_fibonacci(0), 0);
    assert_eq!(client.slow_fibonacci(1), 1);
    assert_eq!(client.slow_fibonacci(2), 1);
    assert_eq!(client.slow_fibonacci(3), 2);
    assert_eq!(client.slow_fibonacci(4), 3);
    assert_eq!(client.slow_fibonacci(5), 5);
    assert_eq!(client.slow_fibonacci(6), 8);

    assert_eq!(client.fibonacci(0), 0);
    assert_eq!(client.fibonacci(1), 1);
    assert_eq!(client.fibonacci(2), 1);
    assert_eq!(client.fibonacci(3), 2);
    assert_eq!(client.fibonacci(4), 3);
    assert_eq!(client.fibonacci(5), 5);
    assert_eq!(client.fibonacci(6), 8);

    assert_eq!(client.slow_fibonacci(0), 0);
    assert_eq!(client.slow_fibonacci(1), 1);
    assert_eq!(client.slow_fibonacci(2), 1);
    assert_eq!(client.slow_fibonacci(3), 2);
    assert_eq!(client.slow_fibonacci(4), 3);
    assert_eq!(client.slow_fibonacci(5), 5);
    assert_eq!(client.slow_fibonacci(6), 8);

    assert_eq!(client.fibonacci(10), client.slow_fibonacci(10));
    assert_eq!(client.fibonacci(40), client.slow_fibonacci(40));
}
