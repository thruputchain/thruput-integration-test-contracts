use thruput_sdk::*;

#[contract(client = "CallerClient")]
pub trait FibonacciCaller {
    fn initialize(&mut self, address: &ContractAddress);
    fn call_fibonacci(&mut self, n: u32, gas_limit: u64);
    fn call_fibonacci_expect_out_of_gas(&mut self, n: u32, gas_limit: u64);
    fn call_burn_gas(&mut self, gas_limit: u64);
    fn call_burn_gas_expect_out_of_gas(&mut self, gas_limit: u64);
}
