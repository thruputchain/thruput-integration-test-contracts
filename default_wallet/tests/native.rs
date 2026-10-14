use default_wallet::{
    DefaultWallet, DefaultWalletContract, WalletCall, WalletError,
};
use thruput_domain_separator::sha2::pubkey_account_address;
use thruput_sdk::{
    AuthClaim, ContractAddress, Ed25519PublicKey, Exn, PublicKey,
};
use thruput_xdr::auth_claim::PubkeyAccountPreimage;
use thruput_xdr::types::{MlDsa65PublicKey, PublicKeyHash};

fn ed25519_key(byte: u8) -> PublicKey {
    PublicKey::Ed25519 {
        ed25519: Ed25519PublicKey::from([byte; 32]),
    }
}

fn mldsa65_key(byte: u8) -> PublicKey {
    PublicKey::MlDsa65 {
        mldsa65: MlDsa65PublicKey::from([byte; 1952]),
    }
}

/// `sha256(cxs(DomainSeparator("PublicKey")) || cxs(PublicKey))`, spelled out
/// from the protocol definition rather than through an SDK helper, so these
/// tests also pin the hash the claim must carry.
fn public_key_hash(pubkey: &PublicKey) -> PublicKeyHash {
    use sha2::Digest;
    let separator = thruput_xdr::preimage::DomainSeparator {
        ext: thruput_xdr::types::ExtensionPoint::_0,
        domain_separator: b"PublicKey".as_slice().try_into().unwrap(),
    };
    let mut hasher = sha2::Sha256::new();
    hasher.update(thruput_sdk::__xdrs::cxs::encode(&separator).unwrap());
    hasher.update(thruput_sdk::__xdrs::cxs::encode(pubkey).unwrap());
    PublicKeyHash::from(<[u8; 32]>::from(hasher.finalize()))
}

fn claim_for_key(pubkey: &PublicKey, nonce: u64) -> AuthClaim {
    let pubkey_preimage = PubkeyAccountPreimage {
        pubkey_hash: public_key_hash(pubkey),
        nonce,
    };

    AuthClaim::kPubkeySignature { pubkey_preimage }
}

fn pubkey_claim(pubkey_byte: u8, nonce: u64) -> AuthClaim {
    claim_for_key(&ed25519_key(pubkey_byte), nonce)
}

