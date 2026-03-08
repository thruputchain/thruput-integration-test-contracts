use crate::{Client, FibonacciContract};
use sdk::*;

client_test! { FibonacciContract =>
    fn test_fibonacci(client: &mut Client) {
        test_fibonacci_inner(client)
    }
}

fn test_fibonacci_inner(client: &mut Client) {
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
    assert_eq!(client.fibonacci(40), client.slow_fibonacci(40));
}
