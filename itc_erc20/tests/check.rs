use itc_erc20 as erc20;

#[test]
fn run_test() {
    let env = sdk::TestEnvironment::no_cost();
    let test_contract = env.register_mock(<erc20::Token as erc20::ERC20>::__register_mock);

    erc20_native_test::test_erc20_contract(env, &test_contract);
}
