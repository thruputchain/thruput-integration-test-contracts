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

        let flag: i32 = invoke_flag::InvokeFlag::ReadOnly.0;
        let calldata = cxs::encode(&true).unwrap();

        sys::uninit_Invoke(
            &target_addr,
            crate::ReadOnlyTestEntrypoints::read_only_bytestring_set.into_u64(),
            u64::MAX,
            &calldata,
            flag,
            &mut return_len,
        )
        .unwrap();
    }
}
