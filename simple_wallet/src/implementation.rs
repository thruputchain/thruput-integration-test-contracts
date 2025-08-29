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

#[contract(exports = "erc1271_exports")]
impl ERC1271 for SimpleWalletContract {
    fn is_valid_signature(&self, msg: &[u8], signature: &[u8]) -> Result<bool, sdk::contract_result::Error> {
        sdk::log::log(&format!("is_valid_signature: msg = {:?}", msg).as_bytes()).unwrap();
        sdk::log::log(&format!("is_valid_signature: signature = {:?}", signature).as_bytes()).unwrap();

        if signature.len() != 64 {
            sdk::log::log(&format!("is_valid_signature: Invalid signature length").as_bytes()).unwrap();
            return caller_error("Invalid signature length");
        }

        let signature = Ed25519Signature::new(signature.try_into().unwrap());
        Ok(verify_signature_ed25519(&self.owner, &signature, &msg).is_ok())
    }
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
        sdk::log::log(&format!("do_action: {:?} len = {}", invocation_bytes, invocation_bytes.len()).as_bytes()).unwrap();

        if !self.is_valid_signature(invocation_bytes, signature.as_ref()).unwrap() {
            sdk::log::log(&format!("invalid signature").as_bytes()).unwrap();
            trap();
        }
        sdk::log::log(&format!("valid signature").as_bytes()).unwrap();

        let res = sdk::serialize::from_bytes(&invocation_bytes[0..48]);
        sdk::log::log(&format!("res = {:?}", res).as_bytes()).unwrap();

        let (addr, entrypoint, gas_limit) : (ContractAddress, ContractEntrypointID, u64) = res.unwrap();

        let remaining = &invocation_bytes[48..80];

        sdk::log::log(&format!("remaining = {:?}", remaining).as_bytes()).unwrap();
    
        let calldata_res = sdk::serialize::from_bytes(remaining);
        sdk::log::log(&format!("calldata_res = {:?}", calldata_res).as_bytes()).unwrap();
        let calldata : &[u8] = calldata_res.unwrap();

        sdk::log::log(&format!("do_action: calldata = {:?}", calldata).as_bytes()).unwrap();

        //let (addr, entrypoint, gas_limit, calldata) : (ContractAddress, ContractEntrypointID, u64, &[u8]) = sdk::serialize::from_bytes(&invocation_bytes).unwrap();

        sdk::log::log(&format!("do_action: entrypoint = {:?}", entrypoint).as_bytes()).unwrap();
        let mut return_len = 0;
        let method_id = u64::from_be_bytes(entrypoint.into_inner());

        sdk::log::log(&format!("do_action: addr = {:?}", addr).as_bytes()).unwrap();

        sdk::sys::raw_Invoke(
            &addr,
            method_id,
            gas_limit,
            &calldata,
            0, // TODO(geoff): invoke flags go here
            &mut return_len).unwrap();
    }
}
