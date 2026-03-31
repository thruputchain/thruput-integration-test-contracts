//! An implementation of Fibonacci

use super::interface::*;
use sdk::sys::raw_ChangePublicKeyRegistration;
use sdk::*;
use sdk::NonceConfig;
use sdk::GasPermissionConfig;

#[derive(Default)]
pub struct SimpleWalletAuthClaimContract;

#[contract(exports = "guest_exports", storage = false)]
impl SimpleWalletAuthClaim for SimpleWalletAuthClaimContract {
    fn initialize(&mut self, owner: &Ed25519PublicKey) {
        let scheme = CryptographicScheme::Ed25519.0;
        let weight = 1u32;
        raw_ChangePublicKeyRegistration(0, Some(&scheme), owner.as_ref(), Some(&weight)).unwrap();

        sdk::set_nonce_config(NonceConfig{
            accepted_keys: 0x01,
            required_weight: 1,
        }).unwrap();

        sdk::set_gas_permission_config(GasPermissionConfig{
                    limit: 1000000000,
                    accepted_keys: 0x01,
                    required_weight: 1,
        }).unwrap();
    }

    fn do_action(
        &self,
        invocation_bytes: &[u8],
    ) {
        let auth_claim = sdk::get_auth_claim(0).unwrap();

        match auth_claim {
            AuthClaim::kInvokedAddressSignature{ invoked_address_signature_id } => {
                assert_eq!(invoked_address_signature_id, 0);
            },
            _ => {
                trap();
            }
        }
    
        let res = sdk::__xdrs::txs::decode(&invocation_bytes);
        let (addr, entrypoint, gas_limit, calldata): (
            ContractAddress,
            ContractEntrypointID,
            u64,
            &[u8],
        ) = res.unwrap();

        let mut return_len = 0;
        let method_id = u64::from_be_bytes(entrypoint.into_inner());

        sdk::sys::raw_Invoke(
            &addr,
            method_id,
            gas_limit,
            &calldata,
            0, // TODO(geoff): invoke flags go here
            &mut return_len,
        )
        .unwrap();
    }
}
