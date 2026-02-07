use super::interface::*;
use sdk::*;

use sdk::storage::Map;

use groundhog_erc20_if::ERC20Client;

use xdrs::WriteXDR;

// An HTLC can be in 3 states:
// 1. Created: The HTLC has been created and can be claimed.
// 2. Claimed: The HTLC has been claimed and the funds have been transferred to the receiver.
// 3. Refunded: The HTLC has been refunded and the funds have been returned to the sender.
// The value only exists on the ledger when the HTLC is in the Created state.
// It is the user's responsibility to not reuse htlc hashes (otherwise funds can be immediately stolen).
// Uses delete exclusive to ensure claims and refunds happen once and only once.
#[derive(Storage, WriteXDR)]
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
pub struct HTLCContract {
    htlcs: Map<SHA256Hash, HTLCEntry>,
}

fn get_block_height() -> BlockHeight {
    let mut height : BlockHeight = 0;
    sdk::sys::raw_GetBlockHeight(&mut height).unwrap();
    height
}

#[contract(exports = "guest_exports")]
impl HTLC for HTLCContract {

    #[init]
    fn initialize(&mut self) -> () {
        ()
    }

    fn create_htlc(&mut self, 
        receiver: &ContractAddress, 
        token: &ContractAddress, 
        amount: i128, 
        expiration_height: BlockHeight, 
        hash: &SHA256Hash) -> () {

        assert!(amount > 0);

        if self.htlcs.try_get(hash).unwrap().is_some() {
            panic!("HTLC already exists");
        }

        let htlc = HTLCEntry {
            sender: sdk::get_caller().unwrap(),
            receiver: receiver.clone(),
            token: token.clone(),
            amount: amount.clone(),
            expiration_height: expiration_height.clone(),
        };

        let mut token_client = htlc.token_client();
        let self_address = sdk::get_current_address().unwrap();
        token_client.transfer_from(&htlc.sender, &self_address, htlc.amount as u128).unwrap();

        self.htlcs.set_exclusive(hash, &htlc).unwrap();
    }

    fn claim_htlc(&mut self, 
        hash: &SHA256Hash, 
        preimage: &SHA256Hash) -> () {
        
        let htlc = self.htlcs.get(hash).unwrap();

        if htlc.expiration_height < get_block_height() {
            panic!("HTLC expired");
        }

        if sdk::crypto::sha256(preimage) != *hash {
            panic!("Invalid preimage");
        }

        let mut token_client = htlc.token_client();
        token_client.transfer(&htlc.receiver, htlc.amount as u128).unwrap();

        self.htlcs.delete_exclusive(hash).unwrap();
    }

    fn refund_htlc(&mut self, 
        hash: &SHA256Hash) -> () {
        let htlc = self.htlcs.get(hash).unwrap();

        if htlc.expiration_height >= get_block_height() {
            panic!("HTLC not expired");
        }

        let mut token_client = htlc.token_client();
        token_client.transfer(&htlc.sender, htlc.amount as u128).unwrap();

        self.htlcs.delete_exclusive(hash).unwrap();
    }
}

