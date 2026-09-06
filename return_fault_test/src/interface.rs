use sdk::*;

#[contract(client = Client)]
pub trait ReturnFaultTest {
    /// `callee_addr` is a separate deployment of this contract, so the inner
    /// call actually runs. Pointing it at this contract's own address makes
    /// the invoke reentrant, which the runtime refuses before any child frame
    /// starts -- the inner fault this exercises would never happen.
    fn run_test(callee_addr: &ContractAddress);

    fn test_inner();
}

