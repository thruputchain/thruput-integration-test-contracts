//! An implementation of Fibonacci

use super::interface::SimpleWallet;
use sdk::storage_prelude::*;
<<<<<<< HEAD
use sdk::crypto::{verify_signature_ed25519};

use alloc::vec::Vec;
=======
use sdk::crypto::verify_signature;
>>>>>>> 07c6879 (contracts receive shared slices instead of owned vecs)

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
        this: Option<&mut Self>,
        owner: Ed25519PublicKey,
    ) -> Result<()> {
        // Make sure we haven't already been initialized
        if this.is_some() {
            panic!("Contract already initialized");
        }

        initialize_root(|_storage: &mut Self| {
            _storage.owner = owner;
        })?;

        Ok(())
    }
    fn do_action(&self, signature: &Ed25519Signature, invocation_bytes: &[u8]) -> Result<()> {
        verify_signature_ed25519(&self.owner, signature, &invocation_bytes)?;

        let (addr, entrypoint, gas_limit, calldata) : (ContractAddress, ContractEntrypointID, u64, &[u8]) = sdk::serialize::from_bytes(&invocation_bytes).unwrap();

        let mut return_len = 0;
        let method_id = u64::from_be_bytes(entrypoint.into_inner());

        sdk::sys::raw_Invoke(
            &addr,
            method_id,
            gas_limit,
            &calldata,
            0, // TODO(geoff): invoke flags go here
            &mut return_len)?;

        Ok(())
    }
}
