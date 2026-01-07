use sdk::*;

#[contract(client = Client)]
pub trait Fibonacci {
    fn fibonacci(&self, n: u32) -> u32;
    fn slow_fibonacci(&self, n: u32) -> u32;
    fn change_nonce_config(&self, required_weight: u32);
}
