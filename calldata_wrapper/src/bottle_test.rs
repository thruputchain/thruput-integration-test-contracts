use crate::Client;
use bottle_test_if::*;
use sdk::*;

#[derive(Copy, Clone, Default)]
pub struct BT;

#[contract(storage = "false")]
impl BottleTest for BT {
    fn run(&mut self, target_contract_hash: &SHA256Hash) {
        let nonce = SHA256Hash::from([7; 32]);
        let deployed_addr = deploy_contract(&nonce, target_contract_hash).unwrap();

        let mut client = Client::new(&deployed_addr);

        let data = vec![1, 2, 3, 4, 5];
        let echo = client.echo(&data).unwrap();

        assert_eq!(echo, data);
    }
}
