//! An implementation of Fibonacci

use crate::FibonacciCaller;
use sdk::*;

#[storage]
pub struct FibonacciContractCaller {
    address: sdk::storage::Value<ContractAddress>,
}

#[contract]
impl FibonacciCaller for FibonacciContractCaller {
    #[init]
    fn initialize(&mut self, address: &ContractAddress) {
        self.address.set(address);
    }

    fn call_fibonacci(&mut self, n: u32, gas_limit: u64) {
        let client = fibonacci::Client::new(&self.address.get());
        let _ = client.with_gas_limit(gas_limit).slow_fibonacci(n);
    }

    fn call_fibonacci_expect_out_of_gas(&mut self, n: u32, gas_limit: u64) {
        let client = fibonacci::Client::new(&self.address.get());
        let result = client
            .with_gas_limit(gas_limit)
            .try_slow_fibonacci(n);
        match result {
            Err(fibonacci::ClientError::Fault(_)) => {} // expected out-of-gas fault
            other => panic!("Expected Fault from out-of-gas, got: {:?}", other),
        }
    }

    fn call_burn_gas(&mut self, gas_limit: u64) {
        let client = fibonacci::Client::new(&self.address.get());
        client.with_gas_limit(gas_limit).burn_gas();
    }

    fn call_burn_gas_expect_out_of_gas(&mut self, gas_limit: u64) {
        let client = fibonacci::Client::new(&self.address.get());
        let result = client
            .with_gas_limit(gas_limit)
            .try_burn_gas();
        match result {
            Err(fibonacci::ClientError::Fault(_)) => {} // expected out-of-gas fault
            other => panic!("Expected Fault from out-of-gas, got: {:?}", other),
        }
    }
}
