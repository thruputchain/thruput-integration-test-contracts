use itc_erc20 as erc20;

#[test]
fn run_test() {
    let mut env = sdk::TestEnvironment::new();
    let test_contract = erc20::test_setup::register(&mut env);

    erc20_native_test::test_erc20_contract(&env, &test_contract);
}
