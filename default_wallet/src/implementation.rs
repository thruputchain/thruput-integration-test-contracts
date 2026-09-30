use super::interface::*;
use alloc::vec::Vec;
use groundhog_domain_separator::syscall::pubkey_account_address;
use groundhog_erc20_if::ERC20Client;
use sdk::sys::{self, SyscallErrorCode};
use sdk::*;

pub struct DefaultWalletContract;

const MAX_CALLS: usize = 16;
const MAX_RETURN_DATA_BYTES: usize =
    sdk::transaction::kMaxReturnBufferLen as usize;

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

fn require_pubkey_signature_auth() {
    match sdk::get_auth_claim() {
        Ok(AuthClaim::kPubkeySignature { pubkey_preimage })
            if pubkey_account_address(&pubkey_preimage)
                == sdk::get_current_address() => {}
        _ => throw(WalletError::InvalidAuthClaim),
    }
}

fn require_root_pubkey_auth() {
    require_root_invocation();
    require_pubkey_signature_auth();
}

fn invoke(
    target: &ContractAddress,
    entrypoint: ContractEntrypointID,
    gas_limit: u64,
    calldata: &[u8],
) -> Vec<u8> {
    let mut return_buf = Vec::new();
    let mut returned_auth_handles = AuthHandleBuf::new();
    match sdk::call::contract_call(
        target,
        entrypoint,
        gas_limit,
        calldata,
        &AuthHandleBuf::new(),
        0,
        &mut return_buf,
        &mut returned_auth_handles,
    ) {
        Ok(sdk::call::CallOutcome::Return) => return_buf,
        Ok(sdk::call::CallOutcome::Abort) => throw(WalletError::CallAborted),
        Err(_) => throw(WalletError::CallFaulted),
    }
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
        let mut encoded_return_data_len = cxs_u32_len(calls.len())
            .unwrap_or_else(|| throw(WalletError::ReturnDataTooLarge));
        for call in calls.iter() {
            let return_buf = invoke(
                &call.target,
                call.entrypoint,
                call.gas_limit,
                call.calldata.as_slice(),
            );
            encoded_return_data_len = encoded_return_data_len
                .checked_add(
                    encoded_return_item_len(return_buf.len()).unwrap_or_else(
                        || throw(WalletError::ReturnDataTooLarge),
                    ),
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

        returns
    }

    fn deploy(
        nonce: &SHA256Hash,
        contract_hash: &SHA256Hash,
        amount: i128,
        initializer: Option<WalletInitCall>,
    ) -> ContractAddress {
        require_root_pubkey_auth();
        if amount != 0 {
            let target = sdk::get_deploy_address(nonce, contract_hash);
            let token = ERC20Client::new(
                &sdk::precompiles::ADDRESS_PRECOMPILED_HOG_TOKEN,
            );
            match token.try_transfer(&target, amount) {
                Ok(()) => {}
                Err(CallError::Exception(_)) => throw(WalletError::CallAborted),
                Err(_) => throw(WalletError::CallFaulted),
            }
        }
        let deployed = sdk::deploy_contract(nonce, contract_hash);
        if let Some(init) = initializer {
            invoke(
                &deployed,
                init.entrypoint,
                init.gas_limit,
                init.calldata.as_slice(),
            );
        }
        deployed
    }
}
