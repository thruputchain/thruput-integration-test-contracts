//! A simple wallet: one owner key, whose signature over an encoded
//! invocation authorizes the wallet to perform that call.
//!
//! Verifies the signature directly. This used to go through the ERC1271
//! interface, which the SDK has retired in favour of `SignedOp`; the wire
//! shape here — `do_action(signature, invocation_bytes)` — is unchanged, as
//! hogd's load generators and integration tests drive it.

extern crate alloc;

use super::interface::*;
use sdk::crypto::verify_signature_ed25519;
use sdk::*;

#[contract_exception]
pub enum WalletError {
    /// The signature does not verify under the owner's key.
    InvalidSignature = 1,
}

#[storage]
pub struct SimpleWalletContract {
    owner: sdk::storage::Value<Ed25519PublicKey>,
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
        if !verify_signature_ed25519(&self.owner.get(), signature, invocation_bytes) {
            sdk::throw(WalletError::InvalidSignature);
        }

        let (addr, entrypoint, gas_limit, calldata): (
            ContractAddress,
            ContractEntrypointID,
            u64,
            &[u8],
        ) = sdk::__xdrs::txs::decode(invocation_bytes).unwrap();

        let auth_handles = AuthHandleBuf::new();

        let mut return_buf = sdk::call::ReturnBuf::new();
        let mut returned_auth_handles = AuthHandleBuf::new();
        let _ = sdk::call::contract_call(
            &addr,
            entrypoint,
            gas_limit,
            calldata,
            &auth_handles,
            0,
            &mut return_buf,
            &mut returned_auth_handles,
        )
        .unwrap();
    }
}
