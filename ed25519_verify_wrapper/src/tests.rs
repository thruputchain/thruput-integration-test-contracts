use crate::{Client, Ed25519VerifyWrapper, Ed25519VerifyWrapperContract};
use ed25519_dalek::{Signer, SigningKey};
use rand::rngs::OsRng;
use sdk::*;
use std::vec;

#[test]
fn test_ed25519_verify_wrapper() {
    sdk::client_test(
        |env| env.register_mock(<Ed25519VerifyWrapperContract as Ed25519VerifyWrapper>::__register_mock),
        test,
    );
}

fn test(client: &mut Client) {
    let msg = vec![1, 2, 3, 4, 5];

    // Generate Ed25519 keypair and sign the message
    let mut csprng = OsRng;
    let signing_key: SigningKey = SigningKey::generate(&mut csprng);
    let public_key: Ed25519PublicKey =
        signing_key.verifying_key().to_bytes().into();
    let signature: Ed25519Signature = signing_key.sign(&msg).to_bytes().into();

    assert!(client.ed25519_verify(&msg, &signature, &public_key));

    // Verify the signature with the wrong public key
    let signing_key2: SigningKey = SigningKey::generate(&mut csprng);
    let wrong_public_key: Ed25519PublicKey =
        signing_key2.verifying_key().to_bytes().into();
    assert!(!client.ed25519_verify(&msg, &signature, &wrong_public_key))
}
