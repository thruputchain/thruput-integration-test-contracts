use genesis_root::{GenesisRoot, GenesisRootContract};
use sdk::ContractAddress;

/// The well-known address the genesis ceremony installs the root contract
/// at. Kept as an independent literal (not imported from the crate) so a
/// drift in the contract's hardcoded constant fails these tests.
const GENESIS_ROOT_ADDRESS: ContractAddress = ContractAddress::new([
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 2,
]);

fn distribute_at(env: &sdk::TestEnvironmentHandle, address: &ContractAddress) -> sdk::TransactResult {
    env.transact(|tx| {
        tx.call(address).run(|| {
            <GenesisRootContract as GenesisRoot>::distribute(Vec::new(), 0, 0)
        })
    })
}

#[test]
fn distribute_rejects_wrong_current_address() {
    let env = sdk::TestEnvironment::no_cost();

    // Any address other than the well-known genesis root address must be
    // rejected before a single token is minted.
    let mut wrong_address_bytes = *GENESIS_ROOT_ADDRESS;
    wrong_address_bytes[0] ^= 0xFF;
    let wrong_address = ContractAddress::from(wrong_address_bytes);

    let result = distribute_at(&env, &wrong_address);
    result.assert_fault();
}

#[test]
fn distribute_rejects_block_height_above_one() {
    let env = sdk::TestEnvironment::no_cost();

    // The ceremony only runs while bootstrapping (height <= 1). Advance
    // past that window; the correct address must still be rejected.
    env.advance_block();
    env.advance_block();

    let result = distribute_at(&env, &GENESIS_ROOT_ADDRESS);
    result.assert_fault();
}
