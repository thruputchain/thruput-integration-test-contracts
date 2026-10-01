use default_wallet::{
    DefaultWallet, DefaultWalletContract, WalletCall, WalletError,
};
use groundhog_domain_separator::sha2::pubkey_account_address;
use groundhog_xdr::auth_claim::PubkeyAccountPreimage;
use sdk::{AuthClaim, ContractAddress, Ed25519PublicKey, Exn, PublicKey};

fn pubkey_claim(pubkey_byte: u8, nonce: u64) -> AuthClaim {
    let pubkey = PublicKey::Ed25519 {
        ed25519: Ed25519PublicKey::from([pubkey_byte; 32]),
    };
    let pubkey_preimage = PubkeyAccountPreimage { pubkey, nonce };

    AuthClaim::kPubkeySignature { pubkey_preimage }
}

fn pubkey_claim_address(auth_claim: &AuthClaim) -> ContractAddress {
    match auth_claim {
        AuthClaim::kPubkeySignature { pubkey_preimage } => {
            pubkey_account_address(pubkey_preimage)
        }
        _ => panic!("expected pubkey signature claim"),
    }
}

fn assert_wallet_error(result: &sdk::TransactResult, expected: WalletError) {
    result.assert_abort();

    let mut reader = sdk::__xdrs::cxs::buf_reader::BufReader::new(
        result.frame().abort_buffer(),
    );
    let selector: u64 = sdk::ReadXDR::read_xdr(&mut reader).unwrap();
    assert_eq!(selector, WalletError::SELECTOR);

    let actual: WalletError = sdk::ReadXDR::read_xdr(&mut reader).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn execute_accepts_matching_pubkey_auth_claim() {
    let env = sdk::TestEnvironment::no_cost();
    let claim = pubkey_claim(17, 7);
    let wallet_address = pubkey_claim_address(&claim);

    let result = env.transaction().with_auth_claim(claim).transact(|tx| {
        tx.call(&wallet_address).run(|| {
            <DefaultWalletContract as DefaultWallet>::execute(Vec::new())
        })
    });

    assert_wallet_error(&result, WalletError::EmptyCallList);
}

#[test]
fn execute_rejects_pubkey_auth_claim_for_different_address() {
    let env = sdk::TestEnvironment::no_cost();
    let claim = pubkey_claim(17, 7);
    let mut wrong_address_bytes = *pubkey_claim_address(&claim);
    wrong_address_bytes[0] ^= 0xFF;
    let wrong_address = ContractAddress::from(wrong_address_bytes);

    let result = env.transaction().with_auth_claim(claim).transact(|tx| {
        tx.call(&wrong_address).run(|| {
            <DefaultWalletContract as DefaultWallet>::execute(Vec::new())
        })
    });

    assert_wallet_error(&result, WalletError::InvalidAuthClaim);
}

#[test]
fn execute_rejects_default_source_address_auth_claim() {
    let env = sdk::TestEnvironment::no_cost();

    let result = env.transact(|tx| {
        tx.call(&ContractAddress::new([1u8; 32])).run(|| {
            <DefaultWalletContract as DefaultWallet>::execute(Vec::new())
        })
    });

    assert_wallet_error(&result, WalletError::InvalidAuthClaim);
}

#[test]
fn deploy_rejects_wrong_auth_before_funding() {
    let env = sdk::TestEnvironment::no_cost();
    let claim = pubkey_claim(17, 7);
    let result = env.transaction().with_auth_claim(claim).transact(|tx| {
        tx.call(&ContractAddress::new([1u8; 32])).run(|| {
            <DefaultWalletContract as DefaultWallet>::deploy(
                &[0; 32].into(),
                &[0; 32].into(),
                100,
                None,
            )
        })
    });
    assert_wallet_error(&result, WalletError::InvalidAuthClaim);
}

#[test]
fn deploy_rejects_source_address_auth() {
    let env = sdk::TestEnvironment::no_cost();
    let result = env.transact(|tx| {
        tx.call(&ContractAddress::new([1u8; 32])).run(|| {
            <DefaultWalletContract as DefaultWallet>::deploy(
                &[0; 32].into(),
                &[0; 32].into(),
                100,
                None,
            )
        })
    });
    assert_wallet_error(&result, WalletError::InvalidAuthClaim);
}

#[test]
fn deploy_commits_exact_transfer_and_callable_contract() {
    // Zero storage costs isolate the requested transfer from collateral charges.
    // The hogd parity suite separately exercises the real cost model.
    let env = sdk::TestEnvironment::no_cost();
    let claim = pubkey_claim(17, 7);
    let wallet = pubkey_claim_address(&claim);
    env.setup_grant_storage_tokens(&wallet, 1_000);
    let marker = 42u64.to_be_bytes().into();
    let contract = env.register_mock(|contract| {
        contract.register_method(marker, || {
            sdk::sys::raw_Return(&[42], false, &[])
        });
    });
    let hash = *contract.as_hash();
    let nonce = [31; 32].into();
    let result = env.transaction().with_auth_claim(claim).transact(|tx| {
        tx.call(&wallet).run(|| {
            let expected = sdk::get_deploy_address(&nonce, &hash);
            let deployed = <DefaultWalletContract as DefaultWallet>::deploy(
                &nonce, &hash, 123, None,
            );
            assert_eq!(deployed, expected);
            deployed
        })
    });
    result.assert_success();
    let deployed = result.assert_return_as::<ContractAddress>();
    env.advance_block();

    env.transact(|tx| {
        tx.call(&wallet).run(|| {
            let token = groundhog_erc20_if::ERC20Client::new(
                &sdk::precompiles::ADDRESS_PRECOMPILED_HOG_TOKEN,
            );
            assert_eq!(token.get_balance_of(&wallet), 877);
            assert_eq!(token.get_balance_of(&deployed), 123);

            // Invoke the deployed code through the runtime, rather than merely
            // checking that deploy returned the predicted address.
            let mut returned = Vec::new();
            let mut returned_auth = sdk::AuthHandleBuf::new();
            let outcome = sdk::call::contract_call(
                &deployed,
                marker,
                u64::MAX,
                &[],
                &sdk::AuthHandleBuf::new(),
                0,
                &mut returned,
                &mut returned_auth,
            )
            .unwrap();
            assert_eq!(outcome, sdk::call::CallOutcome::Return);
            assert_eq!(returned, [42]);
        })
    })
    .assert_success();
}

fn init_call(mode: u8) -> default_wallet::WalletInitCall {
    default_wallet::WalletInitCall {
        entrypoint: 43u64.to_be_bytes().into(),
        gas_limit: u64::MAX,
        calldata: [mode].as_slice().try_into().unwrap(),
    }
}

fn initializer_body() {
    use sdk::storage::{Keypath, Value};
    let mut calldata = Vec::new();
    sdk::contract_op::fetch_calldata_and_auth_tokens(
        &mut calldata,
        &mut sdk::AuthHandleBuf::new(),
    );
    // Writing the caller proves both initialization and the caller identity.
    let mut owner: Value<ContractAddress> = Keypath::default().into();
    owner.set(&sdk::get_caller());
    match calldata.as_slice() {
        [0] => sdk::sys::raw_Return(&[0xff, 0x42], false, &[]),
        [1] => sdk::contract_abort(&[0xff]),
        [2] => panic!("initializer fault after write"),
        _ => panic!("unexpected initializer calldata"),
    }
}

#[test]
fn deploy_initializes_atomically_as_the_wallet_with_optional_funding() {
    use sdk::storage::{Keypath, Value};
    for amount in [0, 123] {
        let env = sdk::TestEnvironment::no_cost();
        let claim = pubkey_claim(17, 7);
        let wallet = pubkey_claim_address(&claim);
        env.setup_grant_storage_tokens(&wallet, 1_000);
        let contract = env.register_mock(|contract| {
            contract.register_method(init_call(0).entrypoint, initializer_body);
        });
        let hash = *contract.as_hash();
        let nonce = [32; 32].into();
        let result = env.transaction().with_auth_claim(claim).transact(|tx| {
            tx.call(&wallet).run(|| {
                let predicted = sdk::get_deploy_address(&nonce, &hash);
                let deployed = <DefaultWalletContract as DefaultWallet>::deploy(
                    &nonce,
                    &hash,
                    amount,
                    Some(init_call(0)),
                );
                assert_eq!(deployed, predicted);
                deployed
            })
        });
        result.assert_success();
        // The initializer's arbitrary return bytes do not replace the address.
        let deployed = result.assert_return_as::<ContractAddress>();
        env.advance_block();
        env.transact(|tx| {
            tx.call(&deployed).run(|| {
                let owner: Value<ContractAddress> = Keypath::default().into();
                assert_eq!(owner.get(), wallet);
                let token = groundhog_erc20_if::ERC20Client::new(
                    &sdk::precompiles::ADDRESS_PRECOMPILED_HOG_TOKEN,
                );
                assert_eq!(token.get_balance_of(&wallet), 1_000 - amount);
                assert_eq!(token.get_balance_of(&deployed), amount);
            })
        })
        .assert_success();
    }
}

#[test]
fn initializer_abort_and_fault_roll_back_funding_deployment_and_storage() {
    use sdk::storage::{Keypath, Value};
    use sdk::NoneIfMissing;
    for (mode, expected) in
        [(1, WalletError::CallAborted), (2, WalletError::CallFaulted)]
    {
        let env = sdk::TestEnvironment::no_cost();
        let claim = pubkey_claim(17, 7);
        let wallet = pubkey_claim_address(&claim);
        env.setup_grant_storage_tokens(&wallet, 1_000);
        let contract = env.register_mock(|contract| {
            contract.register_method(init_call(0).entrypoint, initializer_body);
        });
        let hash = *contract.as_hash();
        let nonce = [33; 32].into();
        let address_result = env.transact(|tx| {
            tx.call(&wallet)
                .run(|| sdk::get_deploy_address(&nonce, &hash))
        });
        address_result.assert_success();
        let target = address_result.assert_return_as::<ContractAddress>();
        let result =
            env.transaction()
                .with_auth_claim(claim.clone())
                .transact(|tx| {
                    tx.call(&wallet).run(|| {
                        <DefaultWalletContract as DefaultWallet>::deploy(
                            &nonce,
                            &hash,
                            123,
                            Some(init_call(mode)),
                        )
                    })
                });
        assert_wallet_error(&result, expected);
        env.advance_block();
        env.transact(|tx| {
            tx.call(&target).run(|| {
                let owner: Value<ContractAddress> = Keypath::default().into();
                assert_eq!(owner.try_get().none_if_missing(), None);
                let token = groundhog_erc20_if::ERC20Client::new(
                    &sdk::precompiles::ADDRESS_PRECOMPILED_HOG_TOKEN,
                );
                assert_eq!(token.get_balance_of(&wallet), 1_000);
                assert_eq!(token.get_balance_of(&target), 0);
            })
        })
        .assert_success();
        // Reusing the exact deployment identity succeeds only if code rolled back.
        env.transaction()
            .with_auth_claim(claim)
            .transact(|tx| {
                tx.call(&wallet).run(|| {
                    <DefaultWalletContract as DefaultWallet>::deploy(
                        &nonce,
                        &hash,
                        123,
                        Some(init_call(0)),
                    )
                })
            })
            .assert_success();
    }
}

#[sdk::contract_exception]
#[derive(Copy, Clone)]
pub enum AdderError {
    Overflow = 1,
}

/// A minimal contract for the wallet to call.
#[sdk::contract(throws = AdderError)]
pub trait Adder {
    fn add(a: u32, b: u32) -> u32;
}

pub struct AdderContract;

#[sdk::contract]
impl Adder for AdderContract {
    fn add(a: u32, b: u32) -> u32 {
        use sdk::OrThrow;
        a.checked_add(b).or_throw(AdderError::Overflow)
    }
}

fn add_call(target: &ContractAddress, a: u32, b: u32) -> WalletCall {
    WalletCall {
        target: *target,
        entrypoint: AdderEntrypoints::add.into(),
        gas_limit: u64::MAX,
        // Calldata is the CXS encoding of the argument tuple, as a generated
        // client sends it.
        calldata: sdk::encode_xdr(&(a, b))
            .unwrap()
            .as_slice()
            .try_into()
            .unwrap(),
    }
}

#[test]
fn execute_calls_deployed_contract_and_returns_each_result() {
    const ADDER: ContractAddress = ContractAddress::new([0xAD; 32]);
    let env = sdk::TestEnvironment::no_cost();
    let adder = env.register_mock(|contract| {
        <AdderContract as Adder>::__mock(contract);
    });
    adder.deploy_contract_to(&ADDER);

    let claim = pubkey_claim(17, 7);
    let wallet = pubkey_claim_address(&claim);
    let result = env.transaction().with_auth_claim(claim).transact(|tx| {
        tx.call(&wallet).run(|| {
            <DefaultWalletContract as DefaultWallet>::execute(vec![
                add_call(&ADDER, 2, 3),
                add_call(&ADDER, 40, 2),
            ])
        })
    });

    result.assert_success();
    let returns = result.assert_return_as::<Vec<default_wallet::WalletBytes>>();
    let returns: Vec<&[u8]> = returns.iter().map(|r| r.as_slice()).collect();
    let expected = [
        sdk::encode_xdr(&5u32).unwrap(),
        sdk::encode_xdr(&42u32).unwrap(),
    ];
    assert_eq!(returns, [expected[0].as_slice(), expected[1].as_slice()]);
}

#[test]
fn execute_rethrows_callee_error() {
    const ADDER: ContractAddress = ContractAddress::new([0xAD; 32]);
    let env = sdk::TestEnvironment::no_cost();
    let adder = env.register_mock(|contract| {
        <AdderContract as Adder>::__mock(contract);
    });
    adder.deploy_contract_to(&ADDER);

    // The callee's typed error reaches the wallet's caller unchanged, even
    // after an earlier call in the batch succeeded.
    let claim = pubkey_claim(17, 7);
    let wallet = pubkey_claim_address(&claim);
    let result = env.transaction().with_auth_claim(claim).transact(|tx| {
        tx.call(&wallet).run(|| {
            <DefaultWalletContract as DefaultWallet>::execute(vec![
                add_call(&ADDER, 2, 3),
                add_call(&ADDER, u32::MAX, 1),
            ])
        })
    });

    assert_eq!(result.assert_abort_as::<AdderError>(), AdderError::Overflow);
}
