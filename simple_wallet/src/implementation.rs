//! A simple wallet contract implementing ERC1271 signature verification.

extern crate alloc;

use super::interface::*;
use alloc::vec::Vec;
use groundhog_erc1271_if::{ERC1271, ERC1271Error};
use sdk::crypto::verify_signature_ed25519;
use sdk::signer::{Opaque, Signed};
use sdk::*;

#[storage]
pub struct SimpleWalletContract {
    owner: sdk::storage::Value<Ed25519PublicKey>,
}

#[contract(storage = false, version = "0.1.0")]
impl ERC1271 for SimpleWalletContract {
    fn is_valid_signature(
        &self,
        msg: &[u8],
        signature: &[u8],
    ) -> Vec<Signed<Opaque>> {
        if signature.len() != 64 {
            sdk::throw(ERC1271Error::InvalidSignature);
        }

        let signature = Ed25519Signature::new(signature.try_into().unwrap());

        if !verify_signature_ed25519(&self.owner.get(), &signature, msg) {
            sdk::throw(ERC1271Error::InvalidSignature);
        }

        Vec::new()
    }
}

#[contract]
impl SimpleWallet for SimpleWalletContract {
    fn initialize(&mut self, owner: &Ed25519PublicKey) {
        self.owner.set(owner);
    }

    fn do_action(
        &self,
        signature: &Ed25519Signature,
        invocation_bytes: &[u8],
    ) {
        self.is_valid_signature(invocation_bytes, signature.as_ref());

        let (addr, entrypoint, gas_limit, calldata): (
            ContractAddress,
            ContractEntrypointID,
            u64,
            &[u8],
        ) = sdk::__xdrs::txs::decode(invocation_bytes).unwrap();

        let auth_handles = AuthHandleBuf::new();

        let _ = sdk::call::contract_call(
            &addr,
            entrypoint,
            gas_limit,
            calldata,
            &auth_handles,
        )
        .unwrap();
    }
}
