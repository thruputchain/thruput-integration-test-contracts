//! The checks in `src/checks.rs`, run against the rv32 build of
//! `read_only_test` rather than the host mock.
//!
//! Requires the lowered `read_only_test` artifact: `scripts/dev.sh
//! build-integration-rv32` writes it.

use read_only_test::checks::check_read_only_bytestring_set;
use thruput_sdk::*;

// Test working directory is the contract directory.
const RV32_PATH: &str = "../../../target/thruput/read_only_test.rv32a";

#[test]
fn read_only_test_rv32() {
    let contract = load_rv32_contract(RV32_PATH);
    let env = TestEnvironment::no_cost();
    let contract_hash = env.setup_create_contract(&contract);

    env.transact(|tx| {
        tx.call(&ContractAddress::named("test")).run(move || {
            check_read_only_bytestring_set(&contract_hash);
        })
    })
    .assert_success();
}
