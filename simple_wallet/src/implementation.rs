//! An implementation of Fibonacci

use super::interface::*;
use sdk::storage_prelude::*;
use sdk::crypto::verify_signature_ed25519;

#[derive(Storage, Debug, Serialize)]
pub struct SimpleWalletContract {
    owner: Ed25519PublicKey,
}

// This implements the Fibonacci interface.  The `contract` macro provides
// an exported entrypoint for each method; these will be placed in a
// module with the specified name ("guest_exports").
#[contract(exports = "guest_exports")]
impl SimpleWallet for SimpleWalletContract {
    fn initialize(
        &mut self,
        owner: &Ed25519PublicKey,
    ) {
        self.owner = owner.clone();
    }

    fn do_action(&self, signature: &Ed25519Signature, invocation_bytes: &[u8])  {
        verify_signature_ed25519(&self.owner, signature, &invocation_bytes).unwrap();

        let (addr, entrypoint, gas_limit, calldata) : (ContractAddress, ContractEntrypointID, u64, &[u8]) = sdk::serialize::from_bytes(&invocation_bytes).unwrap();

        let mut return_len = 0;
        let method_id = u64::from_be_bytes(entrypoint.into_inner());

        sdk::sys::raw_Invoke(
            &addr,
            method_id,
            gas_limit,
            &calldata,
            0, // TODO(geoff): invoke flags go here
            &mut return_len).unwrap();
    }
}
