use bottle_test_if::*;
use sdk::*;

#[derive(Copy, Clone, Default)]
pub struct BT;

#[contract(storage = "false")]
impl BottleTest for BT {
    fn run(&mut self, target_contract_hash: &SHA256Hash) {
        let nonce = SHA256Hash::from([7; 32]);
        let deployed_addr = deploy_contract(&nonce, target_contract_hash).unwrap();

        let mut return_len = 0;

        let flag: i32 = transaction::InvokeFlag::kInvokeFlagReadOnly as i32;

        sys::uninit_Invoke(
            &deployed_addr,
            0xCE6746E15AF20000,
            u64::MAX,
            &[0, 0, 0, 1],
            flag,
            &mut return_len,
        )
        .unwrap();
    }
}
