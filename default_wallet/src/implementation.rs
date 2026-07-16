use super::interface::*;
use alloc::vec::Vec;
use sdk::sys::{self, SyscallErrorCode};
use sdk::*;

pub struct DefaultWalletContract;

const MAX_CALLS: usize = 16;
const MAX_RETURN_DATA_BYTES: usize = sdk::transaction::kMaxReturnBufferLen as usize;

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
        let mut encoded_return_data_len =
            cxs_u32_len(calls.len()).unwrap_or_else(|| throw(WalletError::ReturnDataTooLarge));
        for call in calls.iter() {
            let auth_handles = AuthHandleBuf::new();
            let mut return_buf = Vec::new();
            let mut returned_auth_handles = AuthHandleBuf::new();

            match sdk::call::contract_call(
                &call.target,
                ContractEntrypointID::from(call.entrypoint.to_be_bytes()),
                call.gas_limit,
                call.calldata.as_slice(),
                &auth_handles,
                0,
                &mut return_buf,
                &mut returned_auth_handles,
            ) {
                Ok(sdk::call::CallOutcome::Return) => {
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
                Ok(sdk::call::CallOutcome::Abort) => {
                    throw(WalletError::CallAborted);
                }
                Err(_) => {
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
