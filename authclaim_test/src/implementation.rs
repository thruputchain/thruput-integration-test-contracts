use super::interface::*;
use sdk::*;

#[derive(Default)]
pub struct AuthClaimTestContract;

#[contract(exports = "guest_exports", storage = "false")]
impl AuthClaimTest for AuthClaimTestContract {
    fn test_invoked_address_signature(&self) {
        let auth_claim = sdk::get_auth_claim(0).unwrap();

        match auth_claim {
            AuthClaim::kInvokedAddressSignature{invoked_address_signature_id } => {
                // Hardcoded expected value: KeyID = 5
                let expected_key_id: u32 = 5;
                if invoked_address_signature_id != expected_key_id {
                    trap();
                }
            },
            _ => {
                trap();
            }
        }
    }

    fn test_invoked_address_signed_weight(&self) {
        let auth_claim = sdk::get_auth_claim(1).unwrap();
        
        match auth_claim {
            AuthClaim::kInvokedAddressSignedWeight{ invoked_address_signed_weight} => {
                // Hardcoded expected values: BitMap = bits 0 and 2 set (value = 1 | (1 << 2) = 5), Weight = 10
                let expected_bitmap: BitMap = 5; // bits 0 and 2 set
                let expected_weight: Weight = 10;
                
                if invoked_address_signed_weight.accepted_keys != expected_bitmap {
                    trap();
                }
                if invoked_address_signed_weight.required_weight != expected_weight {
                    trap();
                }
            },
            _ => {
                trap();
            }
        }
    }

    fn test_other_address_signature(&self) {
        let auth_claim = sdk::get_auth_claim(2).unwrap();
        
        match auth_claim {
            AuthClaim::kOtherAddressSignature{ other_address_signature_id } => {
                // Hardcoded expected values: ContractAddress = [0xAA; 32], KeyID = 7
                let expected_address = ContractAddress::from([0xAAu8; 32]);
                let expected_key_id: u32 = 7;
                
                if other_address_signature_id.address != expected_address {
                    trap();
                }
                if other_address_signature_id.key_id != expected_key_id {
                    trap();
                }
            },
            _ => {
                trap();
            }
        }
    }

    fn test_other_address_signed_weight(&self) {
        let auth_claim = sdk::get_auth_claim(3).unwrap();
        
        match auth_claim {
            AuthClaim::kOtherAddressSignedWeight{ other_address_signed_weight } => {
                // Hardcoded expected values: ContractAddress = [0xBB; 32], BitMap = bits 0 and 1 set (value = 1 | (1 << 1) = 3), Weight = 15
                let expected_address = ContractAddress::from([0xBBu8; 32]);
                let expected_bitmap: BitMap = 3; // bits 0 and 1 set
                let expected_weight: Weight = 15;
                
                if other_address_signed_weight.address != expected_address {
                    trap();
                }
                if other_address_signed_weight.accepted_keys != expected_bitmap {
                    trap();
                }
                if other_address_signed_weight.required_weight != expected_weight {
                    trap();
                }
            },
            _ => {
                trap();
            }
        }
    }
}

