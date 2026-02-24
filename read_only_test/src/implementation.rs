use crate::interface::*;
use sdk::*;

// TODO(geoff): Add option in the SDK to get these method numbers exported as part of the client interface,
// not just the implementation handler
/// Derive a u64 entrypoint ID from the first 8 bytes of a SHA-256 hash.
/// The preimage for an SDK-derived entrypoint is: IFACE_NAME || MAJOR_VERSION || METHOD_NAME
/// (see groundhog-contract-util for the canonical implementation).
const fn entrypoint_id(hash: [u8; 32]) -> u64 {
    u64::from_be_bytes([
        hash[0], hash[1], hash[2], hash[3], hash[4], hash[5], hash[6], hash[7],
    ])
}

// ERC20 v0.2.x entrypoints, derived from SHA256("ERC20" || "0.2" || method_name)
const ERC20_GET_BALANCE_OF: u64 = entrypoint_id(sdk::hash_to_array!("ERC200.2get_balance_of"));
const ERC20_TRANSFER: u64 = entrypoint_id(sdk::hash_to_array!("ERC200.2transfer"));

#[derive(Default)]
struct ReadOnlyTestContract;

#[contract(storage = "false")]
impl ReadOnlyTest for ReadOnlyTestContract {
    fn read_only_bytestring_set(&self, expect_error: bool) {
        let key: StorageKeyContractData = static_key!("test");
        let res = try_bytestring_set(&key, b"test");
        sdk::debug_log!("res: {:?}", res);
        if expect_error {
            assert!(res.err().unwrap().is_code(
                SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput
            ));
        } else {
            assert_eq!(res, Ok(()));
        }
    }

    fn read_only_log_event(&self, expect_error: bool) {
        let data = b"test event";
        let res = groundhog_xdr::syscalls::raw_LogEvent(None, None, None, data);
        sdk::debug_log!("log_event res: {:?}", res);
        if expect_error {
            assert!(res.err().unwrap().is_code(
                SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput
            ));
        } else {
            assert_eq!(res, Ok(()));
        }
    }

    fn read_only_native_get_balance(&self, target_addr: &ContractAddress) {
        let flag = groundhog_xdr::transaction::InvokeFlag::kInvokeFlagReadOnly as i32;
        let calldata: &[u8] = target_addr.as_ref();
        let mut return_len = 0u32;
        let res = sdk::sys::raw_Invoke(
            &sdk::precompiles::ADDRESS_PRECOMPILED_HOG_TOKEN,
            ERC20_GET_BALANCE_OF,
            u64::MAX,
            calldata,
            flag,
            &mut return_len,
        );
        assert!(res.is_ok(), "get_balance_of should succeed in read-only mode");
    }

    fn read_only_native_transfer(&self, receiver_addr: &ContractAddress) {
        let flag = groundhog_xdr::transaction::InvokeFlag::kInvokeFlagReadOnly as i32;
        let mut calldata = [0u8; 48];
        calldata[..32].copy_from_slice(receiver_addr.as_ref());
        // amount = (0, 100) = 100 units
        calldata[40..48].copy_from_slice(&100u64.to_be_bytes());
        let mut return_len = 0u32;
        let res = sdk::sys::raw_Invoke(
            &sdk::precompiles::ADDRESS_PRECOMPILED_HOG_TOKEN,
            ERC20_TRANSFER,
            u64::MAX,
            &calldata,
            flag,
            &mut return_len,
        );
        // Transfer should fail in read-only mode
        assert!(res.is_err(), "transfer should fail in read-only mode");
    }

    fn read_only_deploy_output_preserved(&self) {
        let addr = sdk::get_current_address().unwrap();
        let entrypoint =
            u64::from_be_bytes(crate::READ_ONLY_DEPLOY_INNER_EXPORT_NUM.into_inner());
        let flag = groundhog_xdr::transaction::InvokeFlag::kInvokeFlagReadOnly as i32;
        let mut return_len = 0u32;
        let res = sdk::sys::raw_Invoke(
            &addr,
            entrypoint,
            u64::MAX,
            &[],
            flag,
            &mut return_len,
        );
        // The inner method handles the deploy error and returns normally
        assert!(res.is_ok(), "read-only self-invoke should succeed");
    }

    fn read_only_deploy_inner(&self) {
        let nonce = SHA256Hash::default();
        let contract_hash = SHA256Hash::from([0xFFu8; 32]);
        let mut deployed_address = ContractAddress::from([0xABu8; 32]);

        let res = sdk::sys::raw_DeployContract(
            &nonce, &contract_hash, &mut deployed_address,
        );

        // Deploy should fail in read-only mode
        assert!(res.is_err(), "deploy should fail in read-only mode");
        assert!(res.err().unwrap().is_code(
            SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput
        ));

        // The output address should NOT have been zeroed
        assert_eq!(
            deployed_address,
            ContractAddress::from([0xABu8; 32]),
            "deployed_address was overwritten by failing deploy syscall"
        );
    }
}
