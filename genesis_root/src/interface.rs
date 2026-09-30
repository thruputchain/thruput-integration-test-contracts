use alloc::vec::Vec;
use sdk::*;

#[contract_type]
pub struct Distribution {
    pub nonce: SHA256Hash,
    pub contract_hash: SHA256Hash,
    pub initializer: ContractEntrypointID,
    pub initializer_calldata: BytesM,
    pub allocation: i128,
}

#[contract(client = Client)]
pub trait GenesisRoot {
    fn distribute(
        distributions: Vec<Distribution>,
        root_funding: i128,
        token_funding: i128,
    );
}
