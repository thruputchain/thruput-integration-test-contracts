#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "dev")]
extern crate std;

extern crate alloc;

mod interface;
mod implementation;

// A client stub that provides the interface via cross-contract calls
pub use interface::Client;

pub use implementation::SimpleWalletContract;

pub use interface::SimpleWallet;
pub use groundhog_erc1271_if::ERC1271;

#[cfg(feature = "dev")]
pub fn register(
    env: &sdk::TestEnvironmentHandle,
) -> ::sdk::types::Contract {
    let mut contract = env.add_contract();
    contract.with_impl::<SimpleWalletContract, dyn ERC1271>();
    contract.with_impl::<SimpleWalletContract, dyn SimpleWallet>();
    contract.as_contract()
}
