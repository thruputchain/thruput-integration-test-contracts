#![no_std]

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use sdk::*;

pub fn sha256_calldata() {
    let len = get_calldata_len();
    let mut buf: Vec<u8> = vec![0; len as usize];
    let mut written_len = 0;
    let mut auth_token_count = 0;

    sdk::sys::raw_GetCalldata(buf.as_mut_slice(), &mut written_len, &mut auth_token_count).unwrap();

    let mut hash: SHA256Hash = SHA256Hash::default();

    sdk::sys::raw_SHA256(buf.as_slice(), &mut hash).unwrap();
}

// Raw entrypoint, registered without the `#[contract]` macro.
//
// rv32 dispatches guest-side: the macro normally emits one `Entrypoint`
// record per method into the `groundhog_rv32_entrypoints` section, which the
// SDK's `_start` dispatcher scans. Hand-writing the record registers this
// benchmark entrypoint outside the macro's calldata/Return glue, which is
// the point of this contract — it measures the raw syscalls, not the wrapper
// around them.
//
// The dispatcher traps (ebreak) when a wrapper returns, so the wrapper calls
// `raw_Return` itself; an empty buffer is the equivalent of the bare return
// this entrypoint used to do.
#[cfg(target_arch = "riscv32")]
mod rv32_entries {
    use sdk::__rv32::Entrypoint;

    extern "C" fn sha256_calldata_raw() {
        super::sha256_calldata();
        sdk::sys::raw_Return(&[], false, &[])
    }

    #[used]
    #[unsafe(link_section = "groundhog_rv32_entrypoints")]
    static SHA256_CALLDATA: Entrypoint = Entrypoint {
        method_id: 0x0,
        wrapper: sha256_calldata_raw,
    };
}
