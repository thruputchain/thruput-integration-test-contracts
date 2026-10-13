use thruput_sdk::*;

#[contract(client = Client)]
pub trait HTLC {

    // initialize storage layer for contract
    fn initialize(&mut self);

    fn create_htlc(&mut self, 
        receiver: &ContractAddress, 
        token: &ContractAddress, 
        amount: i128, 
        expiration_height: BlockHeight, 
        hash: &SHA256Hash);

    fn claim_htlc(&mut self, 
        hash: &SHA256Hash, 
        preimage: &SHA256Hash);

    fn refund_htlc(&mut self, 
        hash: &SHA256Hash);
}
