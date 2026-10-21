use crate::checks::check_fibonacci;

#[test]
fn test_fibonacci() {
    thruput_sdk::client_test(
        |env| env.register_mock(<crate::FibonacciContract as crate::Fibonacci>::__mock),
        check_fibonacci,
    )
}
