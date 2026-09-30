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

/// An ordinary call to the newly deployed contract, made by the wallet.
#[contract_type]
pub struct WalletInitCall {
    pub entrypoint: ContractEntrypointID,
    pub gas_limit: u64,
    pub calldata: WalletBytes,
}

#[contract_exception]
#[derive(Copy, Clone)]
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
    /// Fund, deploy and optionally initialize a contract in one transaction.
    /// Zero amount skips funding; otherwise transfer exactly the requested HOG.
    /// The initializer's target is the new contract and its caller is this wallet.
    /// Its return value is discarded. Any failure rolls all three steps back.
    fn deploy(
        nonce: &SHA256Hash,
        contract_hash: &SHA256Hash,
        amount: i128,
        initializer: Option<WalletInitCall>,
    ) -> ContractAddress;
}
