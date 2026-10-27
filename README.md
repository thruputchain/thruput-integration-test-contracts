# Thruput integration test contracts

> **These are not example contracts.** Do not copy them as a starting point.
>
> They are edge-case integration tests between the Thruput node and the
> [Thruput Rust SDK](https://github.com/thruputchain/thruput-sdk-rs). Many
> of them deliberately do what a real contract never should: issue raw
> syscalls with malformed arguments, overrun buffer limits, fault or revert on
> purpose, or bypass the SDK's safe wrappers. For how to write a contract,
> start with the SDK's README.

## Purpose

Each contract pins down how the node and the SDK behave at their boundary,
in a case that ordinary contracts rarely reach. The cases include:

- syscall error paths and argument validation;
- calldata, return-buffer and event size limits;
- faults, reverts and aborts, and how they propagate across nested calls;
- auth claims, auth tokens and account wallets;
- commit-lock conflicts and read-only invocation;
- gas accounting at exact boundaries;
- XDR and CXS encoding round trips.

To see what a contract covers, read its source alongside the tests that
drive it. The contract names say which area each one targets.

## How the contracts are used

This repository has no Cargo workspace of its own. Two repositories include
it and build it inside theirs:

- **thruput-sdk-rs**, as the `integration_test/contracts` submodule. The
  SDK's `integration_test` workspace builds the contracts, runs the native
  tests some of them carry under `tests/`, and builds their rv32 guest
  artifacts.
- **The Thruput node**, whose test suites load the rv32 artifacts and drive
  them through the real executor. Most of the contracts are tested this way
  rather than by their own `tests/`.

## Layout

Each directory is one crate:

- `src/interface.rs`: the contract trait, and the generated client;
- `src/implementation.rs`: the `#[contract]` implementation;
- `src/main.rs`: the rv32 binary wrapper. See `fibonacci/src/main.rs` for
  why it exists;
- `tests/`: native tests, where present.

A few small crates keep the whole contract in `src/lib.rs`. The `*_raw`
crates call syscalls directly instead of through the SDK's wrappers.

A `disabled` file in a crate tells the SDK's rv32 build loop to skip that
crate. The file says why. The node may still build the crate.

`xdr/roundtrip.x` holds XDR definitions that the node generates code from
for its round-trip tests.

## Building and running

From a thruput-sdk-rs checkout:

```sh
git submodule update --init integration_test/contracts
./scripts/dev.sh test-itc                     # native tests, all crates
(cd integration_test && cargo test -p default_wallet)   # one crate
```

Building the rv32 artifacts needs the `thruput` CLI; see the SDK's README.

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE).
