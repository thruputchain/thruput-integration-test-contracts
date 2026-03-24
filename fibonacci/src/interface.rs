use sdk::*;

#[contract(client = Client)]
pub trait Fibonacci {
    fn fibonacci(n: u32) -> u32;
    fn slow_fibonacci(n: u32) -> u32;
    fn burn_gas();
    fn change_nonce_config(required_weight: u32);
}