/// One claim per supported signature scheme, for tests that must hold for
/// every pubkey account whatever its key.
fn claims_for_each_scheme() -> [(&'static str, AuthClaim); 2] {
    [
        ("Ed25519", claim_for_key(&ed25519_key(17), 7)),
        ("ML-DSA-65", claim_for_key(&mldsa65_key(17), 7)),
    ]
}

fn pubkey_claim_address(auth_claim: &AuthClaim) -> ContractAddress {
    match auth_claim {
        AuthClaim::kPubkeySignature { pubkey_preimage } => {
            pubkey_account_address(pubkey_preimage)
        }
        _ => panic!("expected pubkey signature claim"),
    }
}

fn assert_wallet_error(result: &thruput_sdk::TransactResult, expected: WalletError) {
    result.assert_abort();

    let mut reader = thruput_sdk::__xdrs::cxs::buf_reader::BufReader::new(
        result.frame().abort_buffer(),
    );
    let selector: u64 = thruput_sdk::ReadXDR::read_xdr(&mut reader).unwrap();
    assert_eq!(selector, WalletError::SELECTOR);

    let actual: WalletError = thruput_sdk::ReadXDR::read_xdr(&mut reader).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn execute_accepts_matching_pubkey_auth_claim() {
    let env = thruput_sdk::TestEnvironment::no_cost();
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
    let env = thruput_sdk::TestEnvironment::no_cost();
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
    let env = thruput_sdk::TestEnvironment::no_cost();

    let result = env.transact(|tx| {
        tx.call(&ContractAddress::new([1u8; 32])).run(|| {
            <DefaultWalletContract as DefaultWallet>::execute(Vec::new())
        })
    });

    assert_wallet_error(&result, WalletError::InvalidAuthClaim);
}

#[test]
fn deploy_rejects_wrong_auth_before_funding() {
    let env = thruput_sdk::TestEnvironment::no_cost();
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
    let env = thruput_sdk::TestEnvironment::no_cost();
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
    let env = thruput_sdk::TestEnvironment::no_cost();
    let claim = pubkey_claim(17, 7);
    let wallet = pubkey_claim_address(&claim);
    env.setup_grant_storage_tokens(&wallet, 1_000);
    let marker = 42u64.to_be_bytes().into();
    let contract = env.register_mock(|contract| {
        contract.register_method(marker, || {
            thruput_sdk::sys::raw_Return(&[42], false, &[])
        });
    });
    let hash = *contract.as_hash();
    let nonce = [31; 32].into();
    let result = env.transaction().with_auth_claim(claim).transact(|tx| {
        tx.call(&wallet).run(|| {
            let expected = thruput_sdk::get_deploy_address(&nonce, &hash);
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
            let token = thruput_erc20_if::ERC20Client::new(
                &thruput_sdk::precompiles::ADDRESS_PRECOMPILED_HOG_TOKEN,
            );
            assert_eq!(token.get_balance_of(&wallet), 877);
            assert_eq!(token.get_balance_of(&deployed), 123);

            // Invoke the deployed code through the runtime, rather than merely
            // checking that deploy returned the predicted address.
            let mut returned = Vec::new();
            let mut returned_auth = thruput_sdk::AuthHandleBuf::new();
            let outcome = thruput_sdk::call::contract_call(
                &deployed,
                marker,
                u64::MAX,
                &[],
                &thruput_sdk::AuthHandleBuf::new(),
                0,
                &mut returned,
                &mut returned_auth,
            )
            .unwrap();
            assert_eq!(outcome, thruput_sdk::call::CallOutcome::Return);
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
    use thruput_sdk::storage::{Keypath, Value};
    let mut calldata = Vec::new();
    thruput_sdk::contract_op::fetch_calldata_and_auth_tokens(&mut calldata);
    // Writing the caller proves both initialization and the caller identity.
    let mut owner: Value<ContractAddress> = Keypath::default().into();
    owner.set(&thruput_sdk::get_caller());
    match calldata.as_slice() {
        [0] => thruput_sdk::sys::raw_Return(&[0xff, 0x42], false, &[]),
        [1] => thruput_sdk::contract_abort(&[0xff]),
        [2] => panic!("initializer fault after write"),
        _ => panic!("unexpected initializer calldata"),
    }
}

#[test]
fn deploy_initializes_atomically_as_the_wallet_with_optional_funding() {
    use thruput_sdk::storage::{Keypath, Value};
    for amount in [0, 123] {
        let env = thruput_sdk::TestEnvironment::no_cost();
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
                let predicted = thruput_sdk::get_deploy_address(&nonce, &hash);
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
                let token = thruput_erc20_if::ERC20Client::new(
                    &thruput_sdk::precompiles::ADDRESS_PRECOMPILED_HOG_TOKEN,
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
    use thruput_sdk::storage::{Keypath, Value};
    use thruput_sdk::NoneIfMissing;
    for (mode, expected) in
        [(1, WalletError::CallAborted), (2, WalletError::CallFaulted)]
    {
        let env = thruput_sdk::TestEnvironment::no_cost();
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
                .run(|| thruput_sdk::get_deploy_address(&nonce, &hash))
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
                let token = thruput_erc20_if::ERC20Client::new(
                    &thruput_sdk::precompiles::ADDRESS_PRECOMPILED_HOG_TOKEN,
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

#[thruput_sdk::contract_exception]
#[derive(Copy, Clone)]
pub enum AdderError {
    Overflow = 1,
}

/// A minimal contract for the wallet to call.
#[thruput_sdk::contract(throws = AdderError)]
pub trait Adder {
    fn add(a: u32, b: u32) -> u32;
    fn store_sum(a: u32, b: u32) -> u32;
}

pub struct AdderContract;

#[thruput_sdk::contract]
impl Adder for AdderContract {
    fn add(a: u32, b: u32) -> u32 {
        use thruput_sdk::OrThrow;
        a.checked_add(b).or_throw(AdderError::Overflow)
    }

    fn store_sum(a: u32, b: u32) -> u32 {
        let sum = Self::add(a, b);
        stored_sum().set(&sum);
        sum
    }
}

fn stored_sum() -> thruput_sdk::storage::Value<u32> {
    thruput_sdk::storage::Keypath::default().into()
}

fn adder_call(
    target: &ContractAddress,
    entrypoint: AdderEntrypoints,
    a: u32,
    b: u32,
) -> WalletCall {
    WalletCall {
        target: *target,
        entrypoint: entrypoint.into(),
        gas_limit: u64::MAX,
        calldata: thruput_sdk::encode_xdr(&(a, b))
            .unwrap()
            .as_slice()
            .try_into()
            .unwrap(),
    }
}

#[test]
fn execute_calls_deployed_contract_and_returns_each_result() {
    const ADDER: ContractAddress = ContractAddress::new([0xAD; 32]);
    let env = thruput_sdk::TestEnvironment::no_cost();
    let adder = env.register_mock(|contract| {
        <AdderContract as Adder>::__mock(contract);
    });
    adder.deploy_contract_to(&ADDER);

    let claim = pubkey_claim(17, 7);
    let wallet = pubkey_claim_address(&claim);
    let result = env.transaction().with_auth_claim(claim).transact(|tx| {
        tx.call(&wallet).run(|| {
            <DefaultWalletContract as DefaultWallet>::execute(vec![
                adder_call(&ADDER, AdderEntrypoints::add, 2, 3),
                adder_call(&ADDER, AdderEntrypoints::add, 40, 2),
            ])
        })
    });

    result.assert_success();
    let returns = result.assert_return_as::<Vec<default_wallet::WalletBytes>>();
    let returns: Vec<&[u8]> = returns.iter().map(|r| r.as_slice()).collect();
    let expected = [
        thruput_sdk::encode_xdr(&5u32).unwrap(),
        thruput_sdk::encode_xdr(&42u32).unwrap(),
    ];
    assert_eq!(returns, [expected[0].as_slice(), expected[1].as_slice()]);
}

#[test]
fn execute_rethrows_callee_error() {
    const ADDER: ContractAddress = ContractAddress::new([0xAD; 32]);
    let env = thruput_sdk::TestEnvironment::no_cost();
    let adder = env.register_mock(|contract| {
        <AdderContract as Adder>::__mock(contract);
    });
    adder.deploy_contract_to(&ADDER);

    let claim = pubkey_claim(17, 7);
    let wallet = pubkey_claim_address(&claim);
    let execute = |calls: Vec<WalletCall>| {
        env.transaction()
            .with_auth_claim(claim.clone())
            .transact(|tx| {
                tx.call(&wallet).run(|| {
                    <DefaultWalletContract as DefaultWallet>::execute(calls)
                })
            })
    };

    // A committed write, so the rollback check below cannot pass vacuously.
    execute(vec![adder_call(&ADDER, AdderEntrypoints::store_sum, 2, 3)])
        .assert_success();
    env.advance_block();

    // The callee's typed error reaches the wallet's caller unchanged, and the
    // earlier call's write in the same batch is rolled back with it.
    let result = execute(vec![
        adder_call(&ADDER, AdderEntrypoints::store_sum, 40, 2),
        adder_call(&ADDER, AdderEntrypoints::add, u32::MAX, 1),
    ]);
    assert_eq!(result.assert_abort_as::<AdderError>(), AdderError::Overflow);
    env.advance_block();

    env.transact(|tx| {
        tx.call(&ADDER).run(|| assert_eq!(stored_sum().get(), 5))
    })
    .assert_success();
}

#[test]
fn deploy_rethrows_funding_error() {
    let env = thruput_sdk::TestEnvironment::no_cost();
    let claim = pubkey_claim(17, 7);
    let wallet = pubkey_claim_address(&claim);
    env.setup_grant_storage_tokens(&wallet, 1_000);
    let contract = env.register_mock(|_| {});
    let hash = *contract.as_hash();
    let nonce = [34; 32].into();
    let result = env.transaction().with_auth_claim(claim).transact(|tx| {
        tx.call(&wallet).run(|| {
            <DefaultWalletContract as DefaultWallet>::deploy(
                &nonce, &hash, -5, None,
            )
        })
    });
    assert_eq!(
        result.assert_abort_as::<thruput_erc20_if::ERC20Error>(),
        thruput_erc20_if::ERC20Error::NegativeAmount
    );
}

#[test]
fn deploy_rethrows_initializer_error() {
    let env = thruput_sdk::TestEnvironment::no_cost();
    let claim = pubkey_claim(17, 7);
    let wallet = pubkey_claim_address(&claim);
    env.setup_grant_storage_tokens(&wallet, 1_000);
    let contract = env.register_mock(|contract| {
        <AdderContract as Adder>::__mock(contract);
    });
    let hash = *contract.as_hash();
    let nonce = [35; 32].into();
    let deploy = |a: u32, b: u32| {
        // The initializer targets the new contract, so the call's own
        // target is unused; only its entrypoint and calldata carry over.
        let call = adder_call(
            &ContractAddress::new([0; 32]),
            AdderEntrypoints::store_sum,
            a,
            b,
        );
        let init = default_wallet::WalletInitCall {
            entrypoint: call.entrypoint,
            gas_limit: call.gas_limit,
            calldata: call.calldata,
        };
        env.transaction()
            .with_auth_claim(claim.clone())
            .transact(|tx| {
                tx.call(&wallet).run(|| {
                    <DefaultWalletContract as DefaultWallet>::deploy(
                        &nonce,
                        &hash,
                        123,
                        Some(init),
                    )
                })
            })
    };

    // The initializer's typed error reaches the wallet's caller unchanged.
    let result = deploy(u32::MAX, 1);
    assert_eq!(result.assert_abort_as::<AdderError>(), AdderError::Overflow);
    env.advance_block();

    // Funding rolled back with it.
    env.transact(|tx| {
        tx.call(&wallet).run(|| {
            let token = thruput_erc20_if::ERC20Client::new(
                &thruput_sdk::precompiles::ADDRESS_PRECOMPILED_HOG_TOKEN,
            );
            assert_eq!(token.get_balance_of(&wallet), 1_000);
        })
    })
    .assert_success();

    // So did deployment: the same identity deploys, and initializes, again.
    let result = deploy(2, 3);
    result.assert_success();
    let deployed = result.assert_return_as::<ContractAddress>();
    env.advance_block();
    env.transact(|tx| {
        tx.call(&deployed).run(|| assert_eq!(stored_sum().get(), 5))
    })
    .assert_success();
}

// --- Pubkey accounts of every scheme ----------------------------------------
//
// The claim names the account's key by its `PublicKeyHash`, so the wallet
// re-derives its address from the claim alone, whatever the scheme. The tests
// below hold for Ed25519 and ML-DSA-65 accounts alike.

#[test]
fn contract_facing_auth_claim_is_48_bytes() {
    // Tag, 4 bytes of padding, then the 40-byte preimage: the same for every
    // scheme, since the preimage carries a key hash rather than the key.
    assert_eq!(core::mem::size_of::<AuthClaim>(), 48);
    assert_eq!(core::mem::size_of::<PubkeyAccountPreimage>(), 40);
}

#[test]
fn pubkey_account_address_commits_to_scheme_key_and_nonce() {
    let ed25519 = pubkey_claim_address(&claim_for_key(&ed25519_key(17), 7));
    let mldsa65 = pubkey_claim_address(&claim_for_key(&mldsa65_key(17), 7));
    let other_key = pubkey_claim_address(&claim_for_key(&ed25519_key(18), 7));
    let other_nonce =
        pubkey_claim_address(&claim_for_key(&ed25519_key(17), 8));
    let addresses = [ed25519, mldsa65, other_key, other_nonce];
    for (i, a) in addresses.iter().enumerate() {
        for b in &addresses[i + 1..] {
            assert_ne!(a, b);
        }
    }

    // A key of one scheme and a key of another with the same leading bytes
    // still hash apart: the hash covers the scheme tag.
    let mut mldsa_bytes = [0u8; 1952];
    mldsa_bytes[..32].copy_from_slice(&[17; 32]);
    let lookalike = PublicKey::MlDsa65 {
        mldsa65: MlDsa65PublicKey::from(mldsa_bytes),
    };
    assert_ne!(
        public_key_hash(&lookalike),
        public_key_hash(&ed25519_key(17))
    );
}

#[test]
fn get_auth_claim_returns_the_pubkey_claim_for_each_scheme() {
    for (scheme, claim) in claims_for_each_scheme() {
        let env = thruput_sdk::TestEnvironment::no_cost();
        let wallet = pubkey_claim_address(&claim);
        let expected = claim.clone();
        env.transaction()
            .with_auth_claim(claim)
            .transact(|tx| {
                tx.call(&wallet).run(|| {
                    let actual = thruput_sdk::get_auth_claim().unwrap();
                    assert_eq!(actual, expected, "{scheme}");
                    // The address derives from the claim alone.
                    assert_eq!(
                        pubkey_claim_address(&actual),
                        thruput_sdk::get_current_address(),
                        "{scheme}"
                    );
                })
            })
            .assert_success();
    }
}

#[test]
fn execute_calls_contract_for_each_scheme() {
    const ADDER: ContractAddress = ContractAddress::new([0xAD; 32]);
    for (scheme, claim) in claims_for_each_scheme() {
        let env = thruput_sdk::TestEnvironment::no_cost();
        let adder = env.register_mock(|contract| {
            <AdderContract as Adder>::__mock(contract);
        });
        adder.deploy_contract_to(&ADDER);

        let wallet = pubkey_claim_address(&claim);
        let result = env.transaction().with_auth_claim(claim).transact(|tx| {
            tx.call(&wallet).run(|| {
                <DefaultWalletContract as DefaultWallet>::execute(vec![
                    adder_call(&ADDER, AdderEntrypoints::store_sum, 2, 3),
                ])
            })
        });

        result.assert_success();
        let returns =
            result.assert_return_as::<Vec<default_wallet::WalletBytes>>();
        assert_eq!(returns.len(), 1, "{scheme}");
        assert_eq!(
            returns[0].as_slice(),
            thruput_sdk::encode_xdr(&5u32).unwrap().as_slice(),
            "{scheme}"
        );
        env.advance_block();
        env.transact(|tx| {
            tx.call(&ADDER).run(|| assert_eq!(stored_sum().get(), 5))
        })
        .assert_success();
    }
}

#[test]
fn execute_transfers_hog_for_each_scheme() {
    use thruput_erc20_if::{ERC20Client, ERC20Entrypoints};
    let token = thruput_sdk::precompiles::ADDRESS_PRECOMPILED_HOG_TOKEN;
    let receiver = ContractAddress::new([0xBE; 32]);
    for (scheme, claim) in claims_for_each_scheme() {
        let env = thruput_sdk::TestEnvironment::no_cost();
        let wallet = pubkey_claim_address(&claim);
        env.setup_grant_storage_tokens(&wallet, 1_000);

        let transfer = WalletCall {
            target: token,
            entrypoint: ERC20Entrypoints::transfer.into(),
            gas_limit: u64::MAX,
            calldata: thruput_sdk::encode_xdr(&(receiver, 250i128))
                .unwrap()
                .as_slice()
                .try_into()
                .unwrap(),
        };
        env.transaction()
            .with_auth_claim(claim)
            .transact(|tx| {
                tx.call(&wallet).run(|| {
                    <DefaultWalletContract as DefaultWallet>::execute(vec![
                        transfer,
                    ])
                })
            })
            .assert_success();
        env.advance_block();

        env.transact(|tx| {
            tx.call(&wallet).run(|| {
                let token = ERC20Client::new(&token);
                assert_eq!(token.get_balance_of(&wallet), 750, "{scheme}");
                assert_eq!(token.get_balance_of(&receiver), 250, "{scheme}");
            })
        })
        .assert_success();
    }
}

#[test]
fn execute_rejects_claim_for_another_key_scheme_or_nonce() {
    let env = thruput_sdk::TestEnvironment::no_cost();
    let wallet = pubkey_claim_address(&claim_for_key(&ed25519_key(17), 7));
    for claim in [
        claim_for_key(&mldsa65_key(17), 7), // same key bytes, other scheme
        claim_for_key(&ed25519_key(18), 7), // other key
        claim_for_key(&ed25519_key(17), 8), // other nonce
    ] {
        let result = env.transaction().with_auth_claim(claim).transact(|tx| {
            tx.call(&wallet).run(|| {
                <DefaultWalletContract as DefaultWallet>::execute(Vec::new())
            })
        });
        assert_wallet_error(&result, WalletError::InvalidAuthClaim);
    }
}

#[test]
fn deploy_commits_for_mldsa65_account() {
    let env = thruput_sdk::TestEnvironment::no_cost();
    let claim = claim_for_key(&mldsa65_key(17), 7);
    let wallet = pubkey_claim_address(&claim);
    env.setup_grant_storage_tokens(&wallet, 1_000);
    let contract = env.register_mock(|contract| {
        contract.register_method(init_call(0).entrypoint, initializer_body);
    });
    let hash = *contract.as_hash();
    let nonce = [36; 32].into();
    let result = env.transaction().with_auth_claim(claim).transact(|tx| {
        tx.call(&wallet).run(|| {
            <DefaultWalletContract as DefaultWallet>::deploy(
                &nonce,
                &hash,
                123,
                Some(init_call(0)),
            )
        })
    });
    result.assert_success();
    let deployed = result.assert_return_as::<ContractAddress>();
    env.advance_block();
    env.transact(|tx| {
        tx.call(&deployed).run(|| {
            use thruput_sdk::storage::{Keypath, Value};
            let owner: Value<ContractAddress> = Keypath::default().into();
            assert_eq!(owner.get(), wallet);
        })
    })
    .assert_success();
}

/// Shared cross-language vectors: thruput-protocol
/// `test_vectors/pubkey_auth.json` (keys `ed25519_devnode_bootstrap_0` and
/// `mldsa65_seed_00..1f`, `constants.default_wallet_execute_entrypoint`).
#[test]
fn pubkey_account_addresses_match_shared_vectors() {
    use hex_literal::hex;
    let ed25519 = PublicKey::Ed25519 {
        ed25519: Ed25519PublicKey::from(hex!(
            "591ff2bdaaf5a0cbfd31f7b0f88c1ddee39fd9e106ba46303bde718459960633"
        )),
    };
    let ed25519_hash = public_key_hash(&ed25519);
    assert_eq!(
        *ed25519_hash,
        hex!(
            "3f825d4436e9bb68496070788b81cf7758677e05491e66e3d8f7c7b19be389fc"
        )
    );
    // The ML-DSA-65 key is long, so this one starts from its hash.
    let mldsa65_hash = PublicKeyHash::from(hex!(
        "e857fe91e152c1cfbe66baf0d4dee823d0f0cc26e5086eff71328c5580abd2cf"
    ));
    for (pubkey_hash, nonce, address) in [
        (
            ed25519_hash,
            0,
            hex!(
                "d966d44f249321ec9c222390588c20701efd827734fd9e4b1f94a4a70700c22c"
            ),
        ),
        (
            ed25519_hash,
            1,
            hex!(
                "cd9345bd5dce1c49fbe5d44cf5e9bb60d57737af0a091ba4e7efcfebc351963b"
            ),
        ),
        (
            mldsa65_hash,
            0,
            hex!(
                "61385e57ff74edd5fd79891a3c25e9c8c4dcb2b734a69efd818bea34b87d31e3"
            ),
        ),
        (
            mldsa65_hash,
            1,
            hex!(
                "9d3650371106be8524b579d6294690018204fa3d4864c2dd6f791408210bdb97"
            ),
        ),
    ] {
        let preimage = PubkeyAccountPreimage { pubkey_hash, nonce };
        assert_eq!(*pubkey_account_address(&preimage), address);
    }

    // The vectors' transactions call the wallet's `execute`.
    let execute: thruput_sdk::ContractEntrypointID =
        default_wallet::DefaultWalletEntrypoints::execute.into();
    assert_eq!(execute, hex!("c3edb11fdd6d5d58").into());
}
