#![no_std]

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use sdk::{fetch_calldata_and_auth_tokens, get_calldata_len};

pub fn calldata_len(count: u32) {
    for _ in 0..count {
        let _ = get_calldata_len();
    }
}

pub fn calldata(count: u32) {
    let mut len: usize = 0;

    for _ in 0..count {
        // Fresh buffers per iteration: each fetch is independent, matching
        // the pre-existing semantics of one fetch per call.
        let mut buf = sdk::internal::new_scratch();
        let mut auth_ids = sdk::contract_op::AuthTokenIdBuf::new();
        fetch_calldata_and_auth_tokens(&mut buf, &mut auth_ids);
        let x_len = buf.len();
        if len > 0 {
            assert_eq!(x_len, len);
        }
        len = x_len;
    }
}

pub fn calldata_raw(count: u32) {
    let len = get_calldata_len();
    let mut buf: Vec<u8> = vec![0; len as usize];
    let mut written_len = 0;
    let mut auth_token_count = 0;
    for _ in 0..count {
        sdk::sys::raw_GetCalldata(buf.as_mut_slice(), &mut written_len, &mut [], &mut auth_token_count)
            .unwrap();
    }
}

// Raw entrypoints, registered without the `#[contract]` macro.
//
// rv32 dispatches guest-side: the macro normally emits one `Entrypoint`
// record per method into the `groundhog_rv32_entrypoints` section, which the
// SDK's `_start` dispatcher scans. Hand-writing the records registers these
// benchmark entrypoints outside the macro's calldata/Return glue, which is
// the whole point of this contract — it measures the raw syscalls, not the
// wrapper around them.
//
// The dispatcher traps (ebreak) when a wrapper returns, so every wrapper
// must call `raw_Return` itself; an empty buffer is the equivalent of the
// bare return these entrypoints used to do.
#[cfg(target_arch = "riscv32")]
mod rv32_entries {
    use sdk::__rv32::Entrypoint;

    fn done() -> ! {
        sdk::sys::raw_Return(&[], false, &[])
    }

    macro_rules! raw_entrypoint {
        ($static_name:ident, $wrapper:ident, $method_id:expr, $body:expr) => {
            extern "C" fn $wrapper() {
                $body;
                done()
            }

            #[used]
            #[unsafe(link_section = "groundhog_rv32_entrypoints")]
            static $static_name: Entrypoint = Entrypoint {
                method_id: $method_id,
                wrapper: $wrapper,
            };
        };
    }

    raw_entrypoint!(CALLDATA_LEN_1, calldata_len_raw_1, 0x0, super::calldata_len(1));
    raw_entrypoint!(CALLDATA_LEN_100, calldata_len_raw_100, 0x1, super::calldata_len(100));
    raw_entrypoint!(CALLDATA_LEN_1000, calldata_len_raw_1000, 0x2, super::calldata_len(1000));

    raw_entrypoint!(CALLDATA_1, calldata_1, 0x10, super::calldata(1));
    raw_entrypoint!(CALLDATA_100, calldata_100, 0x11, super::calldata(100));
    raw_entrypoint!(CALLDATA_1000, calldata_1000, 0x12, super::calldata(1000));

    raw_entrypoint!(CALLDATA_RAW_1, calldata_raw_1, 0x20, super::calldata_raw(1));
    raw_entrypoint!(CALLDATA_RAW_100, calldata_raw_100, 0x21, super::calldata_raw(100));
    raw_entrypoint!(CALLDATA_RAW_1000, calldata_raw_1000, 0x22, super::calldata_raw(1000));
}
