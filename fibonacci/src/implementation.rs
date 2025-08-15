//! An implementation of Fibonacci

use super::interface::Fibonacci;
use sdk::*;

#[derive(Default)]
struct FibonacciContract;


// This implements the Fibonacci interface.  The `contract` macro provides
// an exported entrypoint for each method; these will be placed in a
// module with the specified name ("guest_exports").
#[contract(exports = "guest_exports", storage = "false")]
impl Fibonacci for FibonacciContract {
    fn fibonacci(&self, n: u32) -> u32 {
        let mut a = 0;
        let mut b = 1;
        for _ in 0..n {
            let c = a + b;
            a = b;
            b = c;
        }
        a
    }

    fn slow_fibonacci(&self, n: u32) -> u32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        self.slow_fibonacci(n - 1) + self.slow_fibonacci(n - 2)
    }
}

#[unsafe(export_name = "FFFFFFFFFFFFFFFF")]
pub extern "C" fn startup_null() -> () {
    ()
}
