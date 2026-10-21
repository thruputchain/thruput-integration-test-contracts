//! The checks in `src/checks.rs`, run against the rv32 build of
//! `calldata_wrapper` rather than the host mock.
//!
//! Requires the lowered `calldata_wrapper` artifact: `scripts/dev.sh
//! build-integration-rv32` writes it.

use calldata_wrapper::checks::check_echo;
use thruput_sdk::*;

// Test working directory is the contract directory.
const RV32_PATH: &str = "../../../target/thruput/calldata_wrapper.rv32a";

static NONCE: SHA256Hash = sha256_static!("CALLDATA_WRAPPER_RV32_NONCE");

#[test]
fn calldata_wrapper_rv32() {
    let contract = load_rv32_contract(RV32_PATH);
    let env = TestEnvironment::no_cost();
    let contract_hash = env.setup_create_contract(&contract);

    env.transact(|tx| {
        tx.call(&ContractAddress::named("test")).run(move || {
            let addr = deploy_contract(&NONCE, &contract_hash);
            check_echo(&calldata_wrapper::Client::new(&addr));
        })
    })
    .assert_success();
}
