use crate::interface::*;
use sdk::*;
use sdk::sys::SyscallErrorCode;

#[link(wasm_import_module = "marmot")]
unsafe extern "C" {
    #[link_name = "syscall"]
    fn direct_syscall(
        syscall_no: u64,
        arg1: u64,
        arg2: u64,
        arg3: u64,
        arg4: u64,
        arg5: u64,
        arg6: u64,
        arg7: u64,
    ) -> i32;
}

const K_SYSCALL_BYTESTRING_SET: u64 = groundhog_xdr::SyscallNo::kBytestringSet.0 as u64;

#[derive(Default)]
pub struct ReadOnlyTestContract;

#[contract(storage = false)]
impl ReadOnlyTest for ReadOnlyTestContract {
    fn read_only_bytestring_set(&self, expect_error: bool) {
        let key: StorageKeyContractData = static_key!("test");
        let res = sdk::sys_result::bytestring::bytestring_set(&key, b"test");
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
        let flag = groundhog_xdr::invoke_flag::InvokeFlag::ReadOnly.0;
        let entrypoint =
            groundhog_erc20_if::ERC20Entrypoints::get_balance_of.into_u64();
        let calldata = cxs::encode(target_addr).unwrap();
        let res = sdk::sys::raw_Invoke(
            &sdk::precompiles::ADDRESS_PRECOMPILED_HOG_TOKEN,
            &calldata,
            &[],
            &sdk::sys::CopyableInvokeInputs {
                entrypoint_id: entrypoint,
                gas_limit: u64::MAX,
                flags: flag,
            },
            &mut sdk::sys::CopyableInvokeOutputs {
                return_buffer_len: 0,
                returned_auth_token_len: 0,
            },
        );
        assert!(res.is_ok(), "get_balance_of should succeed in read-only mode");
    }

    fn read_only_native_transfer(&self, receiver_addr: &ContractAddress) {
        let flag = groundhog_xdr::invoke_flag::InvokeFlag::ReadOnly.0;
        let entrypoint =
            groundhog_erc20_if::ERC20Entrypoints::transfer.into_u64();
        // CXS-encode the arguments: (ContractAddress, (u64, u64))
        let calldata = cxs::encode(&(*receiver_addr, (0u64, 100u64))).unwrap();
        let res = sdk::sys::raw_Invoke(
            &sdk::precompiles::ADDRESS_PRECOMPILED_HOG_TOKEN,
            &calldata,
            &[],
            &sdk::sys::CopyableInvokeInputs {
                entrypoint_id: entrypoint,
                gas_limit: u64::MAX,
                flags: flag,
            },
            &mut sdk::sys::CopyableInvokeOutputs {
                return_buffer_len: 0,
                returned_auth_token_len: 0,
            },
        );
        // Transfer should fail in read-only mode
        assert!(res.is_err(), "transfer should fail in read-only mode");
    }

    fn read_only_deploy_output_preserved(&self) {
        let addr = sdk::get_current_address();
        let entrypoint =
            crate::ReadOnlyTestEntrypoints::read_only_deploy_inner.into_u64();
        let flag = groundhog_xdr::invoke_flag::InvokeFlag::ReadOnly.0;
        let res = sdk::sys::raw_Invoke(
            &addr,
            &[],
            &[],
            &sdk::sys::CopyableInvokeInputs {
                entrypoint_id: entrypoint,
                gas_limit: u64::MAX,
                flags: flag,
            },
            &mut sdk::sys::CopyableInvokeOutputs {
                return_buffer_len: 0,
                returned_auth_token_len: 0,
            },
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

    fn read_only_aliasing_before_read_only_check(&self)
    {
        let addr = sdk::get_current_address();
        let entrypoint = crate::ReadOnlyTestEntrypoints::
            read_only_aliasing_before_read_only_check_inner
            .into_u64();
        let flag = groundhog_xdr::invoke_flag::InvokeFlag::ReadOnly.0;
        let res = sdk::sys::raw_Invoke(
            &addr,
            &[],
            &[],
            &sdk::sys::CopyableInvokeInputs {
                entrypoint_id: entrypoint,
                gas_limit: u64::MAX,
                flags: flag,
            },
            &mut sdk::sys::CopyableInvokeOutputs {
                return_buffer_len: 0,
                returned_auth_token_len: 0,
            },
        );
        // The inner method handles the deploy error and returns normally
        assert!(res.is_ok(), "read-only self-invoke should succeed");
    }

    fn read_only_aliasing_before_read_only_check_inner(&self)
    {
        let mut buf : [u8;64] = [0xAB;64];
        let key_ptr = buf.as_mut_ptr() as u64;
        let bytestring_ptr = (buf.as_mut_ptr() as u64) + 16;
        let flags = 0;
        let err_code = unsafe {
            direct_syscall(
                K_SYSCALL_BYTESTRING_SET,
                key_ptr,
                bytestring_ptr,
                flags,
                0,
                0,
                0,
                0
            )
        };
        assert_eq!(err_code, groundhog_xdr::syscall_error::SyscallErrorCode::kSyscallErrorCodeFault.0);
        assert_eq!(buf, [0xAB;64]);
    }


}
