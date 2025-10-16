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
        test_fibonacci_inner(&mut client).unwrap_or_else(|e|
            panic!("fibonacci failure: {}", e));
    }
}

fn test_fibonacci_inner(client: &mut Client) -> InvokeResult<()> {
    assert_eq!(client.fibonacci(0)?, 0);
    assert_eq!(client.fibonacci(1)?, 1);
    assert_eq!(client.fibonacci(2)?, 1);
    assert_eq!(client.fibonacci(3)?, 2);
    assert_eq!(client.fibonacci(4)?, 3);
    assert_eq!(client.fibonacci(5)?, 5);
    assert_eq!(client.fibonacci(6)?, 8);

    assert_eq!(client.slow_fibonacci(0)?, 0);
    assert_eq!(client.slow_fibonacci(1)?, 1);
    assert_eq!(client.slow_fibonacci(2)?, 1);
    assert_eq!(client.slow_fibonacci(3)?, 2);
    assert_eq!(client.slow_fibonacci(4)?, 3);
    assert_eq!(client.slow_fibonacci(5)?, 5);
    assert_eq!(client.slow_fibonacci(6)?, 8);

    assert_eq!(client.fibonacci(0)?, 0);
    assert_eq!(client.fibonacci(1)?, 1);
    assert_eq!(client.fibonacci(2)?, 1);
    assert_eq!(client.fibonacci(3)?, 2);
    assert_eq!(client.fibonacci(4)?, 3);
    assert_eq!(client.fibonacci(5)?, 5);
    assert_eq!(client.fibonacci(6)?, 8);

    assert_eq!(client.slow_fibonacci(0)?, 0);
    assert_eq!(client.slow_fibonacci(1)?, 1);
    assert_eq!(client.slow_fibonacci(2)?, 1);
    assert_eq!(client.slow_fibonacci(3)?, 2);
    assert_eq!(client.slow_fibonacci(4)?, 3);
    assert_eq!(client.slow_fibonacci(5)?, 5);
    assert_eq!(client.slow_fibonacci(6)?, 8);

    assert_eq!(client.fibonacci(10)?, client.slow_fibonacci(10)?);
    assert_eq!(client.fibonacci(40)?, client.slow_fibonacci(40)?);

    Ok(())
}
