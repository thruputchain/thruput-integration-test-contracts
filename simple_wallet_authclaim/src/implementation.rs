use super::interface::*;
use sdk::transaction::InvocationFrameStatusCode;
use sdk::*;

pub struct SimpleWalletAuthClaimContract;

#[contract]
impl SimpleWalletAuthClaim for SimpleWalletAuthClaimContract {
    fn initialize(owner: &Ed25519PublicKey) {
        let pk = PublicKey::Ed25519 {
            ed25519: *owner,
        };
        sdk::add_address_public_key(0, pk, 1).unwrap();
        sdk::set_address_required_weight(1).unwrap();
    }

    fn do_action(
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

        let res = sdk::__xdrs::txs::decode(invocation_bytes);
        let (addr, entrypoint, gas_limit, calldata): (
            ContractAddress,
            ContractEntrypointID,
            u64,
            &[u8],
        ) = res.unwrap();

        let method_id = u64::from_be_bytes(entrypoint.into_inner());

        let mut outputs = sdk::sys::CopyableInvokeOutputs {
            return_buffer_len: 0,
            returned_auth_token_len: 0,
            // Placeholder; overwritten by the syscall on Ok(). Using Fault
            // (not Return) keeps the wallet fail-closed if the field is left
            // unwritten — the non-Return check below then traps.
            frame_status: InvocationFrameStatusCode::Fault,
        };
        sdk::sys::raw_Invoke(
            &addr,
            calldata,
            &[],
            &sdk::sys::CopyableInvokeInputs {
                entrypoint_id: method_id,
                gas_limit,
                flags: 0, // TODO(geoff): invoke flags go here
            },
            &mut outputs,
        )
        .unwrap();
        // A child abort/fault is no longer surfaced as a syscall error; keep
        // the wallet fail-closed by propagating it as a trap.
        if outputs.frame_status != InvocationFrameStatusCode::Return {
            trap();
        }
    }
}
