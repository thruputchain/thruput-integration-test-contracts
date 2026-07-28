use alloc::vec::Vec;
use sdk::*;

pub type WalletBytes = BytesM;

#[contract_type]
pub struct WalletCall {
    pub target: ContractAddress,
    pub entrypoint: ContractEntrypointID,
    pub gas_limit: u64,
    pub calldata: WalletBytes,
}

#[contract_exception]
#[derive(Copy)]
pub enum WalletError {
    NotRootInvocation = 1,
    InvalidAuthClaim = 2,
    EmptyCallList = 3,
    TooManyCalls = 4,
    CallAborted = 5,
    CallFaulted = 6,
    ReturnDataTooLarge = 7,
}

#[contract(client = Client, throws = WalletError)]
pub trait DefaultWallet {
    fn execute(calls: Vec<WalletCall>) -> Vec<WalletBytes>;
    fn deploy(nonce: &SHA256Hash, contract_hash: &SHA256Hash) -> ContractAddress;
}
