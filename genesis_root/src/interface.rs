use alloc::vec::Vec;
use sdk::*;

/// Fund an existing address without deploying or initializing a contract.
#[contract_type]
pub struct AccountAllocation {
    pub address: ContractAddress,
    pub allocation: i128,
}

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
        account_allocations: Vec<AccountAllocation>,
        root_funding: i128,
        token_funding: i128,
    );
}
