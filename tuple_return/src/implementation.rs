//! An implementation of Fibonacci

use super::interface::TupleReturn;
use sdk::*;

#[derive(Default)]
struct TupleReturnContract;

// Check to make sure specs are properly generated for tuple return types.
#[contract(exports = "guest_exports", storage = "false")]
impl TupleReturn for TupleReturnContract {
    fn call1(&self, _n: u32) -> (u32, u32) {
        (1,2)
    }

    fn call2(&self, _n: u32) -> (u32, (u64, u64)) {
        (3, (4, 5))
    }
}
