//! An implementation of Fibonacci

use super::interface::*;
use sdk::crypto::verify_signature_ed25519;
use sdk::*;

use groundhog_erc1271_if::ERC1271;

#[storage]
pub struct SimpleWalletContract {
    owner: Ed25519PublicKey,
}

#[contract(exports = "erc1271_exports", test_setup = "erc1271_test_setup", version = "0.1.0")]
impl ERC1271 for SimpleWalletContract {
    fn is_valid_signature(
        &self,
        msg: &[u8],
        signature: &[u8],
    ) -> bool {
        if signature.len() != 64 {
            panic!("Invalid signature length");
        }

        let signature = Ed25519Signature::new(signature.try_into().unwrap());

        verify_signature_ed25519(&self.owner, &signature, &msg)
    }
}

// This implements the Fibonacci interface.  The `contract` macro provides
// an exported entrypoint for each method; these will be placed in a
// module with the specified name ("guest_exports").
#[contract(test_setup = "simple_wallet_test_setup")]
impl SimpleWallet for SimpleWalletContract {
    #[init]
    fn initialize(&mut self, owner: &Ed25519PublicKey) {
        self.owner = owner.clone();
    }

    fn do_action(
        &self,
        signature: &Ed25519Signature,
        invocation_bytes: &[u8],
    ) {
        if !self
            .is_valid_signature(invocation_bytes, signature.as_ref())
        {
            sdk::log(&format!("invalid signature").as_bytes());
            trap();
        }

        let (addr, entrypoint, gas_limit, calldata): (
            ContractAddress,
            ContractEntrypointID,
            u64,
            &[u8],
        ) = xdrs::txs::decode(&invocation_bytes).unwrap();

        let _ = sdk::call::contract_call(&addr,
            entrypoint,
            gas_limit,
            calldata,
        ).unwrap();
    }
}
