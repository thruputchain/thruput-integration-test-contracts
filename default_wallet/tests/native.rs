use default_wallet::{
    DefaultWallet, DefaultWalletContract, WalletCall, WalletError,
    WalletTokenRef,
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
        mldsa65: MlDsa65PublicKey::from([byte; 1952]).into(),
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

/// `execute`'s `mints` for a batch that mints nothing.
fn no_mints() -> thruput_sdk::VecM<default_wallet::WalletBytes, { default_wallet::MAX_MINTS }> {
    Vec::new().try_into().unwrap()
}

/// A call's `tokens` when it forwards none.
fn no_tokens() -> thruput_sdk::VecM<WalletTokenRef, { default_wallet::MAX_CALL_TOKENS }> {
    Vec::new().try_into().unwrap()
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
            <DefaultWalletContract as DefaultWallet>::execute(no_mints(), Vec::new())
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
            <DefaultWalletContract as DefaultWallet>::execute(no_mints(), Vec::new())
        })
    });

    assert_wallet_error(&result, WalletError::InvalidAuthClaim);
}

#[test]
fn execute_rejects_default_source_address_auth_claim() {
    let env = thruput_sdk::TestEnvironment::no_cost();

    let result = env.transact(|tx| {
        tx.call(&ContractAddress::new([1u8; 32])).run(|| {
            <DefaultWalletContract as DefaultWallet>::execute(no_mints(), Vec::new())
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
                &thruput_sdk::types::genesis_addresses::ADDRESS_NATIVE_TOKEN,
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
    thruput_sdk::fetch_calldata_and_auth_tokens(&mut calldata);
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
                    &thruput_sdk::types::genesis_addresses::ADDRESS_NATIVE_TOKEN,
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
                    &thruput_sdk::types::genesis_addresses::ADDRESS_NATIVE_TOKEN,
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
        tokens: no_tokens(),
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
            <DefaultWalletContract as DefaultWallet>::execute(no_mints(), vec![
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
                    <DefaultWalletContract as DefaultWallet>::execute(no_mints(), calls)
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
                &thruput_sdk::types::genesis_addresses::ADDRESS_NATIVE_TOKEN,
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
        mldsa65: MlDsa65PublicKey::from(mldsa_bytes).into(),
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
                <DefaultWalletContract as DefaultWallet>::execute(no_mints(), vec![
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
fn execute_transfers_native_tokens_for_each_scheme() {
    use thruput_erc20_if::{ERC20Client, ERC20Entrypoints};
    let token = thruput_sdk::types::genesis_addresses::ADDRESS_NATIVE_TOKEN;
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
            tokens: no_tokens(),
        };
        env.transaction()
            .with_auth_claim(claim)
            .transact(|tx| {
                tx.call(&wallet).run(|| {
                    <DefaultWalletContract as DefaultWallet>::execute(no_mints(), vec![
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
                <DefaultWalletContract as DefaultWallet>::execute(no_mints(), Vec::new())
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

/// A contract whose methods take and return auth tokens, for the wallet to
/// forward tokens to and collect them from.
#[thruput_sdk::contract]
pub trait TokenDesk {
    /// Consume `token`, returning its payload and who minted it.
    fn redeem(token: thruput_sdk::signer::Signed<u64>) -> (u64, ContractAddress);
    /// Consume both tokens, returning their payloads in argument order.
    fn redeem_pair(
        first: thruput_sdk::signer::Signed<u64>,
        second: thruput_sdk::signer::Signed<u64>,
    ) -> (u64, u64);
    /// Mint two tokens `consumer` may consume, carrying `a` and `b`.
    fn issue(
        consumer: &ContractAddress,
        a: u64,
        b: u64,
    ) -> (
        thruput_sdk::signer::Signed<u64>,
        thruput_sdk::signer::Signed<u64>,
    );
}

pub struct TokenDeskContract;

#[thruput_sdk::contract]
impl TokenDesk for TokenDeskContract {
    fn redeem(token: thruput_sdk::signer::Signed<u64>) -> (u64, ContractAddress) {
        token.consume()
    }

    fn redeem_pair(
        first: thruput_sdk::signer::Signed<u64>,
        second: thruput_sdk::signer::Signed<u64>,
    ) -> (u64, u64) {
        (first.consume().0, second.consume().0)
    }

    fn issue(
        consumer: &ContractAddress,
        a: u64,
        b: u64,
    ) -> (
        thruput_sdk::signer::Signed<u64>,
        thruput_sdk::signer::Signed<u64>,
    ) {
        let mint = |data| {
            thruput_sdk::signer::mint_auth_token::<u64>(
                thruput_sdk::block::get_block_height(),
                thruput_sdk::mintable::AuthTokenPreimage {
                    consumer: *consumer,
                    data,
                },
            )
        };
        (mint(a), mint(b))
    }
}

const DESK: ContractAddress = ContractAddress::new([0xDE; 32]);
const OTHER_DESK: ContractAddress = ContractAddress::new([0x0D; 32]);

fn wallet_bytes(bytes: &[u8]) -> default_wallet::WalletBytes {
    bytes.try_into().unwrap()
}

/// The encoded node for a token carrying `data` that `consumer` may consume.
fn mint_node(consumer: ContractAddress, data: u64) -> default_wallet::WalletBytes {
    let node = thruput_sdk::mintable::build_auth_token_node::<u64>(
        thruput_sdk::mintable::AuthTokenPreimage { consumer, data },
    );
    wallet_bytes(&thruput_sdk::mintable::encode_auth_token_node(&node))
}

fn minted(index: u32) -> WalletTokenRef {
    WalletTokenRef::Minted(index)
}

fn returned(call: u32, index: u32) -> WalletTokenRef {
    WalletTokenRef::Returned { call, index }
}

fn desk_call<A: thruput_sdk::WriteXDR>(
    entrypoint: TokenDeskEntrypoints,
    args: &A,
    tokens: Vec<WalletTokenRef>,
) -> WalletCall {
    WalletCall {
        target: DESK,
        entrypoint: entrypoint.into(),
        gas_limit: u64::MAX,
        calldata: wallet_bytes(&thruput_sdk::encode_xdr(args).unwrap()),
        tokens: tokens.try_into().unwrap(),
    }
}

fn redeem(token: WalletTokenRef) -> WalletCall {
    desk_call(TokenDeskEntrypoints::redeem, &(), vec![token])
}

fn redeem_pair(first: WalletTokenRef, second: WalletTokenRef) -> WalletCall {
    desk_call(TokenDeskEntrypoints::redeem_pair, &(), vec![first, second])
}

fn issue(a: u64, b: u64) -> WalletCall {
    desk_call(TokenDeskEntrypoints::issue, &(DESK, a, b), Vec::new())
}

/// An environment with the token desk deployed, and a wallet to drive it.
struct TokenBatch {
    env: thruput_sdk::TestEnvironmentHandle,
    claim: AuthClaim,
    wallet: ContractAddress,
}

impl TokenBatch {
    fn new() -> Self {
        let env = thruput_sdk::TestEnvironment::no_cost();
        for address in [DESK, OTHER_DESK] {
            env.register_mock(|contract| {
                <TokenDeskContract as TokenDesk>::__mock(contract);
            })
            .deploy_contract_to(&address);
        }
        let claim = pubkey_claim(17, 7);
        let wallet = pubkey_claim_address(&claim);
        Self { env, claim, wallet }
    }

    fn execute(
        &self,
        mints: Vec<default_wallet::WalletBytes>,
        calls: Vec<WalletCall>,
    ) -> thruput_sdk::TransactResult {
        self.env
            .transaction()
            .with_auth_claim(self.claim.clone())
            .transact(|tx| {
                tx.call(&self.wallet).run(|| {
                    <DefaultWalletContract as DefaultWallet>::execute(
                        mints.try_into().unwrap(),
                        calls,
                    )
                })
            })
    }
}

/// Decode one call's return data out of the wallet's return value.
fn return_item<R: for<'r> thruput_sdk::ReadXDR<'r>>(
    result: &thruput_sdk::TransactResult,
    call: usize,
) -> R {
    let returns = result.assert_return_as::<Vec<default_wallet::WalletBytes>>();
    thruput_sdk::__xdrs::cxs::decode(returns[call].as_slice()).unwrap()
}

#[test]
fn execute_forwards_a_token_minted_as_the_wallet() {
    let batch = TokenBatch::new();
    let result =
        batch.execute(vec![mint_node(DESK, 7)], vec![redeem(minted(0))]);
    result.assert_success();
    // The desk sees the wallet, the account that signed, as the minter.
    assert_eq!(
        return_item::<(u64, ContractAddress)>(&result, 0),
        (7, batch.wallet)
    );
}

#[test]
fn execute_forwards_minted_tokens_in_reference_order() {
    let batch = TokenBatch::new();
    let result = batch.execute(
        vec![mint_node(DESK, 1), mint_node(DESK, 2), mint_node(DESK, 3)],
        vec![redeem_pair(minted(2), minted(0)), redeem(minted(1))],
    );
    result.assert_success();
    assert_eq!(return_item::<(u64, u64)>(&result, 0), (3, 1));
    assert_eq!(return_item::<(u64, ContractAddress)>(&result, 1).0, 2);
}

#[test]
fn execute_forwards_tokens_an_earlier_call_returned() {
    let batch = TokenBatch::new();
    // Two calls return tokens and a mint sits beside them, so a reference
    // resolves by the call and position it names, not by a running count.
    let result = batch.execute(
        vec![mint_node(DESK, 9)],
        vec![
            issue(11, 22),
            issue(33, 44),
            redeem_pair(returned(1, 1), returned(0, 0)),
            redeem_pair(returned(0, 1), minted(0)),
            redeem(returned(1, 0)),
        ],
    );
    result.assert_success();
    assert_eq!(return_item::<(u64, u64)>(&result, 2), (44, 11));
    assert_eq!(return_item::<(u64, u64)>(&result, 3), (22, 9));
    // A returned token keeps its own minter: the desk, not the wallet.
    assert_eq!(
        return_item::<(u64, ContractAddress)>(&result, 4),
        (33, DESK)
    );
}

#[test]
fn execute_drops_tokens_left_unforwarded() {
    let batch = TokenBatch::new();
    batch
        .execute(vec![mint_node(DESK, 7)], vec![issue(1, 2)])
        .assert_success();
}

#[test]
fn execute_rejects_references_that_name_no_token() {
    let batch = TokenBatch::new();
    let cases: [(&str, Vec<WalletCall>); 5] = [
        ("mint index past mints", vec![redeem(minted(1))]),
        ("its own call", vec![redeem(returned(0, 0))]),
        ("a later call", vec![redeem(returned(1, 0)), issue(1, 2)]),
        ("index past returned", vec![issue(1, 2), redeem(returned(0, 2))]),
        (
            "a call that returned none",
            vec![redeem(minted(0)), redeem(returned(0, 0))],
        ),
    ];
    for (case, calls) in cases {
        let result = batch.execute(vec![mint_node(DESK, 7)], calls);
        assert!(result.frame().is_abort(), "{case}");
        assert_wallet_error(&result, WalletError::InvalidTokenRef);
    }
}

#[test]
fn execute_rejects_forwarding_a_token_twice() {
    let batch = TokenBatch::new();
    let cases: [(&str, Vec<WalletCall>); 4] = [
        ("twice in one call", vec![redeem_pair(minted(0), minted(0))]),
        (
            "again in a later call",
            vec![redeem(minted(0)), redeem(minted(0))],
        ),
        (
            "a returned token twice in one call",
            vec![issue(1, 2), redeem_pair(returned(0, 1), returned(0, 1))],
        ),
        (
            "a returned token again in a later call",
            vec![issue(1, 2), redeem(returned(0, 0)), redeem(returned(0, 0))],
        ),
    ];
    for (case, calls) in cases {
        let result = batch.execute(vec![mint_node(DESK, 7)], calls);
        assert!(result.frame().is_abort(), "{case}");
        assert_wallet_error(&result, WalletError::TokenAlreadyForwarded);
    }
}

#[test]
fn execute_bounds_calls_and_mints() {
    let batch = TokenBatch::new();
    assert_wallet_error(
        &batch.execute(vec![mint_node(DESK, 7)], Vec::new()),
        WalletError::EmptyCallList,
    );
    assert_wallet_error(
        &batch.execute(Vec::new(), (0..17).map(|_| issue(1, 2)).collect()),
        WalletError::TooManyCalls,
    );
    // The mint bound is the argument type's, so the largest list that
    // decodes is the largest the wallet sees.
    let mints = (0..u64::from(default_wallet::MAX_MINTS))
        .map(|data| mint_node(DESK, data))
        .collect();
    let result = batch.execute(mints, vec![redeem(minted(15))]);
    result.assert_success();
    assert_eq!(return_item::<(u64, ContractAddress)>(&result, 0).0, 15);
}

#[test]
fn execute_aborts_on_a_mint_the_runtime_refuses() {
    use thruput_sdk::exn::LibraryExn;
    let batch = TokenBatch::new();
    let result = batch.execute(
        vec![mint_node(DESK, 7), wallet_bytes(&[0xFF; 3])],
        vec![redeem(minted(0))],
    );
    let exn: LibraryExn = result.assert_abort_as();
    assert!(
        matches!(exn, LibraryExn::Syscall(_)),
        "expected the MintAuthToken syscall error, got {exn:?}"
    );
}

#[test]
fn execute_rethrows_a_consumer_mismatch_from_the_callee() {
    use thruput_sdk::exn::LibraryExn;
    let batch = TokenBatch::new();
    // Minted for another contract, so the desk's consume is refused and its
    // error reaches the wallet's caller as any callee error does.
    let result =
        batch.execute(vec![mint_node(OTHER_DESK, 7)], vec![redeem(minted(0))]);
    let exn: LibraryExn = result.assert_abort_as();
    assert!(
        matches!(exn, LibraryExn::Syscall(_)),
        "expected the ConsumeAuthToken syscall error, got {exn:?}"
    );
}

/// The reference encoding is a discriminant followed by the variant's fields,
/// and the discriminants are fixed: clients and hogd's native wallet encode
/// the same bytes.
#[test]
fn wallet_token_ref_discriminants_are_pinned() {
    assert_eq!(
        thruput_sdk::encode_xdr(&minted(5)).unwrap().to_vec(),
        thruput_sdk::encode_xdr(&(0i32, 5u32)).unwrap().to_vec(),
    );
    assert_eq!(
        thruput_sdk::encode_xdr(&returned(7, 9)).unwrap().to_vec(),
        thruput_sdk::encode_xdr(&(1i32, 7u32, 9u32)).unwrap().to_vec(),
    );
}
