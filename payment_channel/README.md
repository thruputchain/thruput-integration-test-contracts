# Payment Channel Contract

A Lightning-style payment channel contract that enables fast, off-chain payments between two parties.

## Overview

Payment channels allow two parties to exchange funds off-chain by signing state updates, with on-chain settlement only required when opening or closing the channel. This dramatically reduces transaction costs and latency for repeated payments between the same parties.

## Methods

| Method | Description |
|--------|-------------|
| `initialize()` | Initialize contract storage |
| `open_channel(party_a_pubkey, party_a_dest, party_b_pubkey, party_b_dest, token, amount)` → `u64` | Create a new channel, returns channel ID |
| `join_channel(channel_id, amount)` | Party B joins with their deposit |
| `deposit(channel_id, amount)` | Either party can add more funds |
| `close_channel(channel_id, balance_a, balance_b, nonce, sig_a, sig_b)` | Submit signed state to initiate closure (starts dispute period) |
| `dispute(channel_id, balance_a, balance_b, nonce, sig_a, sig_b)` | Challenge with a newer signed state (higher nonce) |
| `finalize(channel_id)` | After dispute period, distribute funds |
| `get_dispute_period()` → `u64` | Returns dispute period in blocks (currently 100) |

## Lifecycle

### 1. Opening a Channel

Party A opens a channel by calling `open_channel` with:
- Both parties' Ed25519 public keys (for signature verification)
- Both parties' destination addresses (where funds go on close)
- The ERC20 token contract address
- Party A's initial deposit amount

```
channel_id = open_channel(
    party_a_pubkey,
    party_a_dest,
    party_b_pubkey,
    party_b_dest,
    token,
    deposit_amount
)
```

### 2. Joining the Channel

Party B joins by calling `join_channel` with their deposit:

```
join_channel(channel_id, deposit_amount)
```

### 3. Off-Chain State Updates

Parties exchange signed state updates off-chain. Each state update contains:
- `channel_id`: The channel being updated
- `balance_a`: Party A's balance
- `balance_b`: Party B's balance
- `nonce`: Incrementing counter (higher = newer)

The message format for signing is:
```
channel_id (8 bytes BE) || balance_a (16 bytes BE) || balance_b (16 bytes BE) || nonce (8 bytes BE)
```

Both parties sign each state update. The sum of balances must equal total deposits.

### 4. Closing the Channel

Either party can close by submitting the latest signed state:

```
close_channel(channel_id, balance_a, balance_b, nonce, sig_a, sig_b)
```

This starts a dispute period (100 blocks by default).

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
- **Time-locked disputes**: 100-block dispute period gives parties time to challenge fraud

## Example Usage

```rust
// Alice opens a channel with Bob, depositing 1000 tokens
let channel_id = open_channel(
    alice_pubkey,
    alice_address,
    bob_pubkey,
    bob_address,
    usdc_token,
    1000
);

// Bob joins with 500 tokens (total capacity: 1500)
join_channel(channel_id, 500);

// Off-chain: Alice pays Bob 100 tokens
// They sign state: (channel_id, 900, 600, 1)

// Off-chain: Bob pays Alice 50 tokens
// They sign state: (channel_id, 950, 550, 2)

// Alice closes with final state
close_channel(channel_id, 950, 550, 2, alice_sig, bob_sig);

// Wait 100 blocks...

// Anyone can finalize
finalize(channel_id);
// Alice receives 950 tokens, Bob receives 550 tokens
```

## Storage

The contract stores:
- `channels`: Map from channel ID to Channel struct
- `next_channel_id`: Counter for generating unique channel IDs

Each Channel contains:
- Party public keys and destination addresses
- Deposit amounts
- Token contract address
- Join status
- Submitted close state (balances, nonce, dispute deadline)
