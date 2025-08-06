use sdk::prelude::*;

pub use sdk::contract_result::*;

#[contract(client = "Client")]
pub trait FibonacciCaller {
    fn initialize(&mut self, address: &ContractAddress);
    fn call_fibonacci(&mut self, n: u32, gas_limit: u64);
    fn call_fibonacci_expect_out_of_gas(&mut self, n: u32, gas_limit: u64);
}
