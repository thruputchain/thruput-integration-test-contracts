#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "dev")]
extern crate std;

extern crate alloc;

mod interface;
mod implementation;

// A client stub that provides the interface via cross-contract calls
pub use interface::Client;

// Contract methods exposed with C ABI for running as guest
pub use implementation::exports::*;
pub use implementation::erc1271_exports::*;

#[cfg(feature = "dev")]
pub fn register(
    env: &sdk::TestEnvironmentHandle,
) -> ::sdk::types::Contract {
    let mut contract = env.add_contract();
    implementation::erc1271_test_setup::register_mock(&mut contract);
    implementation::simple_wallet_test_setup::register_mock(&mut contract);
    contract.as_contract()
}
