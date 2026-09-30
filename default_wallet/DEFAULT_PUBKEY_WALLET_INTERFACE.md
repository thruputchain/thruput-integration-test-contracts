# Default Pubkey Wallet Interface

## Summary

Every `kPubkeySignature` account should have a default smart wallet.

The desired flow is:

```text
create keypair -> derive pubkey wallet address -> execute calls or deploy contracts
```

No deploy step. No `create_wallet` flow. No app-specific wallet setup.

## Interface

```rust
#[contract(client = DefaultWalletClient, throws = WalletError)]
pub trait DefaultWallet {
    fn execute(calls: Vec<WalletCall>) -> Vec<Bytes>;
    fn deploy(
        nonce: SHA256Hash,
        contract_hash: SHA256Hash,
        amount: i128,
        initializer: Option<WalletInitCall>,
    ) -> ContractAddress;
}

#[contract_type]
pub struct WalletCall {
    pub target: ContractAddress,
    pub entrypoint: ContractEntrypointID,
    pub gas_limit: u64,
    pub calldata: Bytes,
}

#[contract_type]
pub struct WalletInitCall {
    pub entrypoint: ContractEntrypointID,
    pub gas_limit: u64,
    pub calldata: Bytes,
}

#[contract_error]
pub enum WalletError {
    NotRootInvocation = 1,
    InvalidAuthClaim = 2,
    EmptyCallList = 3,
    TooManyCalls = 4,
    CallAborted = 5,
    CallFaulted = 6,
    ReturnDataTooLarge = 7,
}
```

## Semantics

### `execute(calls)`

Executes one or more contract calls from the wallet, in order, inside one transaction.

- Must be the transaction root invocation.
- Must be authorized by `kPubkeySignature` for this wallet address.
- Returns one raw return-data item per call, in call order.
- V1 should be all-or-nothing: if any call aborts or faults, the wallet aborts and no state changes commit.

Single-call usage is `execute([call])`. Multicall usage is `execute([call_a, call_b, call_c])`.

Return values are ordered the same way: `execute([call_a, call_b, call_c]) -> [return_a, return_b, return_c]`.

Do not include partial-success or allow-failure behavior in v1 unless there is a concrete app requirement.

### `deploy(nonce, contract_hash, amount, initializer)`

Funds, deploys and optionally initializes a contract from the wallet address in
one transaction.

- Must be the transaction root invocation.
- Must be authorized by `kPubkeySignature` for this wallet address.
- Uses the wallet address as deployer.
- An amount greater than zero transfers exactly that many raw HOG units to the
  predicted address before deployment. Zero skips the token call entirely; the
  target must already have any required collateral.
- The amount is caller-selected funding, not a storage-cost estimate. It must
  cover deployment and any initial state created by the initializer.
- An optional initializer is an ordinary method call on the newly deployed
  contract. Its caller is the wallet; no auth tokens are forwarded. Its entrypoint,
  calldata and gas limit are explicit, with no constructor-name convention.
- Returns only the deployed contract address; initializer return data is discarded.
- A funding, deployment or initialization failure rolls back all application
  state and success events. Normal transaction replay bookkeeping still applies.
- Initializer aborts report `CallAborted`; invocation failures report `CallFaulted`,
  as for `execute`. Contracts must enforce their own initialization policy on
  later calls; the wallet does not make a method callable only once.
- The outer transaction still carries `contracts_to_deploy` when needed.

Including `deploy` makes the default wallet complete: a user can create a keypair, execute one call, execute multiple calls atomically, and deploy contracts.

## Auth Rule

The transaction auth claim must be `kPubkeySignature`, and its `PubkeyAccountPreimage` must derive the invoked wallet address.

```text
root invoked address == derive_pubkey_wallet_address(auth_claim.pubkey_preimage)
```

The protocol verifies the signature before execution. The wallet should either verify that the auth claim is for itself or rely on this as an admission invariant if the protocol guarantees it.

The default wallet is intentionally single-key. Key management, multisig, recovery, and custom policy should live in advanced wallet contracts, not this default interface.

## SDK Shape

High-level app code should look like this:

```ts
const wallet = defaultWallet(addressFromPublicKey(publicKey));

const tx = await wallet.execute([
  token.transfer({ to: pool, amount }),
  pool.deposit({ amount }),
]);

const signed = await signer.signTransaction(tx);
await client.sendTransaction(signed);
```

SDKs should expose both the high-level wallet helper and the raw `WalletCall` / invocation builders so tools and advanced apps can construct the same transaction manually.

## Upgradability

Upgradability is valuable but separate from this default wallet interface.

The desired long-term model is:

```text
pubkey address starts with default wallet behavior
later, user installs custom wallet code at the same address
address, balances, identity, and history stay the same
```

That would let users start with a keypair and later upgrade to a richer wallet with multisig, recovery, spending limits, sponsorship policy, or app-specific controls.

This should be handled as an address-lifecycle or protocol feature, separate from the default wallet interface.

## V1 Implementation Decisions

- `execute` allows up to 16 child calls.
- Aggregate raw child return data is capped at the protocol return-buffer limit.
- Each `WalletCall` carries an explicit child-call gas limit.
- Forwarded auth tokens are not supported in v1.

## Implementation Notes

The Rust contract implementation represents `Bytes` with the SDK's concrete XDR bytes type and uses `ContractEntrypointID` directly in `WalletCall`.
