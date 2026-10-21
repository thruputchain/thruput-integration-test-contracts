//! Checks shared by the native unit tests (`src/tests.rs`) and the rv32
//! test (`tests/rv32.rs`), so the mock and the rv32 build are held to the
//! same assertions.

use crate::Client;

pub fn check_echo(client: &Client) {
    let data = alloc::vec![1, 2, 3, 4, 5];
    let echo = client.echo(&data);

    assert_eq!(echo, data);
}
