//! Implementation of Initial Token Claim contract

use super::interface::*;
use alloc::format;
use alloc::vec::Vec;
use sdk::*;

use groundhog_erc20_if::ERC20Client;

#[sdk::storage]
#[derive(Debug)]
pub struct InitialTokenClaimContract;

// This implements the InitialTokenClaim interface. The `contract` macro provides
// an exported entrypoint for each method; these will be placed in a
// module with the specified name ("guest_exports").
#[contract]
impl InitialTokenClaim for InitialTokenClaimContract {
    fn claim_initial_tokens(
        &mut self,
        hash: &SHA256Hash,
        nonce: &SHA256Hash,
        entrypoint: &ContractEntrypointID,
        calldata: &[u8],
    ) {
        // Create storage key: sha256("Initial Deployment" || hash || nonce || entrypoint || calldata)
        let mut key_data = Vec::new();
        key_data.extend_from_slice(b"Initial Deployment");
        key_data.extend_from_slice(hash.as_ref());
        key_data.extend_from_slice(nonce.as_ref());
        key_data.extend_from_slice(&entrypoint.into_inner());
        key_data.extend_from_slice(calldata);
        let storage_key = sdk::crypto::sha256(&key_data);
        let contract_data_key = StorageKeyContractData::from(storage_key);

        // Load the allocation amount
        let allocation: u128 = non_negative_int_get(&contract_data_key) as u128;

        // Delete the storage key to prevent double-claiming
        // Convert StorageKey to StorageKeyContractData for the syscall
        sdk::delete(&contract_data_key, true);

        // Calculate receiver address from contract hash and nonce
        let receiver_addr = sdk::get_deploy_address(nonce, hash);

        // Transfer tokens to the receiver via HOG token contract
        // TODO: Call the HOG token contract's transfer method
        // For now, we'll just log the transfer
        let erc20_client =
            ERC20Client::new(&sdk::precompiles::ADDRESS_PRECOMPILED_HOG_TOKEN);

        erc20_client.transfer(&receiver_addr, allocation);

        // Deploy the contract using hash and nonce
        let deployed_addr = sdk::deploy_contract(nonce, hash);
        sdk::log(format!("Deployed contract address: {:?}", deployed_addr).as_bytes());

        let auth_handles = AuthHandleBuf::new();

        // Call the initialize entrypoint with provided calldata
        let _ = sdk::call::contract_call(&deployed_addr,
            *entrypoint,
            u64::MAX,
            calldata,
            &auth_handles,
            0,
        ).unwrap();

        sdk::log(format!("End ITC").as_bytes());
    }
}
