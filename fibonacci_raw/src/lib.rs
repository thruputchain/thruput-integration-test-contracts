#![no_std]

#[allow(unused_imports)]
use sdk::*;

pub fn fibonacci_slow(count: u32) -> u32 {
    if count == 0 {
        return 0;
    }
    if count == 1 {
        return 1;
    }
    fibonacci_slow(count - 1) + fibonacci_slow(count - 2)
}

// Raw entrypoints, registered without the `#[contract]` macro.
//
// rv32 dispatches guest-side: the macro normally emits one `Entrypoint`
// record per method into the `groundhog_rv32_entrypoints` section, which the
// SDK's `_start` dispatcher scans. Hand-writing the records registers these
// benchmark entrypoints outside the macro's calldata/Return glue, which is
// the point of this contract — it measures the recursion, not the wrapper
// around it.
//
// The dispatcher traps (ebreak) when a wrapper returns, so every wrapper
// calls `raw_Return` itself; an empty buffer is the equivalent of the bare
// return these entrypoints used to do.
#[cfg(target_arch = "riscv32")]
mod rv32_entries {
    use sdk::__rv32::Entrypoint;

    fn done() -> ! {
        sdk::sys::raw_Return(&[], false, &[])
    }

    macro_rules! raw_entrypoint {
        ($static_name:ident, $wrapper:ident, $method_id:expr, $n:expr) => {
            extern "C" fn $wrapper() {
                super::fibonacci_slow($n);
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

    raw_entrypoint!(FIB_1, fibonacci_slow_raw_1, 0x1, 1);
    raw_entrypoint!(FIB_5, fibonacci_slow_raw_5, 0x5, 5);
    raw_entrypoint!(FIB_10, fibonacci_slow_raw_10, 0xA, 10);
    raw_entrypoint!(FIB_15, fibonacci_slow_raw_15, 0xF, 15);
    raw_entrypoint!(FIB_20, fibonacci_slow_raw_20, 0x14, 20);
    raw_entrypoint!(FIB_25, fibonacci_slow_raw_25, 0x19, 25);
}
