use sdk::*;

#[contract(client = Client)]
pub trait ReturnFaultTest {
    fn run_test();

    fn test_inner();
}

