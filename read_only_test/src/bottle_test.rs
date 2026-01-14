use bottle_test_if::*;
use sdk::*;

#[derive(Debug)]
#[storage]
pub struct BT {
    target_addr: Option<ContractAddress>,
}


#[contract]
impl BottleTest for BT {

    #[init]
    fn setup(&mut self, target_contract_hash: &SHA256Hash) {
        let nonce: SHA256Hash = [7; 32].into();
        let target_addr = deploy_contract(&nonce, target_contract_hash).unwrap();

        self.target_addr = Some(target_addr);
    }

    fn run(&mut self) {
        let target_addr = self.target_addr.as_ref().unwrap_or_else(||
            panic!("bottle-test: target_addr unset; please run setup() before run()"));

        let mut return_len = 0;

        let flag: i32 = transaction::InvokeFlag::kInvokeFlagReadOnly as i32;

        sys::uninit_Invoke(
            &target_addr,
            0xCE6746E15AF20000,
            u64::MAX,
            &[0, 0, 0, 1],
            flag,
            &mut return_len,
        )
        .unwrap();
    }
}
