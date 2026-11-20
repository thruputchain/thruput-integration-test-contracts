#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "dev")]
extern crate std;

extern crate alloc;

mod implementation;
mod interface;

pub use implementation::DefaultWalletContract;
pub use interface::{
    Client, DefaultWallet, DefaultWalletEntrypoints, WalletBytes, WalletCall,
    WalletError, WalletInitCall, WalletTokenRef, MAX_CALL_TOKENS, MAX_MINTS,
};

thruput_sdk::declare_mock_contract!();
