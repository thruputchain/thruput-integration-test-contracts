use super::interface::*;
use sdk::*;

#[derive(Default)]
pub struct SimpleWalletAuthClaimContract;

#[contract(storage = false)]
impl SimpleWalletAuthClaim for SimpleWalletAuthClaimContract {
    fn initialize(&mut self, owner: &Ed25519PublicKey) {
        let pk = PublicKey::Ed25519 {
            ed25519: *owner,
        };
        sdk::add_address_public_key(0, pk, 1).unwrap();
        sdk::set_address_required_weight(1).unwrap();
    }

    fn do_action(
        &self,
        invocation_bytes: &[u8],
    ) {
        let auth_claim = sdk::get_auth_claim().unwrap();

        match auth_claim {
            AuthClaim::kSourceAddressAuthData { source_address_claim } => {
                if source_address_claim.required_weight == 0 {
                    trap();
                }
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

        let method_id = u64::from_be_bytes(entrypoint.into_inner());

        sdk::sys::raw_Invoke(
            &addr,
            &calldata,
            &[],
            &sdk::sys::CopyableInvokeInputs {
                entrypoint_id: method_id,
                gas_limit,
                flags: 0, // TODO(geoff): invoke flags go here
            },
            &mut sdk::sys::CopyableInvokeOutputs {
                return_buffer_len: 0,
                returned_auth_token_len: 0,
            },
        )
        .unwrap();
    }
}
