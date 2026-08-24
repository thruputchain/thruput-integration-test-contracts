# integration_test_contracts

RV32 test and example contracts for the Groundhog L1. These crates are
exercised by hogd's execution-layer tests (`hogd/hog_exec/...`), the
devnode genesis/bootstrap path (`dev_wallet`, `genesis_root`), and the
serialization roundtrip tests (`cxs_roundtrip`, `txs_roundtrip`).

## This repo does not build standalone

There is no root `Cargo.toml` here. The cargo workspace root lives in the
hogd repository at `built_contracts/`, which vendors this repo as the git
submodule `built_contracts/integration_test_contracts`. Everything below
therefore refers to paths inside a hogd checkout.

Two crates additionally depend on a CMake-generated, gitignored source
file: `cxs_roundtrip/src/roundtrip.rs` and `txs_roundtrip/src/roundtrip.rs`
are generated from `xdr/roundtrip.x` by the `add_xdr_rs` rules in
`built_contracts/CMakeLists.txt`. Until a hogd CMake build has run, those
two crates do not compile at all.

## Toolchain prerequisites

- The `groundhog` CLI, exactly version `groundhog 0.9.0`.
  `built_contracts/build_risc.sh` hard-gates on `groundhog --version` and
  refuses to build otherwise. hogd's `scripts/fetch.sh` installs it
  (`cargo +1.96.0 install --locked --registry groundhog groundhog-cli@0.9.0`)
  along with `groundhog-rv32-lower@0.9.0`.
- The pinned Rust toolchain in `built_contracts/rust-toolchain.toml`
  (currently `nightly-2026-05-01` with `rust-src`) for the
  `riscv32im-unknown-none-elf` build-std target.
- Registry credentials for the private `groundhog` registry
  (`CARGO_REGISTRIES_GROUNDHOG_TOKEN`), as described in hogd's README.

## How the contracts are built

hogd's CMake configure step (see hogd `README.md`; `./scripts/build.sh -n`
then build the tree) wires up `built_contracts/CMakeLists.txt`, which
defines:

- `build_all_contracts` — the top-level `ALL` target; depends on the two
  below.
- `all_riscv_contracts` / `all_riscvs_built` — builds every contract via
  `built_contracts/build_risc.sh <name> <build-dir>`, which runs
  `groundhog build -p <name> --target rv32 -- --frozen --offline` and
  copies out `<name>.rv32a`, `<name>.rv32.json`, and (for spec contracts)
  `<name>_spec.json`.
- `all_contract_headers` / per-contract `<name>_header` — generates a C++
  driver header from `<name>_spec.json` via `generate_header.py`. Contracts
  with custom XDR types additionally get `<name>.x` from
  `generate_types_xdr.py` and `<name>.hh` via the `add_xdr` (x23c) rules.
- `build_roundtrip_rs` — the `xdr/roundtrip.x` → `src/roundtrip.rs` codegen
  the roundtrip crates need (see above).
- `contract_loader` / `contract_loader_test` — the C++ library hogd links
  to load the built `.rv32a` artifacts, plus its gtest.

`build_risc.sh` builds with `--frozen --offline`, so dependency changes
also require updating `built_contracts/Cargo.lock`.

Not everything is lowered to RV32: `calldata_raw`, `sha256_raw`, and
`fibonacci_raw` have no `#[contract]` macro or `[[bin]]` wrapper (no RV32
entry point), and `nocontract_wallet` is a client-stub-only crate that is
not built for either target. `hash_bug` is built as a RAW contract
(`.rv32a` without a spec).

## `disabled` markers

A `disabled` file in a crate directory is honored only by sdk_rs-side
scripts that iterate over contract crates; each file records why the crate
is opted out. hogd's CMake build ignores the marker entirely — it builds
every contract listed in `built_contracts/CMakeLists.txt`, including
marked ones.

## Host-side type checking and tests

The crates are `no_std` for the RV32 target. Host builds (for native tests
and IDE checks through the `built_contracts` workspace) need the `std`
feature (`cargo check -p <name> --features std`); the `dev` feature (used
by each crate's self dev-dependency) additionally enables the sdk's native
mock execution environment, which is what `cargo test -p <name>` builds
against for crates with `tests/` (e.g. `default_wallet`, `genesis_root`).
Running those tests needs the sdk's prebuilt `test-ffi` library, which the
sdk's build script downloads from groundhog.dev (credentials required).

## Adding a new contract

1. Create the crate directory here: `Cargo.toml` (copy an existing
   contract's feature layout: `entrypoints`, `dev`, `std`, `logging`, the
   self dev-dependency, and the `[[bin]]` wrapper with
   `required-features = ["entrypoints"]`) plus
   `src/{lib,interface,implementation,main}.rs`.
2. Add the crate to the workspace `members` list in
   `built_contracts/Cargo.toml` (hogd repo) and refresh
   `built_contracts/Cargo.lock` (the build runs `--frozen --offline`).
3. Add the crate name to `add_riscv_builds(SPEC_CONTRACTS ...)` in
   `built_contracts/CMakeLists.txt` (or `RAW_CONTRACTS` if it has no
   `#[contract]` spec), and add an `add_contract_header(<name>)` entry —
   plus `add_types_xdr(<name>)` / `add_xdr(<name>_hh ...)` if it defines
   custom XDR types.
4. Add a hogd-side driver that consumes the generated header, e.g. a test
   under `hogd/hog_exec/`, loading the artifact through `contract_loader`.
5. Commit the submodule bump in hogd once the change lands here.

## License

Apache-2.0 (see `LICENSE`).
