#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "dev")]
extern crate std;

extern crate alloc;

mod interface;
mod implementation;

// A client stub that provides the interface via cross-contract calls
pub use interface::Client;

pub use implementation::SimpleWalletContract;

#[cfg(feature = "dev")]
pub use implementation::ERC1271TestSetup;
#[cfg(feature = "dev")]
pub use implementation::SimpleWalletTestSetup;

#[cfg(feature = "dev")]
pub fn register(
    env: &sdk::TestEnvironmentHandle,
) -> ::sdk::types::Contract {
    let mut contract = env.add_contract();
    contract.with_impl::<SimpleWalletContract, ERC1271TestSetup>();
    contract.with_impl::<SimpleWalletContract, SimpleWalletTestSetup>();
    contract.as_contract()
}
