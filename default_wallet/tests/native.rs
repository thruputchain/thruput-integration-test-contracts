use default_wallet::{DefaultWallet, DefaultWalletContract, WalletError};
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
        AuthClaim::kPubkeySignature { pubkey_preimage } => pubkey_account_address(pubkey_preimage),
        _ => panic!("expected pubkey signature claim"),
    }
}

fn assert_wallet_error(result: &sdk::TransactResult, expected: WalletError) {
    result.assert_abort();

    let mut reader = sdk::__xdrs::cxs::buf_reader::BufReader::new(result.frame().abort_buffer());
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
        tx.call(&wallet_address)
            .run(|| <DefaultWalletContract as DefaultWallet>::execute(Vec::new()))
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
        tx.call(&wrong_address)
            .run(|| <DefaultWalletContract as DefaultWallet>::execute(Vec::new()))
    });

    assert_wallet_error(&result, WalletError::InvalidAuthClaim);
}

#[test]
fn execute_rejects_default_source_address_auth_claim() {
    let env = sdk::TestEnvironment::no_cost();

    let result = env.transact(|tx| {
        tx.call(&ContractAddress::new([1u8; 32]))
            .run(|| <DefaultWalletContract as DefaultWallet>::execute(Vec::new()))
    });

    assert_wallet_error(&result, WalletError::InvalidAuthClaim);
}

#[test]
fn deploy_funded_rejects_wrong_auth_before_funding() {
    let env = sdk::TestEnvironment::no_cost();
    let claim = pubkey_claim(17, 7);
    let result = env.transaction().with_auth_claim(claim).transact(|tx| {
        tx.call(&ContractAddress::new([1u8; 32])).run(|| {
            <DefaultWalletContract as DefaultWallet>::deploy_funded(
                &[0; 32].into(),
                &[0; 32].into(),
                100,
            )
        })
    });
    assert_wallet_error(&result, WalletError::InvalidAuthClaim);
}

#[test]
fn deploy_funded_rejects_source_address_auth() {
    let env = sdk::TestEnvironment::no_cost();
    let result = env.transact(|tx| {
        tx.call(&ContractAddress::new([1u8; 32])).run(|| {
            <DefaultWalletContract as DefaultWallet>::deploy_funded(
                &[0; 32].into(),
                &[0; 32].into(),
                100,
            )
        })
    });
    assert_wallet_error(&result, WalletError::InvalidAuthClaim);
}
