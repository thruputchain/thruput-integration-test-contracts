use sdk::*;

#[contract(client = Client)]
pub trait ReturnFaultTest {
    fn run_test(&self);

    fn test_inner(&self);
}

