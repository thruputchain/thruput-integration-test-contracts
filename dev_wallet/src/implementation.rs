use super::interface::*;
use thruput_erc20_if::ERC20Client;
use thruput_sdk::sys::{self, SyscallErrorCode};
use thruput_sdk::*;

pub struct DevWalletContract;

// Local-node deploy policy: hogd charges deployment storage to the deployed
// contract address, so the wallet funds that address before deploying it.
const DEPLOY_STORAGE_TOKENS: i128 = 100_000_000;

fn require_source_address_auth() {
    let auth_claim = thruput_sdk::get_auth_claim().unwrap();

    match auth_claim {
        AuthClaim::kSourceAddressAuthData {
            source_address_claim,
        } => {
            if source_address_claim.allowed_keys & 0x01 == 0
                || source_address_claim.required_weight != 1
            {
                trap();
            }
        }
        _ => {
            trap();
        }
    }
}

fn require_root_invocation() {
    let mut sender = ContractAddress::from([0u8; 32]);
    match sys::raw_GetMsgSender(&mut sender) {
        Err(e)
            if e == SyscallErrorCode::kSyscallErrorCodeIsRootTransaction
                .into() => {}
        _ => trap(),
    }
}

fn require_root_source_address_auth() {
    require_root_invocation();
    require_source_address_auth();
}

#[contract]
impl DevWallet for DevWalletContract {
    fn initialize(owner: &Ed25519PublicKey) {
        let pk = PublicKey::Ed25519 { ed25519: *owner };
        thruput_sdk::add_address_public_key(0, pk, 1).unwrap();
        thruput_sdk::set_address_required_weight(1).unwrap();
    }

    fn do_action(invocation_bytes: &[u8]) {
        require_root_source_address_auth();

        let (addr, entrypoint, gas_limit, calldata): (
            ContractAddress,
            ContractEntrypointID,
            u64,
            &[u8],
        ) = thruput_sdk::__xdrs::txs::decode(invocation_bytes).unwrap();

        let mut return_buf = thruput_sdk::call::ReturnBuf::new();
        let mut auth_handles = AuthHandleBuf::new();
        match thruput_sdk::call::contract_call(
            &addr,
            entrypoint,
            gas_limit,
            calldata,
            &AuthHandleBuf::new(),
            0,
            &mut return_buf,
            &mut auth_handles,
        ) {
            Ok(thruput_sdk::call::CallOutcome::Return) => thruput_sdk::internal::contract_return(
                return_buf.as_ref(),
                auth_handles.as_slice(),
            ),
            Ok(thruput_sdk::call::CallOutcome::Abort) => {
                thruput_sdk::contract_abort(return_buf.as_ref())
            }
            Err(_) => {
                trap();
            }
        }
    }

    fn deploy(
        nonce: &SHA256Hash,
        contract_hash: &SHA256Hash,
    ) -> ContractAddress {
        require_root_source_address_auth();
        let deployed_address = thruput_sdk::get_deploy_address(nonce, contract_hash);
        let token_client =
            ERC20Client::new(&thruput_sdk::types::genesis_addresses::ADDRESS_NATIVE_TOKEN);
        token_client.transfer(&deployed_address, DEPLOY_STORAGE_TOKENS);
        thruput_sdk::deploy_contract(nonce, contract_hash)
    }

    fn create_wallet(
        nonce: &SHA256Hash,
        wallet_contract_hash: &SHA256Hash,
        owner: &Ed25519PublicKey,
    ) -> ContractAddress {
        require_root_source_address_auth();
        let deployed_address =
            thruput_sdk::deploy_contract(nonce, wallet_contract_hash);
        Client::new(&deployed_address).initialize(owner);
        deployed_address
    }
}
