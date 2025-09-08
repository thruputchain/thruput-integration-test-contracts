//! An implementation of Fibonacci

use super::interface::*;
use sdk::*;
use sdk::serialize::*;
use sdk::crypto::verify_signature_ed25519;

use groundhog_erc1271_if::ERC1271;
use sdk::contract_result::caller_error;

#[derive(Storage, Debug, Serialize)]
pub struct SimpleWalletContract {
    owner: Ed25519PublicKey,
}

#[contract(exports = "erc1271_exports", test_setup = "erc1271_test_setup")]
impl ERC1271 for SimpleWalletContract {
    fn is_valid_signature(&self, msg: &[u8], signature: &[u8]) -> Result<bool, sdk::contract_result::Error> {
        if signature.len() != 64 {
            return caller_error("Invalid signature length");
        }

        let signature = Ed25519Signature::new(signature.try_into().unwrap());
        Ok(verify_signature_ed25519(&self.owner, &signature, &msg).is_ok())
    }
}

// This implements the Fibonacci interface.  The `contract` macro provides
// an exported entrypoint for each method; these will be placed in a
// module with the specified name ("guest_exports").
#[contract(exports = "guest_exports", test_setup = "simple_wallet_test_setup")]
impl SimpleWallet for SimpleWalletContract {
    fn initialize(
        &mut self,
        owner: &Ed25519PublicKey,
    ) {
        self.owner = owner.clone();
    }

    fn do_action(&self, signature: &Ed25519Signature, invocation_bytes: &[u8])  {
        if !self.is_valid_signature(invocation_bytes, signature.as_ref()).unwrap() {
            sdk::log::log(&format!("invalid signature").as_bytes()).unwrap();
            trap();
        }

        let res = sdk::serialize::decode_xdr(&invocation_bytes);
        let (addr, entrypoint, gas_limit, calldata) : (ContractAddress, ContractEntrypointID, u64, &[u8]) = res.unwrap();

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
