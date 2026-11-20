//! The checks in `src/checks.rs`, run against the rv32 build of
//! `fibonacci` rather than the host mock.
//!
//! Requires the lowered `calldata_wrapper` artifact. See README for building it.

use fibonacci::checks::check_fibonacci;
use thruput_sdk::*;

// Test working directory is the contract directory.
const RV32_PATH: &str = "../../../target/thruput/fibonacci.rv32a";

static NONCE: SHA256Hash = sha256_static!("FIBONACCI_RV32_NONCE");

#[test]
fn fibonacci_rv32() {
    let contract = load_rv32_contract(RV32_PATH);
    let env = TestEnvironment::no_cost();
    let contract_hash = env.setup_create_contract(&contract);

    env.transact(|tx| {
        tx.call(&ContractAddress::named("test")).run(move || {
            let addr = deploy_contract(&NONCE, &contract_hash);
            check_fibonacci(&fibonacci::Client::new(&addr));
        })
    })
    .assert_success();
}
