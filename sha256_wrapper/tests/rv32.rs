//! The checks in `src/checks.rs`, run against the rv32 build of
//! `sha256_wrapper` rather than the host mock.
//!
//! Requires the lowered `calldata_wrapper` artifact. See README for building it.

use sha256_wrapper::checks::check_sha256;
use thruput_sdk::*;

// Test working directory is the contract directory.
const RV32_PATH: &str = "../../../target/thruput/sha256_wrapper.rv32a";

static NONCE: SHA256Hash = sha256_static!("SHA256_WRAPPER_RV32_NONCE");

#[test]
fn sha256_wrapper_rv32() {
    let contract = load_rv32_contract(RV32_PATH);
    let env = TestEnvironment::no_cost();
    let contract_hash = env.setup_create_contract(&contract);

    env.transact(|tx| {
        tx.call(&ContractAddress::named("test")).run(move || {
            let addr = deploy_contract(&NONCE, &contract_hash);
            check_sha256(&sha256_wrapper::Client::new(&addr));
        })
    })
    .assert_success();
}
