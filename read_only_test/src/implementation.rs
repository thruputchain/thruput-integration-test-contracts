use crate::interface::*;
use sdk::*;
use sdk::xdr::cxs;
use sdk::sys::SyscallErrorCode;
use sdk::transaction::InvocationFrameStatusCode;
use groundhog_xdr::syscalls::SyscallWord;

// `groundhog_syscall::syscall_wrapper` is the single shared syscall-dispatch
// chokepoint: the SDK routes its own syscalls through it too, so the wasm
// linker sees exactly one `marmot.syscall` import (the meter caps function
// imports at 1). The thin `direct_syscall` shim keeps the existing call sites
// (which pass `SyscallWord`-typed args) unchanged.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
unsafe fn direct_syscall(
    syscall_no: SyscallWord,
    arg1: SyscallWord,
    arg2: SyscallWord,
    arg3: SyscallWord,
    arg4: SyscallWord,
    arg5: SyscallWord,
    arg6: SyscallWord,
    arg7: SyscallWord,
) -> i32 {
    groundhog_syscall::syscall_wrapper(
        syscall_no, arg1, arg2, arg3, arg4, arg5, arg6, arg7,
    )
}

const K_SYSCALL_BYTESTRING_SET: SyscallWord =
    groundhog_xdr::SyscallNo::kBytestringSet.0 as SyscallWord;

pub struct ReadOnlyTestContract;

#[contract]
impl ReadOnlyTest for ReadOnlyTestContract {
    fn read_only_bytestring_set(expect_error: bool) {
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

    fn read_only_log_event(expect_error: bool) {
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

    fn write_storage_and_log_event() {
        let key: StorageKeyContractData = static_key!("rv32_parity_key");
        let storage_value = b"rv32 parity bytes";
        let topic0 = [0x5au8; 32];
        let event_data = b"rv32 parity event";

        sdk::sys_result::bytestring::bytestring_set(&key, storage_value)
            .unwrap();
        groundhog_xdr::syscalls::raw_LogEvent(
            Some(&topic0),
            None,
            None,
            event_data,
        )
        .unwrap();
    }

    fn read_only_native_get_balance(target_addr: &ContractAddress) {
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
                frame_status: InvocationFrameStatusCode::Return,
            },
        );
        assert!(res.is_ok(), "get_balance_of should succeed in read-only mode");
    }

    fn read_only_native_transfer(receiver_addr: &ContractAddress) {
        let flag = groundhog_xdr::invoke_flag::InvokeFlag::ReadOnly.0;
        let entrypoint =
            groundhog_erc20_if::ERC20Entrypoints::transfer.into_u64();
        // CXS-encode the arguments: (ContractAddress, (u64, u64))
        let calldata = cxs::encode(&(*receiver_addr, (0u64, 100u64))).unwrap();
        let mut outputs = sdk::sys::CopyableInvokeOutputs {
            return_buffer_len: 0,
            returned_auth_token_len: 0,
            frame_status: InvocationFrameStatusCode::Return,
        };
        let res = sdk::sys::raw_Invoke(
            &sdk::precompiles::ADDRESS_PRECOMPILED_HOG_TOKEN,
            &calldata,
            &[],
            &sdk::sys::CopyableInvokeInputs {
                entrypoint_id: entrypoint,
                gas_limit: u64::MAX,
                flags: flag,
            },
            &mut outputs,
        );
        // Transfer must not succeed in read-only mode: either the invoke is
        // rejected outright (Err), or the child frame faults when it attempts
        // a write (Ok with a non-Return frame_status).
        match res {
            Err(_) => {}
            Ok(()) => assert_ne!(
                outputs.frame_status,
                InvocationFrameStatusCode::Return,
                "transfer unexpectedly succeeded in read-only mode"
            ),
        }
    }

    fn read_only_deploy_output_preserved(
        callee_addr: &ContractAddress,
        contract_hash: &SHA256Hash,
    ) {
        // `callee_addr` is a *separate* deployment of this same contract: a
        // self-invoke would be rejected by the reentrance guard before the
        // read-only frame ever ran.  The inner method handles the deploy error
        // and returns normally, so this invoke is expected to succeed.
        Client::new(callee_addr)
            .with_read_only()
            .read_only_deploy_inner(contract_hash);
    }

    fn read_only_deploy_inner(contract_hash: &SHA256Hash) {
        let nonce = SHA256Hash::default();
        let mut deployed_address = ContractAddress::from([0xABu8; 32]);

        let res = sdk::sys::raw_DeployContract(
            &nonce, contract_hash, &mut deployed_address,
        );

        // `contract_hash` names a real, deployable contract, so the only reason
        // this can fail is the read-only guard -- which rejects the syscall
        // before it runs, and must leave the output untouched when it does.
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

    fn read_only_aliasing_before_read_only_check(
        callee_addr: &ContractAddress,
    ) {
        // As above, `callee_addr` is a separate deployment of this contract.
        // The inner method handles the syscall fault and returns normally.
        Client::new(callee_addr)
            .with_read_only()
            .read_only_aliasing_before_read_only_check_inner();
    }

    fn read_only_aliasing_before_read_only_check_inner()
    {
        let mut buf : [u8;64] = [0xAB;64];
        let key_ptr = buf.as_mut_ptr() as groundhog_xdr::syscalls::SyscallWord;
        let bytestring_ptr = (buf.as_mut_ptr() as groundhog_xdr::syscalls::SyscallWord) + 16;
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

        // The kFault above only demonstrates an *ordering* if this frame is
        // genuinely read-only: check that the same syscall, called with
        // non-aliased arguments, is rejected by the read-only guard.
        let key: StorageKeyContractData = static_key!("test");
        let res = sdk::sys_result::bytestring::bytestring_set(&key, b"test");
        assert!(res.err().unwrap().is_code(
            SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput
        ), "frame was not read-only, so the fault above proves no ordering");
    }


}
