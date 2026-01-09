# Payment Channel Contract

A Lightning-style payment channel contract that enables fast, off-chain payments between two parties.

## Overview

Payment channels allow two parties to exchange funds off-chain by signing state updates, with on-chain settlement only required when opening or closing the channel. This dramatically reduces transaction costs and latency for repeated payments between the same parties.

## Methods

| Method | Description |
|--------|-------------|
| `initialize(dispute_period)` | Initialize contract storage with default dispute period |
| `open_channel(party_a_pubkey, party_b_pubkey, party_b_dest, token, amount, salt)` -> `u64` | Create a new channel, returns channel ID |
| `join_channel(channel_id, amount)` | Party B joins with their deposit |
| `deposit(channel_id, amount)` | Either party can add more funds |
| `close_channel(channel_id, balance_a, balance_b, nonce, sig_a, sig_b)` | Submit signed state to initiate closure (starts dispute period) |
| `dispute(channel_id, balance_a, balance_b, nonce, sig_a, sig_b)` | Challenge with a newer signed state (higher nonce) |
| `finalize(channel_id)` | After dispute period, distribute funds |
| `get_dispute_period(channel_id)` -> `u64` | Returns dispute period in blocks for a channel |

## Lifecycle

### 1. Opening a Channel

Party A opens a channel by calling `open_channel` with:
- Both parties' Ed25519 public keys (for signature verification)
- Party B's destination address (where B's funds go on close)
- The ERC20 token contract address
- Party A's initial deposit amount
- A salt value (allows same parties to have multiple channels)

Party A's destination address is automatically set to the caller's address.

**Important**: Before calling `open_channel`, Party A must approve the payment channel contract to spend their tokens via the ERC20 `adjust_allowance` method.

```
// First, approve the payment channel to spend tokens
erc20.adjust_allowance(payment_channel_address, deposit_amount)

// Then open the channel
channel_id = open_channel(
    party_a_pubkey,
    party_b_pubkey,
    party_b_dest,
    token,
    deposit_amount,
    salt
)
```

The channel ID is computed deterministically as:
```
channel_id = hash(caller, party_a_pubkey, party_b_pubkey, token, salt)[0..8] as u64
```

### 2. Joining the Channel

Party B joins by calling `join_channel` with their deposit. They must first approve the payment channel contract to spend their tokens.

```
// First, approve the payment channel to spend tokens
erc20.adjust_allowance(payment_channel_address, deposit_amount)

// Then join the channel
join_channel(channel_id, deposit_amount)
```

### 3. Off-Chain State Updates

Parties exchange signed state updates off-chain. Each state update contains:
- `channel_id`: The channel being updated
- `balance_a`: Party A's balance
- `balance_b`: Party B's balance
- `nonce`: Incrementing counter (higher = newer)

The message format for signing is the CXS-serialized `ChannelState` struct:
```rust
struct ChannelState {
    channel_id: u64,
    balance_a: i128,
    balance_b: i128,
    nonce: u64,
}
```

Both parties sign each state update. The sum of balances must equal total deposits.

### 4. Closing the Channel

Either party can close by submitting the latest signed state:

```
close_channel(channel_id, balance_a, balance_b, nonce, sig_a, sig_b)
```

This starts the dispute period (configured at initialization).

### 5. Disputing (Optional)

During the dispute period, either party can challenge with a newer state (higher nonce):

```
dispute(channel_id, balance_a, balance_b, higher_nonce, sig_a, sig_b)
```

This updates the final balances but doesn't extend the dispute deadline.

### 6. Finalizing

After the dispute period ends, anyone can finalize the channel:

```
finalize(channel_id)
```

This transfers the final balances to each party's destination address and deletes the channel.

## Security Properties

- **Double-spend protection**: Higher nonces always take precedence during disputes
- **Balanced funds**: Submitted balances must equal total deposits (no inflation/deflation)
- **Signature verification**: Both parties must sign every state update
- **Atomic finalization**: Uses `delete_exclusive` to prevent double-finalization
- **Time-locked disputes**: Configurable dispute period gives parties time to challenge fraud
- **Deterministic channel IDs**: Hash-based IDs prevent contention when opening multiple channels

## Example Usage

```rust
// Initialize with 100-block dispute period
initialize(100);

// Alice approves and opens a channel with Bob, depositing 1000 tokens
erc20.adjust_allowance(payment_channel, 1000);
let channel_id = open_channel(
    alice_pubkey,
    bob_pubkey,
    bob_address,
    usdc_token,
    1000,
    0  // salt
);

// Bob approves and joins with 500 tokens (total capacity: 1500)
erc20.adjust_allowance(payment_channel, 500);
join_channel(channel_id, 500);

// Off-chain: Alice pays Bob 100 tokens
// They sign state: (channel_id, 900, 600, 1)

// Off-chain: Bob pays Alice 50 tokens
// They sign state: (channel_id, 950, 550, 2)

// Alice closes with final state
close_channel(channel_id, 950, 550, 2, alice_sig, bob_sig);

// Wait for dispute period...

// Anyone can finalize
finalize(channel_id);
// Alice receives 950 tokens, Bob receives 550 tokens
```

## Storage

The contract stores:
- `channels`: Map from channel ID to Channel struct
- `default_dispute_period`: The dispute period used for new channels

Each Channel contains:
- Party public keys and destination addresses
- Deposit amounts
- Token contract address
- Join status
- Per-channel dispute period
- Submitted close state (balances, nonce, dispute deadline)
