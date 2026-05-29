#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "dev")]
extern crate std;

extern crate alloc;

mod implementation;
mod interface;

pub use implementation::DeployAndTransferContract;
pub use interface::Client;
pub use interface::DeployAndTransfer;

#[cfg(feature = "dev")]
pub fn register(env: &sdk::TestEnvironmentHandle) -> ::sdk::types::Contract {
    let mut contract = env.add_contract();
    contract.register(<DeployAndTransferContract as DeployAndTransfer>::__mock);
    contract.as_contract()
}
