static FIBONACCI_NONCE: sdk::SHA256Hash = sdk::sha256_static!("FIBONACCI_TEST_NONCE");
static CALLER_NONCE: sdk::SHA256Hash = sdk::sha256_static!("FIBONACCI_CALLER_TEST_NONCE");

fn setup_env() -> (sdk::TestEnvironmentHandle, sdk::SHA256Hash, sdk::SHA256Hash) {
    let env = sdk::TestEnvironment::no_cost();

    let fibonacci_contract = env.register_mock(<fibonacci::FibonacciContract as fibonacci::Fibonacci>::__register_mock);
    let fibonacci_hash = env.setup_create_contract(&fibonacci_contract);

    let caller_contract =
        env.register_mock(<fibonacci_call::FibonacciContractCaller as fibonacci_call::FibonacciCaller>::__register_mock);
    let caller_hash = env.setup_create_contract(&caller_contract);

    (env, fibonacci_hash, caller_hash)
}

/// try_call with an inner out-of-gas should return Err(CallError::Fault(..))
/// instead of aborting the caller.
#[test]
fn try_call_inner_out_of_gas() {
    let (env, fibonacci_hash, caller_hash) = setup_env();

    sdk::contract_exec(env, move || {
        let fibonacci_addr =
            sdk::deploy_contract(&FIBONACCI_NONCE, &fibonacci_hash);
        let caller_addr =
            sdk::deploy_contract(&CALLER_NONCE, &caller_hash);

        let mut client = fibonacci_call::Client::new(&caller_addr);
        client.initialize(&fibonacci_addr);

        // burn_gas loops on sha256 and will exhaust the gas budget.
        // try_call should return Err(CallError::Fault(..)).
        client.call_burn_gas_expect_out_of_gas(1000);
    })
    .unwrap_or_else(|e| panic!("Test failed: {}", e));
}

/// The non-try call variant aborts the caller on inner out-of-gas.
/// This must produce a clean abort (not SIGABRT/__stack_chk_fail).
#[test]
fn call_inner_out_of_gas_aborts_cleanly() {
    let (env, fibonacci_hash, caller_hash) = setup_env();

    let result: sdk::call::CallResult<()> =
        sdk::contract_test(env, move || {
            let fibonacci_addr =
                sdk::deploy_contract(&FIBONACCI_NONCE, &fibonacci_hash);
            let caller_addr =
                sdk::deploy_contract(&CALLER_NONCE, &caller_hash);

            let mut client = fibonacci_call::Client::new(&caller_addr);
            client.initialize(&fibonacci_addr);

            // call (not try_call) will abort the caller on inner OOG.
            client.call_burn_gas(1000);
        });

    // The caller should have aborted, not crashed.
    assert!(
        result.is_err(),
        "Expected the caller to abort, got: {:?}",
        result
    );
}
