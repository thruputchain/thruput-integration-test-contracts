use sdk::*;

#[contract(client = Client)]
pub trait TupleReturn {
    fn call1(&self, n: u32) -> (u32, u32);
    fn call2(&self, n: u32) -> (u32, (u64, u64));
}
