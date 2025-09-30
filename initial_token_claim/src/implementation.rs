//! Implementation of Initial Token Claim contract

use super::interface::*;
use sdk::*;

use groundhog_erc20_if::ERC20Client;

#[derive(Default, Debug)]
pub struct InitialTokenClaimContract {
    // This contract doesn't need persistent state
}

// This implements the InitialTokenClaim interface. The `contract` macro provides
// an exported entrypoint for each method; these will be placed in a
// module with the specified name ("guest_exports").
#[contract(exports = "guest_exports", storage = "false")]
impl InitialTokenClaim for InitialTokenClaimContract {
    fn claim_initial_tokens(
        &mut self,
        hash: &SHA256Hash,
        nonce: &SHA256Hash,
        method: &ContractEntrypointID,
        calldata: &[u8],
    ) -> () {
        // Create storage key: sha256("Initial Deployment" || hash || nonce || entrypoint || calldata)
        let mut key_data = Vec::new();
        key_data.extend_from_slice(b"Initial Deployment");
        key_data.extend_from_slice(hash.as_ref());
        key_data.extend_from_slice(nonce.as_ref());
        key_data.extend_from_slice(&method.into_inner());
        key_data.extend_from_slice(calldata);
        let storage_key = sdk::crypto::sha_256(&key_data).unwrap();
        let contract_data_key = StorageKeyContractData::from(storage_key);

        // Load the allocation amount
        let allocation: u128 =
            non_negative_int_get(&contract_data_key).unwrap() as u128;

        // Delete the storage key to prevent double-claiming
        // Convert StorageKey to StorageKeyContractData for the syscall
        sdk::delete::delete(&contract_data_key, true).unwrap();

        // Calculate receiver address from contract hash and nonce
        let mut receiver_addr = ContractAddress::default();
        sdk::sys::raw_GetDeployAddress(None, nonce, hash, &mut receiver_addr)
            .unwrap();

        // Transfer tokens to the receiver via HOG token contract
        // TODO: Call the HOG token contract's transfer method
        // For now, we'll just log the transfer
        let mut erc20_client = ERC20Client::new(&sdk::precompiles::ADDRESS_PRECOMPILED_HOG_TOKEN);
        erc20_client.transfer(&receiver_addr, allocation).unwrap();
    

        // Deploy the contract using hash and nonce
        let mut deployed_addr = ContractAddress::default();
        sdk::sys::raw_DeployContract(nonce, hash, &mut deployed_addr).unwrap();

        // Call the initialize entrypoint with provided calldata
        let method_id = u64::from_be_bytes(method.into_inner());
        let mut return_len = 0;
        sdk::sys::raw_Invoke(
            &deployed_addr,
            method_id,
            u64::MAX,
            calldata,
            0,
            &mut return_len,
        )
        .unwrap();
    }
}
