use super::interface::*;
use alloc::vec::Vec;
use thruput_domain_separator::syscall::pubkey_account_address;
use thruput_erc20_if::ERC20Client;
use thruput_sdk::sys::{self, SyscallErrorCode};
use thruput_sdk::*;

pub struct DefaultWalletContract;

const MAX_CALLS: usize = 16;
// The return-buffer limit this wallet is built for. The protocol does not
// fix a constant — the limit is a chain's
// `kConfigParamMaxReturnBufferLen` — so this compiles in the value the
// harness configures, which is what hogd's `precompile_default_wallet.cc`
// mirrors.
const MAX_RETURN_DATA_BYTES: usize = 1024;

fn cxs_u32_len(value: usize) -> Option<usize> {
    let value = u32::try_from(value).ok()?;
    Some(match value {
        0..=62 => 1,
        63..=255 => 2,
        256..=65_535 => 3,
        _ => 5,
    })
}

fn encoded_return_item_len(return_data_len: usize) -> Option<usize> {
    cxs_u32_len(return_data_len)?.checked_add(return_data_len)
}

fn require_root_invocation() {
    let mut sender = ContractAddress::from([0u8; 32]);
    match sys::raw_GetMsgSender(&mut sender) {
        Err(e)
            if e == SyscallErrorCode::kSyscallErrorCodeIsRootTransaction
                .into() => {}
        _ => throw(WalletError::NotRootInvocation),
    }
}

// The claim's preimage names the account key by its `PublicKeyHash`, which
// commits to the scheme, so the address re-derives from the claim alone for
// every scheme (Ed25519, ML-DSA-65). The wallet never sees the key itself:
// admission checks it against that hash and verifies the signature.
fn require_pubkey_signature_auth() {
    match thruput_sdk::get_auth_claim() {
        Ok(AuthClaim::kPubkeySignature { pubkey_preimage })
            if pubkey_account_address(&pubkey_preimage)
                == thruput_sdk::get_current_address() => {}
        _ => throw(WalletError::InvalidAuthClaim),
    }
}

fn require_root_pubkey_auth() {
    require_root_invocation();
    require_pubkey_signature_auth();
}

/// Call `target`, forwarding `auth_handles`, and return its return data.
/// The ids of the tokens it returned land in `returned_auth_handles`.
fn invoke(
    target: &ContractAddress,
    entrypoint: ContractEntrypointID,
    gas_limit: u64,
    calldata: &[u8],
    auth_handles: &AuthHandleBuf,
    returned_auth_handles: &mut AuthHandleBuf,
) -> Vec<u8> {
    let mut return_buf = Vec::new();
    match thruput_sdk::call::contract_call(
        target,
        entrypoint,
        gas_limit,
        calldata,
        auth_handles,
        0,
        &mut return_buf,
        returned_auth_handles,
    ) {
        Ok(thruput_sdk::call::CallOutcome::Return) => return_buf,
        Ok(thruput_sdk::call::CallOutcome::Abort) => {
            match RawExceptionRef::parse(&return_buf) {
                Ok(exn) => exn.re_throw(),
                Err(_) => throw(WalletError::CallAborted),
            }
        }
        Err(_) => throw(WalletError::CallFaulted),
    }
}

fn require_call_count(call_count: usize) {
    if call_count == 0 {
        throw(WalletError::EmptyCallList);
    }
    if call_count > MAX_CALLS {
        throw(WalletError::TooManyCalls);
    }
}

/// The batch's return value as it accumulates, held to the return-buffer
/// limit after every call so an oversized batch stops at the call that
/// crosses it.
struct BatchReturns {
    returns: Vec<WalletBytes>,
    encoded_len: usize,
}

impl BatchReturns {
    fn new(call_count: usize) -> Self {
        Self {
            returns: Vec::with_capacity(call_count),
            encoded_len: cxs_u32_len(call_count)
                .unwrap_or_else(|| throw(WalletError::ReturnDataTooLarge)),
        }
    }

    fn push(&mut self, return_buf: &[u8]) {
        self.encoded_len = self
            .encoded_len
            .checked_add(
                encoded_return_item_len(return_buf.len()).unwrap_or_else(
                    || throw(WalletError::ReturnDataTooLarge),
                ),
            )
            .unwrap_or_else(|| throw(WalletError::ReturnDataTooLarge));
        if self.encoded_len > MAX_RETURN_DATA_BYTES {
            throw(WalletError::ReturnDataTooLarge);
        }
        self.returns.push(
            WalletBytes::try_from(return_buf)
                .unwrap_or_else(|_| throw(WalletError::ReturnDataTooLarge)),
        );
    }
}

