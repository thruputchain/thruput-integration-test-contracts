use alloc::vec::Vec;
use thruput_sdk::*;

pub type WalletBytes = BytesM;

/// The most tokens `execute_with_tokens` mints.
pub const MAX_MINTS: u32 = 16;
/// The most tokens one call forwards: the default of the chain's
/// `kConfigParamMaxAuthTokens`, which bounds what an invocation accepts.
pub const MAX_CALL_TOKENS: u32 = 64;

#[contract_type]
pub struct WalletCall {
    pub target: ContractAddress,
    pub entrypoint: ContractEntrypointID,
    pub gas_limit: u64,
    pub calldata: WalletBytes,
}

/// Where a forwarded auth token comes from.
#[contract_type]
#[derive(Copy, Clone)]
pub enum WalletTokenSource {
    /// One of the tokens `execute_with_tokens` minted from its `mints`.
    Minted = 0,
    /// One of the tokens an earlier call in the same batch returned.
    Returned = 1,
}

/// Names one auth token the wallet holds, by where it came from rather than
/// by its runtime id, so a signer can write the reference without knowing how
/// many tokens any callee will return.
#[contract_type]
#[derive(Copy, Clone)]
pub struct WalletTokenRef {
    pub source: WalletTokenSource,
    /// For `Returned`, the index in `calls` of the call that returned the
    /// token; it must be an earlier call. For `Minted`, must be zero.
    pub call: u32,
    /// For `Minted`, the index in `mints`. For `Returned`, the token's
    /// position among the tokens that call returned.
    pub index: u32,
}

/// A [`WalletCall`] that also forwards auth tokens to its target.
#[contract_type]
pub struct WalletTokenCall {
    pub target: ContractAddress,
    pub entrypoint: ContractEntrypointID,
    pub gas_limit: u64,
    pub calldata: WalletBytes,
    /// Forwarded in this order, which is the order the target's `Signed`
    /// arguments take them in.
    pub tokens: VecM<WalletTokenRef, MAX_CALL_TOKENS>,
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
    /// A token reference names no token: a mint index past `mints`, a call
    /// that is not earlier in the batch, an index past what that call
    /// returned, or a `Minted` reference with a nonzero `call`.
    InvalidTokenRef = 8,
    /// A token reference names a token the batch has already forwarded.
    TokenAlreadyForwarded = 9,
}

#[contract(client = Client, throws = WalletError)]
pub trait DefaultWallet {
    fn execute(calls: Vec<WalletCall>) -> Vec<WalletBytes>;
    /// `execute`, with auth tokens: mint `mints` as this wallet, then run
    /// `calls` in order, forwarding to each the tokens it references.
    ///
    /// Each mint is an XDR-encoded `AuthTokenNode`, valid for the current
    /// block only. `mints` and each call's `tokens` are bounded by their
    /// types, so a longer list fails to decode instead of reaching the wallet. Its minter is the wallet, so a consumer that trusts the
    /// minter address is trusting the account that signed the transaction.
    /// Tokens left unforwarded when the batch ends are dropped.
    fn execute_with_tokens(
        mints: VecM<WalletBytes, MAX_MINTS>,
        calls: Vec<WalletTokenCall>,
    ) -> Vec<WalletBytes>;
    /// Fund, deploy and optionally initialize a contract in one transaction.
    /// Zero amount skips funding; otherwise transfer exactly the requested
    /// native tokens.
    /// The initializer's target is the new contract and its caller is this wallet.
    /// Its return value is discarded. Any failure rolls all three steps back.
    fn deploy(
        nonce: &SHA256Hash,
        contract_hash: &SHA256Hash,
        amount: i128,
        initializer: Option<WalletInitCall>,
    ) -> ContractAddress;
}
