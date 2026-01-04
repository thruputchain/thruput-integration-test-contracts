#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std;

// A trait describing the contract interface
pub mod interface;

// A client stub that provides the interface via cross-contract calls
#[cfg(feature = "client")]
pub use interface::Client;

#[cfg(feature = "guest")]
const _: () = {
    compile_error!("nocontract_wallet is not implemented as a wasm contract");
};

