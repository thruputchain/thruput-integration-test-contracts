use crate::implementation;
use crate::Client;
use ed25519_dalek::{Signer, SigningKey};
use rand::rngs::OsRng;
use sdk::prelude::*;
use sdk::contract_result::*;

use std::vec;

#[test]
fn test_ed25519_verify_wrapper() {
    let mut env = sdk::TestEnvironment::new();

    let test_contract = implementation::test_setup::register(&mut env);
    let contract_hash = env.setup_create_contract(&test_contract);

    sdk::TestEnvironment::enable_syscall_logging();

    let mut handler = env.prepare_test(move || -> () {
        match test_ed25519_verify_wrapper_inner(&contract_hash) {
            Ok(_) => (),
            Err(error) => panic!("test failed: {}", error),
        }
    });

    let res = handler.root_invoke();
    assert_eq!(res.result.panic, sdk::InvokeErrorCode::kInvokeNone as i32);
}

fn test_ed25519_verify_wrapper_inner(
    contract_hash: &sdk::types::SHA256Hash,
) -> Result<()> {
    let nonce = SHA256Hash::from([7; 32]);
    let deployed_addr = deploy_contract(&nonce, contract_hash)?;

    let mut client = Client::new(&deployed_addr);

    let msg = vec![1, 2, 3, 4, 5];

    // Generate Ed25519 keypair and sign the message
    let mut csprng = OsRng;
    let signing_key: SigningKey = SigningKey::generate(&mut csprng);
    let public_key: Ed25519PublicKey =
        signing_key.verifying_key().to_bytes().into();
    let signature: Ed25519Signature = signing_key.sign(&msg).to_bytes().into();

    let res = client.ed25519_verify(&msg, &signature, &public_key).unwrap();
    assert_eq!(res, true);

    // Verify the signature with the wrong public key
    let signing_key2: SigningKey = SigningKey::generate(&mut csprng);
    let wrong_public_key: Ed25519PublicKey =
        signing_key2.verifying_key().to_bytes().into();
    let res = client.ed25519_verify(&msg, &signature, &wrong_public_key).unwrap();
    assert_eq!(res, false);

    Ok(())
}
