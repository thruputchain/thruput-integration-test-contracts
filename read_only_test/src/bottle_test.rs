use bottle_test_if::*;
use thruput_sdk::*;
use thruput_sdk::xdr::cxs;
use thruput_sdk::storage::Value;

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

        let mut outputs = core::mem::MaybeUninit::uninit();

        let flag = InvokeFlag::ReadOnly.0.cast_unsigned();
        let calldata = cxs::encode(&true).unwrap();

        sys::uninit_Invoke(
            &target_addr,
            &calldata,
            &[],
            &sys::CopyableInvokeInputs {
                entrypoint_id: crate::ReadOnlyTestEntrypoints::read_only_bytestring_set.into_u64(),
                gas_limit: u64::MAX,
                flags: flag,
                reserved: 0,
            },
            &mut outputs,
        )
        .unwrap();
    }
}
