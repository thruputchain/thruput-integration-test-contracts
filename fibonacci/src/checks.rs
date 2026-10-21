//! Checks shared by the native unit tests (`src/tests.rs`) and the rv32
//! test (`tests/rv32.rs`), so the mock and the rv32 build are held to the
//! same assertions.

use crate::Client;

pub fn check_fibonacci(client: &Client) {
    assert_eq!(client.fibonacci(0), 0);
    assert_eq!(client.fibonacci(1), 1);
    assert_eq!(client.fibonacci(2), 1);
    assert_eq!(client.fibonacci(3), 2);
    assert_eq!(client.fibonacci(4), 3);
    assert_eq!(client.fibonacci(5), 5);
    assert_eq!(client.fibonacci(6), 8);

    assert_eq!(client.slow_fibonacci(0), 0);
    assert_eq!(client.slow_fibonacci(1), 1);
    assert_eq!(client.slow_fibonacci(2), 1);
    assert_eq!(client.slow_fibonacci(3), 2);
    assert_eq!(client.slow_fibonacci(4), 3);
    assert_eq!(client.slow_fibonacci(5), 5);
    assert_eq!(client.slow_fibonacci(6), 8);

    assert_eq!(client.fibonacci(0), 0);
    assert_eq!(client.fibonacci(1), 1);
    assert_eq!(client.fibonacci(2), 1);
    assert_eq!(client.fibonacci(3), 2);
    assert_eq!(client.fibonacci(4), 3);
    assert_eq!(client.fibonacci(5), 5);
    assert_eq!(client.fibonacci(6), 8);

    assert_eq!(client.slow_fibonacci(0), 0);
    assert_eq!(client.slow_fibonacci(1), 1);
    assert_eq!(client.slow_fibonacci(2), 1);
    assert_eq!(client.slow_fibonacci(3), 2);
    assert_eq!(client.slow_fibonacci(4), 3);
    assert_eq!(client.slow_fibonacci(5), 5);
    assert_eq!(client.slow_fibonacci(6), 8);

    assert_eq!(client.fibonacci(10), client.slow_fibonacci(10));
    assert_eq!(client.fibonacci(25), client.slow_fibonacci(25));
}
