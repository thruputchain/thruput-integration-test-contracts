#![cfg_attr(not(feature = "dev"), no_std)]

thruput_sdk::declare_mock_contract!();

use thruput_sdk::storage::NNInt;
use thruput_sdk::*;

#[contract_exception]
pub enum Error {
    NotAuthorized = 1,
}

#[derive(Debug)]
#[thruput_sdk::storage]
pub struct Counter {
    value: NNInt,
}

#[contract(throws = Error)]
impl Counter {
    pub fn initialize(&mut self, initial_value: u64) {
        self.value.strict_inc(initial_value as i128);
    }

    pub fn increment(&mut self, amount: u64) {
        self.value.strict_inc(amount as i128);
    }

    pub fn decrement(&mut self, amount: u64) {
        self.value.strict_inc(-(amount as i128));
    }

    pub fn get(&self) -> u64 {
        self.value.try_get().none_if_missing().unwrap_or(0) as u64
    }
}
