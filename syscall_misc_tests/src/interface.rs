use sdk::*;

#[contract(client = Client)]
pub trait SyscallMiscTests {
    fn test_get_return_before_return_buffer_set(&self);
    fn test_get_return_after_fault(&self);


    // test methods
    fn internal_always_fault(&self);
}
