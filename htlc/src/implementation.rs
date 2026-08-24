use crate::HTLC;
use sdk::*;

use sdk::storage::{Deletable, Map, Value};

use groundhog_erc20_if::ERC20Client;

// An HTLC can be in 3 states:
// 1. Created: The HTLC has been created and can be claimed.
// 2. Claimed: The HTLC has been claimed and the funds have been transferred to the receiver.
// 3. Refunded: The HTLC has been refunded and the funds have been returned to the sender.
// The value only exists on the ledger when the HTLC is in the Created state.
// It is the user's responsibility to not reuse htlc hashes (otherwise funds can be immediately stolen).
// Uses delete exclusive to ensure claims and refunds happen once and only once.
#[derive(ReadXDR, WriteXDR)]
#[xdrs(crate = "::sdk::__xdrs")]
struct HTLCEntry {
    sender: ContractAddress,
    receiver: ContractAddress,
    token: ContractAddress,
    amount: i128,
    expiration_height: BlockHeight,
}

impl HTLCEntry {
    pub fn token_client(&self) -> ERC20Client {
        ERC20Client::new(&self.token)
    }
}

#[storage]
struct HTLCContract {
    htlcs: Map<SHA256Hash, Value<HTLCEntry>>,
}

fn get_block_height() -> BlockHeight {
    let mut height : BlockHeight = 0.into();
    sdk::sys::raw_GetBlockHeight(&mut height).unwrap();
    height
}

#[contract]
impl HTLC for HTLCContract {

    /// Intentionally a no-op: the contract keeps no root-level state to set
    /// up (channels live in the `htlcs` map). The entrypoint stays in the
    /// interface because hogd's htlc_integration_test drives it as part of
    /// the standard deploy-then-initialize flow.
    fn initialize(&mut self) {}

    fn create_htlc(&mut self, 
        receiver: &ContractAddress, 
        token: &ContractAddress, 
        amount: i128, 
        expiration_height: BlockHeight,
        hash: &SHA256Hash) {

        assert!(amount > 0);

        if self.htlcs.at(hash).try_get().none_if_missing().is_some() {
            panic!("HTLC already exists");
        }

        let htlc = HTLCEntry {
            sender: sdk::get_caller(),
            receiver: receiver.clone(),
            token: token.clone(),
            amount: amount.clone(),
            expiration_height: expiration_height.clone(),
        };

        let token_client = htlc.token_client();
        let self_address = sdk::get_current_address();
        token_client.transfer_from(&htlc.sender, &self_address, htlc.amount as u128);

        self.htlcs.at_mut_exclusive(hash).set(&htlc);
    }

    fn claim_htlc(&mut self, 
        hash: &SHA256Hash, 
        preimage: &SHA256Hash) {

        let htlc = self.htlcs.at(hash).get();

        if htlc.expiration_height < get_block_height() {
            panic!("HTLC expired");
        }

        if sdk::crypto::sha256(preimage.as_ref()) != *hash {
            panic!("Invalid preimage");
        }

        let token_client = htlc.token_client();
        token_client.transfer(&htlc.receiver, htlc.amount as u128);

        self.htlcs.at_mut_exclusive(hash).delete();
    }

    fn refund_htlc(&mut self, 
        hash: &SHA256Hash) -> () {
        let htlc = self.htlcs.at(hash).get();

        if htlc.expiration_height >= get_block_height() {
            panic!("HTLC not expired");
        }

        let token_client = htlc.token_client();
        token_client.transfer(&htlc.sender, htlc.amount as u128);

        self.htlcs.at_mut_exclusive(hash).delete();
    }
}