/// The auth tokens a batch holds, by where each came from.
struct BatchTokens {
    minted: Vec<u32>,
    /// Per completed call, the ids of the tokens it returned.
    returned: Vec<AuthHandleBuf>,
    /// Every id forwarded so far. A successful call takes the tokens it was
    /// forwarded and a failed one ends the batch, so none can go out twice.
    forwarded: Vec<u32>,
}

impl BatchTokens {
    /// Resolve `token` to the id of a token the wallet still holds, and mark
    /// it forwarded.
    fn take(&mut self, token: &WalletTokenRef) -> u32 {
        let index = token.index as usize;
        let handle = match token.source {
            WalletTokenSource::Minted if token.call == 0 => {
                self.minted.get(index)
            }
            WalletTokenSource::Minted => None,
            // Only completed calls have an entry, so a reference to the
            // current or a later call finds nothing.
            WalletTokenSource::Returned => self
                .returned
                .get(token.call as usize)
                .and_then(|returned| returned.get(index)),
        };
        let handle = *handle
            .unwrap_or_else(|| throw(WalletError::InvalidTokenRef));
        if self.forwarded.contains(&handle) {
            throw(WalletError::TokenAlreadyForwarded);
        }
        self.forwarded.push(handle);
        handle
    }
}

#[contract]
impl DefaultWallet for DefaultWalletContract {
    fn execute(calls: Vec<WalletCall>) -> Vec<WalletBytes> {
        require_root_pubkey_auth();
        require_call_count(calls.len());

        let mut returns = BatchReturns::new(calls.len());
        for call in calls.iter() {
            let return_buf = invoke(
                &call.target,
                call.entrypoint,
                call.gas_limit,
                call.calldata.as_slice(),
                &AuthHandleBuf::new(),
                &mut AuthHandleBuf::new(),
            );
            returns.push(&return_buf);
        }

        returns.returns
    }

    fn execute_with_tokens(
        mints: VecM<WalletBytes, MAX_MINTS>,
        calls: Vec<WalletTokenCall>,
    ) -> Vec<WalletBytes> {
        require_root_pubkey_auth();
        require_call_count(calls.len());

        // Minted before any call runs, so no callee's behavior can change
        // which token a `Minted` reference names. The tokens are forwarded
        // within this transaction, so they need not outlive the block.
        let expiry = thruput_sdk::block::get_block_height();
        let mut tokens = BatchTokens {
            minted: mints
                .iter()
                .map(|node| {
                    thruput_sdk::signer::mint_opaque_auth_token(
                        expiry.clone(),
                        node.as_slice(),
                    )
                    .handle()
                })
                .collect(),
            returned: Vec::with_capacity(calls.len()),
            forwarded: Vec::new(),
        };

        let mut returns = BatchReturns::new(calls.len());
        for call in calls.iter() {
            let mut auth_handles = AuthHandleBuf::new();
            for token in call.tokens.iter() {
                auth_handles.push(tokens.take(token));
            }
            let mut returned_auth_handles = AuthHandleBuf::new();
            let return_buf = invoke(
                &call.target,
                call.entrypoint,
                call.gas_limit,
                call.calldata.as_slice(),
                &auth_handles,
                &mut returned_auth_handles,
            );
            tokens.returned.push(returned_auth_handles);
            returns.push(&return_buf);
        }

        returns.returns
    }

    fn deploy(
        nonce: &SHA256Hash,
        contract_hash: &SHA256Hash,
        amount: i128,
        initializer: Option<WalletInitCall>,
    ) -> ContractAddress {
        require_root_pubkey_auth();
        if amount != 0 {
            let target = thruput_sdk::get_deploy_address(nonce, contract_hash);
            let token = ERC20Client::new(
                &thruput_sdk::types::genesis_addresses::ADDRESS_NATIVE_TOKEN,
            );
            match token.try_transfer(&target, amount) {
                Ok(()) => {}
                Err(CallError::Exception(exn)) => exn.re_throw(),
                Err(_) => throw(WalletError::CallFaulted),
            }
        }
        let deployed = thruput_sdk::deploy_contract(nonce, contract_hash);
        if let Some(init) = initializer {
            invoke(
                &deployed,
                init.entrypoint,
                init.gas_limit,
                init.calldata.as_slice(),
                &AuthHandleBuf::new(),
                &mut AuthHandleBuf::new(),
            );
        }
        deployed
    }
}
