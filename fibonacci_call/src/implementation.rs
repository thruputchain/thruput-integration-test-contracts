//! An implementation of Fibonacci

use super::interface::*;
use sdk::*;
use sdk::serialize::*;


#[derive(Default, Debug, Serialize, Deserialize)]
pub struct FibonacciContractCaller {
    address: ContractAddress,
}

impl FibonacciContractCaller {
    pub fn new(address: ContractAddress) -> Self {
        Self { address }
    }
}

// This implements the Fibonacci interface.  The `contract` macro provides
// an exported entrypoint for each method; these will be placed in a
// module with the specified name ("guest_exports").
#[contract(exports = "guest_exports")]
impl FibonacciCaller for FibonacciContractCaller {
    fn initialize(&mut self, address: &ContractAddress) {
        self.address = *address;
    }

    fn call_fibonacci(&mut self, n: u32, gas_limit: u64) {
        let mut client = fibonacci::Client::new(&self.address);
        client.with_gas_limit(gas_limit).slow_fibonacci(n).unwrap();
    }

    fn call_fibonacci_expect_out_of_gas(&mut self, n: u32, gas_limit: u64) {
        let mut client = fibonacci::Client::new(&self.address);
        client.with_gas_limit(gas_limit).slow_fibonacci(n).unwrap_err();
    }
}
