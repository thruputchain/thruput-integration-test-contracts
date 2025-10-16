#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "mock")]
extern crate std;

extern crate alloc;

// A trait describing the contract interface
pub mod interface;

// A client stub that provides the interface via cross-contract calls
#[cfg(feature = "client")]
pub use interface::Client;

// An implementation of the contract interface
#[cfg(feature = "guest")]
pub mod implementation;

// Contract methods exposed with C ABI for running as guest
#[cfg(feature = "guest")]
pub use implementation::erc1271_exports::*;
#[cfg(feature = "guest")]
pub use implementation::guest_exports::*;

#[cfg(feature = "mock")]
pub fn register(
    env: &std::rc::Rc<::sdk::TestEnvironment>,
) -> ::sdk::types::Contract {
    let mut contract = env.add_contract();
    implementation::erc1271_test_setup::register_mock(&mut contract);
    implementation::simple_wallet_test_setup::register_mock(&mut contract);
    contract.as_contract()
}
