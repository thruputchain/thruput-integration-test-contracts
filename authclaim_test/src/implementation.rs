use super::interface::*;
use sdk::*;

#[derive(Default)]
pub struct AuthClaimTestContract;

#[contract(storage = false)]
impl AuthClaimTest for AuthClaimTestContract {
    fn test_get_auth_claim(&self) {
        let auth_claim = sdk::get_auth_claim().unwrap();

        match auth_claim {
            AuthClaim::kSourceAddressAuthData {
                source_address_claim,
            } => {
                if source_address_claim.required_weight == 0
                    && source_address_claim.allowed_keys == 0
                {
                    return;
                }
                trap();
            }
            _ => {
                trap();
            }
        }
    }

    fn test_set_address_required_weight(&mut self) {
        sdk::set_address_required_weight(5).unwrap();
    }

    fn test_add_address_public_key(&mut self) {
        let pk = PublicKey::Ed25519 {
            ed25519: Ed25519PublicKey::from([0xAAu8; 32]),
        };
        sdk::add_address_public_key(0, pk, 10).unwrap();
    }

    fn test_remove_address_public_key(&mut self) {
        let pk = PublicKey::Ed25519 {
            ed25519: Ed25519PublicKey::from([0xBBu8; 32]),
        };
        sdk::add_address_public_key(1, pk, 5).unwrap();
        sdk::remove_address_public_key(1).unwrap();
    }

    fn test_add_pubkey_index_oob(&self) {
        let pk = PublicKey::Ed25519 {
            ed25519: Ed25519PublicKey::from([0xCCu8; 32]),
        };
        let res = sdk::add_address_public_key(21, pk, 1);
        assert!(res.is_err());
    }
}
