use super::interface::*;
use groundhog_erc20_if::ERC20Client;
use sdk::sys::{self, SyscallErrorCode};
use sdk::*;

#[storage]
pub struct DevWalletContract {
    owner: sdk::storage::Value<Ed25519PublicKey>,
}

// Local-node deploy policy: hogd charges deployment storage to the deployed
// contract address, so the wallet funds that address before deploying it.
const DEPLOY_STORAGE_TOKENS: u128 = 100_000_000;

fn require_source_address_auth() {
    let auth_claim = sdk::get_auth_claim().unwrap();

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
    fn initialize(&mut self, owner: &Ed25519PublicKey) {
        // One-shot: this entrypoint is unauthenticated, and
        // add_address_public_key overwrites key 0, so a re-call would hand
        // ownership of the wallet to the caller. Refuse once an owner is set.
        if self.owner.try_get().none_if_missing().is_some() {
            trap();
        }
        self.owner.set(owner);
        let pk = PublicKey::Ed25519 { ed25519: *owner };
        sdk::add_address_public_key(0, pk, 1).unwrap();
        sdk::set_address_required_weight(1).unwrap();
    }

    fn do_action(invocation_bytes: &[u8]) {
        require_root_source_address_auth();

        let (addr, entrypoint, gas_limit, calldata): (
            ContractAddress,
            ContractEntrypointID,
            u64,
            &[u8],
        ) = sdk::__xdrs::txs::decode(invocation_bytes).unwrap();

        let mut return_buf = sdk::call::ReturnBuf::new();
        let mut auth_handles = AuthHandleBuf::new();
        match sdk::call::contract_call(
            &addr,
            entrypoint,
            gas_limit,
            calldata,
            &AuthHandleBuf::new(),
            0,
            &mut return_buf,
            &mut auth_handles,
        ) {
            Ok(sdk::call::CallOutcome::Return) => sdk::internal::contract_return(
                return_buf.as_ref(),
                auth_handles.as_slice(),
            ),
            Ok(sdk::call::CallOutcome::Abort) => {
                sdk::contract_abort(return_buf.as_ref())
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
        let deployed_address = sdk::get_deploy_address(nonce, contract_hash);
        let token_client =
            ERC20Client::new(&sdk::precompiles::ADDRESS_PRECOMPILED_HOG_TOKEN);
        token_client.transfer(&deployed_address, DEPLOY_STORAGE_TOKENS);
        sdk::deploy_contract(nonce, contract_hash)
    }

    fn create_wallet(
        nonce: &SHA256Hash,
        wallet_contract_hash: &SHA256Hash,
        owner: &Ed25519PublicKey,
    ) -> ContractAddress {
        require_root_source_address_auth();
        let deployed_address =
            sdk::deploy_contract(nonce, wallet_contract_hash);
        Client::new(&deployed_address).initialize(owner);
        deployed_address
    }
}
