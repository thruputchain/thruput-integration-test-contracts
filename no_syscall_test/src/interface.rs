use sdk::*;

#[contract]
pub trait NoSyscallTest {
    /// Make a bad syscall
    fn run_test(syscall: u64) -> ();
}

