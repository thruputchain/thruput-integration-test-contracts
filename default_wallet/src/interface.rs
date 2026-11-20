use alloc::vec::Vec;
use thruput_sdk::*;

pub type WalletBytes = BytesM;

/// The most tokens `execute` mints.
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
    /// The auth tokens to forward, in the order the target's `Signed`
    /// arguments take them.
    pub tokens: VecM<WalletTokenRef, MAX_CALL_TOKENS>,
}

/// Names one auth token the wallet holds, by where it came from rather than
/// by its runtime id, so a signer can write the reference without knowing how
/// many tokens any callee will return.
#[contract_type]
#[derive(Copy, Clone)]
pub enum WalletTokenRef {
    /// A token `execute` minted: its position in `mints`.
    Minted(u32) = 0,
    /// A token an earlier call in the batch returned: that call's position in
    /// `calls`, and the token's position among the tokens it returned.
    Returned { call: u32, index: u32 } = 1,
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
    /// that is not earlier in the batch, or an index past what that call
    /// returned.
    InvalidTokenRef = 8,
    /// A token reference names a token the batch has already forwarded.
    TokenAlreadyForwarded = 9,
}

#[contract(client = Client, throws = WalletError)]
pub trait DefaultWallet {
    /// Mint `mints` as this wallet, then run `calls` in order, forwarding to
    /// each the tokens it references.
    ///
    /// Each mint is an XDR-encoded `AuthTokenNode`, valid for the current
    /// block only. Its minter is the wallet, so a consumer that trusts the
    /// minter address is trusting the account that signed the transaction.
    /// Tokens left unforwarded when the batch ends are dropped. `mints` and
    /// each call's `tokens` are bounded by their types, so a longer list
    /// fails to decode instead of reaching the wallet.
    fn execute(
        mints: VecM<WalletBytes, MAX_MINTS>,
        calls: Vec<WalletCall>,
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
