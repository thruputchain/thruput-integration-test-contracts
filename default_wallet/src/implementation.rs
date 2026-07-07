use super::interface::*;
use alloc::vec::Vec;
use sdk::sys::{self, SyscallErrorCode};
use sdk::*;

pub struct DefaultWalletContract;

const MAX_CALLS: usize = 16;
const MAX_RETURN_DATA_BYTES: usize = sdk::transaction::kMaxReturnBufferLen as usize;

fn padded_xdr_len(len: usize) -> Option<usize> {
    len.checked_add(3).map(|len| len & !3)
}

fn encoded_return_item_len(return_data_len: usize) -> Option<usize> {
    4usize.checked_add(padded_xdr_len(return_data_len)?)
}

fn require_root_invocation() {
    let mut sender = ContractAddress::from([0u8; 32]);
    match sys::raw_GetMsgSender(&mut sender) {
        Err(e) if e == SyscallErrorCode::kSyscallErrorCodeIsRootTransaction.into() => {}
        _ => throw(WalletError::NotRootInvocation),
    }
}

fn require_pubkey_signature_auth() {
    // The protocol admits `kPubkeySignature` transactions only for the
    // pubkey-derived account address in the claim preimage. The contract SDK
    // does not currently expose that derivation helper, so this contract checks
    // the claim type and relies on the admission invariant for address binding.
    match sdk::get_auth_claim() {
        Ok(AuthClaim::kPubkeySignature { .. }) => {}
        _ => throw(WalletError::InvalidAuthClaim),
    }
}

fn require_root_pubkey_auth() {
    require_root_invocation();
    require_pubkey_signature_auth();
}

#[contract]
impl DefaultWallet for DefaultWalletContract {
    fn execute(calls: Vec<WalletCall>) -> Vec<WalletBytes> {
        require_root_pubkey_auth();

        if calls.is_empty() {
            throw(WalletError::EmptyCallList);
        }
        if calls.len() > MAX_CALLS {
            throw(WalletError::TooManyCalls);
        }

        let mut returns = Vec::with_capacity(calls.len());
        let mut encoded_return_data_len = 4usize;
        for call in calls.iter() {
            match sdk::call::contract_call(
                &call.target,
                ContractEntrypointID::from(call.entrypoint.to_be_bytes()),
                call.gas_limit,
                call.calldata.as_slice(),
                &AuthHandleBuf::new(),
                0,
            ) {
                Ok(Ok((return_buf, _auth_handles))) => {
                    encoded_return_data_len = encoded_return_data_len
                        .checked_add(
                            encoded_return_item_len(return_buf.len())
                                .unwrap_or_else(|| throw(WalletError::ReturnDataTooLarge)),
                        )
                        .unwrap_or_else(|| throw(WalletError::ReturnDataTooLarge));
                    if encoded_return_data_len > MAX_RETURN_DATA_BYTES {
                        throw(WalletError::ReturnDataTooLarge);
                    }

                    returns.push(
                        WalletBytes::try_from(return_buf.as_slice())
                            .unwrap_or_else(|_| throw(WalletError::ReturnDataTooLarge)),
                    );
                }
                Ok(Err(sdk::call::ContractError::Abort(_))) => {
                    throw(WalletError::CallAborted);
                }
                Ok(Err(sdk::call::ContractError::Fault(_))) | Err(_) => {
                    throw(WalletError::CallFaulted);
                }
            }
        }

        returns
    }

    fn deploy(nonce: &SHA256Hash, contract_hash: &SHA256Hash) -> ContractAddress {
        require_root_pubkey_auth();
        sdk::deploy_contract(nonce, contract_hash)
    }
}
