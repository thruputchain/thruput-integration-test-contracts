use bottle_test_if::*;
use sdk::*;
use sdk::storage::Value;

#[derive(Debug)]
#[storage]
pub struct BT {
    target_addr: Value<Option<ContractAddress>>,
}


#[contract]
impl BottleTest for BT {

    fn setup(&mut self, target_contract_hash: &SHA256Hash) {
        let nonce: SHA256Hash = [7; 32].into();
        let target_addr = deploy_contract(&nonce, target_contract_hash);

        self.target_addr.set(&Some(target_addr));
    }

    fn run(&mut self) {
        let target_addr = self.target_addr.get().unwrap_or_else(||
            panic!("bottle-test: target_addr unset; please run setup() before run()"));

        let mut return_len : core::mem::MaybeUninit<u32> = core::mem::MaybeUninit::uninit();

        let flag: i32 = transaction::InvokeFlag::ReadOnly.0;

        use crate::implementation::read_only_test_exports::READ_ONLY_BYTESTRING_SET_EXPORT_NUM;

        sys::uninit_Invoke(
            &target_addr,
            u64::from_be_bytes(READ_ONLY_BYTESTRING_SET_EXPORT_NUM.into_inner()),
            u64::MAX,
            &[0, 0, 0, 1],
            flag,
            &mut return_len,
        )
        .unwrap();
    }
}
