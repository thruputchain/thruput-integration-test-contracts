//! Checks shared by the native unit tests (`src/tests.rs`) and the rv32
//! test (`tests/rv32.rs`), so the mock and the rv32 build are held to the
//! same assertions.

use thruput_sdk::*;
use thruput_sdk::xdr::cxs;

/// Deploys `contract_hash` and invokes `read_only_bytestring_set(true)` on
/// it with the `ReadOnly` flag: the callee expects its write to fail.
pub fn check_read_only_bytestring_set(
    contract_hash: &SHA256Hash,
) {
    let nonce = SHA256Hash::from([7; 32]);
    let deployed_addr = deploy_contract(&nonce, contract_hash);

    let mut outputs = core::mem::MaybeUninit::uninit();

    let flag = InvokeFlag::ReadOnly.0.cast_unsigned();
    let calldata = cxs::encode(&true).unwrap();

    let entrypoint_id =
        crate::ReadOnlyTestEntrypoints::read_only_bytestring_set.into_u64();

    sys::uninit_Invoke(
        &deployed_addr,
        &calldata,
        &[], // auth
        &sys::CopyableInvokeInputs {
            entrypoint_id,
            gas_limit: u64::MAX, // gas limit
            flags: flag,
            reserved: 0,
        },
        &mut outputs,
    )
    .unwrap();
}
