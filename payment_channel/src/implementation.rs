use super::interface::*;
use alloc::vec::Vec;
use sdk::crypto::verify_signature_ed25519;
use sdk::storage::{Deletable, Map, Value};
use sdk::*;

use thruput_erc20_if::ERC20Client;

use sdk::WriteXDR;

// Payment Channel Contract
//
// Token deposits are handled via ERC20 transfer_from. Callers must first approve
// this contract to spend their tokens before calling open_channel, join_channel,
// or deposit. The contract pulls tokens atomically during these operations.
//
// Channel states:
// - Open: Channel is active, parties can deposit and exchange off-chain state updates
// - Closing: A close has been initiated, dispute period is active
// - (Deleted): After finalization, the channel entry is removed from storage
//
// The nonce in submitted state must be > 0 (nonce 0 means no state submitted yet)

#[derive(WriteXDR, ReadXDR)]
#[xdrs(crate = "::sdk::__xdrs")]
struct Channel {
    // Party A (channel opener)
    party_a_pubkey: Ed25519PublicKey,
    party_a_dest: ContractAddress,
    deposit_a: i128,

    // Party B (channel joiner)
    party_b_pubkey: Ed25519PublicKey,
    party_b_dest: ContractAddress,
    deposit_b: i128,

    // Token being used
    token: ContractAddress,

    // Whether party B has joined
    joined: bool,

    // Per-channel dispute period in blocks
    dispute_period: u64,

    // Submitted close state (nonce > 0 means closing)
    submitted_balance_a: i128,
    submitted_balance_b: i128,
    submitted_nonce: u64,
    dispute_deadline: i64, // BlockHeight
    // TODO(geoff): convert to BlockHeight, once we add arithmetic ops on BlockHeight
}

impl Channel {
    pub fn token_client(&self) -> ERC20Client {
        ERC20Client::new(&self.token)
    }

    pub fn total_deposit(&self) -> i128 {
        // Checked so a corrupted or overflowing pair fails loudly rather than
        // wrapping the total that close/dispute settlement is checked against.
        self.deposit_a
            .checked_add(self.deposit_b)
            .expect("channel total deposit overflow")
    }

    pub fn is_closing(&self) -> bool {
        self.submitted_nonce > 0
    }
}

#[storage]
pub struct PaymentChannelContract {
    channels: Map<u64, Value<Channel>>,
    default_dispute_period: Value<u64>,
}

fn get_block_height() -> i64 {
    let mut height: BlockHeight = 0.into();
    sdk::sys::raw_GetBlockHeight(&mut height).unwrap();
    height.into_inner()
}

// Serialize ChannelState to bytes for signing
fn serialize_state(state: &ChannelState) -> Vec<u8> {
    encode_xdr(state).unwrap()
}

#[contract]
impl PaymentChannel for PaymentChannelContract {
    fn initialize(&mut self, dispute_period: u64) {
        assert!(dispute_period > 0, "Dispute period must be positive");
        self.default_dispute_period.set(&dispute_period);
    }

    fn open_channel(
        &mut self,
        party_a_pubkey: &Ed25519PublicKey,
        party_b_pubkey: &Ed25519PublicKey,
        party_b_dest: &ContractAddress,
        token: &ContractAddress,
        amount: i128,
        salt: u64,
    ) -> u64 {
        assert!(amount > 0, "Deposit must be positive");
        let dispute_period = self.default_dispute_period.get();
        assert!(dispute_period > 0, "Dispute period must be set");

        // Get caller as party_a_dest
        let party_a_dest = sdk::get_caller();

        // Generate deterministic channel_id from inputs
        // This avoids contention when multiple channels are opened in the same block
        let channel_id = {
            let mut hasher_input = Vec::new();
            hasher_input.extend_from_slice(party_a_dest.as_ref());
            hasher_input.extend_from_slice(party_a_pubkey.as_ref());
            hasher_input.extend_from_slice(party_b_pubkey.as_ref());
            hasher_input.extend_from_slice(token.as_ref());
            hasher_input.extend_from_slice(&salt.to_le_bytes());
            let hash = sdk::crypto::sha256(&hasher_input);
            // Take first 8 bytes as u64
            u64::from_le_bytes([
                hash[0], hash[1], hash[2], hash[3], hash[4], hash[5], hash[6], hash[7],
            ])
        };

        let channel = Channel {
            party_a_pubkey: *party_a_pubkey,
            party_a_dest,
            deposit_a: amount,

            party_b_pubkey: *party_b_pubkey,
            party_b_dest: *party_b_dest,
            deposit_b: 0,

            token: *token,
            joined: false,

            dispute_period,

            submitted_balance_a: 0,
            submitted_balance_b: 0,
            submitted_nonce: 0,
            dispute_deadline: 0,
        };

        // Transfer tokens from caller to this contract
        let token_client = channel.token_client();
        let self_address = sdk::get_current_address();
        token_client
            .transfer_from(&party_a_dest, &self_address, amount);

        // Atomically create channel (fails if already exists)
        self.channels.at_mut_exclusive(&channel_id).set(&channel);

        channel_id
    }

    fn join_channel(&mut self, channel_id: u64, amount: i128) {
        assert!(amount > 0, "Deposit must be positive");

        let mut channel = self.channels.at(&channel_id).get();

        assert!(!channel.joined, "Channel already joined");
        assert!(!channel.is_closing(), "Channel is closing");

        // Reject a join whose deposit would make the channel total
        // unrepresentable, before moving tokens — the same aggregate invariant
        // deposit() enforces, so total_deposit() can never overflow later.
        channel
            .deposit_a
            .checked_add(amount)
            .expect("join would overflow channel total");

        // Transfer tokens from caller (party B) to this contract
        let caller = sdk::get_caller();
        let token_client = channel.token_client();
        let self_address = sdk::get_current_address();
        token_client
            .transfer_from(&caller, &self_address, amount);

        channel.deposit_b = amount;
        channel.joined = true;

        // Must be exclusive; cannot allow two joins in the same block, or else funds will be lost
        self.channels.at_mut_exclusive(&channel_id).set(&channel);
    }

    fn deposit(&mut self, channel_id: u64, amount: i128) {
        assert!(amount > 0, "Deposit must be positive");

        let mut channel = self.channels.at(&channel_id).get();

        assert!(channel.joined, "Channel not yet joined");
        assert!(!channel.is_closing(), "Channel is closing");

        // Determine which party is depositing based on caller, and check the
        // channel has headroom before moving any tokens. The release contract
        // profile disables overflow checks, so an unchecked `+=` on a top-up
        // whose sum exceeds i128::MAX would wrap the deposit negative. Checking
        // the aggregate total (not just this party's balance) is required: a
        // per-party top-up can keep that party under the limit while pushing
        // deposit_a + deposit_b past it, which would then panic the checked
        // total_deposit() on every close/dispute and lock the channel. Doing
        // this before transfer_from means a rejected top-up commits no token
        // side effect.
        let caller = sdk::get_caller();
        let is_party_a = if caller == channel.party_a_dest {
            true
        } else if caller == channel.party_b_dest {
            false
        } else {
            panic!("Caller is not a channel party");
        };
        // A representable aggregate total also bounds each non-negative party
        // balance, so this single check makes the stored `+=` below safe.
        channel
            .deposit_a
            .checked_add(channel.deposit_b)
            .and_then(|total| total.checked_add(amount))
            .expect("deposit would overflow channel total");

        // Transfer tokens from caller to this contract
        let token_client = channel.token_client();
        let self_address = sdk::get_current_address();
        token_client
            .transfer_from(&caller, &self_address, amount);

        if is_party_a {
            channel.deposit_a += amount;
        } else {
            channel.deposit_b += amount;
        }

        self.channels.at_mut_exclusive(&channel_id).set(&channel);
    }

    fn close_channel(
        &mut self,
        channel_id: u64,
        balance_a: i128,
        balance_b: i128,
        nonce: u64,
        sig_a: &Ed25519Signature,
        sig_b: &Ed25519Signature,
    ) {
        assert!(nonce > 0, "Nonce must be positive");
        assert!(balance_a >= 0, "Balance A must be non-negative");
        assert!(balance_b >= 0, "Balance B must be non-negative");

        let mut channel = self.channels.at(&channel_id).get();

        assert!(channel.joined, "Channel not yet joined");
        assert!(!channel.is_closing(), "Channel already closing");

        // Verify balances don't exceed total deposits
        let total = channel.total_deposit();
        assert!(
            balance_a + balance_b == total,
            "Balances must equal total deposits"
        );

        // Build and serialize the state for signature verification
        let state = ChannelState {
            channel_id,
            balance_a,
            balance_b,
            nonce,
        };
        let msg = serialize_state(&state);

        // Verify signatures
        assert!(
            verify_signature_ed25519(&channel.party_a_pubkey, sig_a, &msg),
            "Invalid signature from party A"
        );
        assert!(
            verify_signature_ed25519(&channel.party_b_pubkey, sig_b, &msg),
            "Invalid signature from party B"
        );

        // Start the dispute period
        channel.submitted_balance_a = balance_a;
        channel.submitted_balance_b = balance_b;
        channel.submitted_nonce = nonce;
        channel.dispute_deadline = get_block_height() + channel.dispute_period as i64;

        // shared write is ok here: multiple closes in the same block are idempotent
        self.channels.at_mut(&channel_id).set(&channel);
    }

    fn dispute(
        &mut self,
        channel_id: u64,
        balance_a: i128,
        balance_b: i128,
        nonce: u64,
        sig_a: &Ed25519Signature,
        sig_b: &Ed25519Signature,
    ) {
        assert!(balance_a >= 0, "Balance A must be non-negative");
        assert!(balance_b >= 0, "Balance B must be non-negative");

        let mut channel = self.channels.at(&channel_id).get();

        assert!(channel.is_closing(), "Channel is not closing");
        assert!(
            get_block_height() < channel.dispute_deadline,
            "Dispute period has ended"
        );
        assert!(
            nonce > channel.submitted_nonce,
            "Nonce must be higher than current"
        );

        // Verify balances don't exceed total deposits
        let total = channel.total_deposit();
        assert!(
            balance_a + balance_b == total,
            "Balances must equal total deposits"
        );

        // Build and serialize the state for signature verification
        let state = ChannelState {
            channel_id,
            balance_a,
            balance_b,
            nonce,
        };
        let msg = serialize_state(&state);

        // Verify signatures
        assert!(
            verify_signature_ed25519(&channel.party_a_pubkey, sig_a, &msg),
            "Invalid signature from party A"
        );
        assert!(
            verify_signature_ed25519(&channel.party_b_pubkey, sig_b, &msg),
            "Invalid signature from party B"
        );

        // Update to the newer state
        channel.submitted_balance_a = balance_a;
        channel.submitted_balance_b = balance_b;
        channel.submitted_nonce = nonce;
        // We have to bump the dispute deadline; otherwise someone could DOS the dispute by submitting
        // one nonce per block, if the dispute period << number of nonces (for a long lived channel)
        channel.dispute_deadline = get_block_height() + channel.dispute_period as i64;

        // shared write is ok here: multiple disputes (of the same nonce) in the same block are idempotent
        self.channels.at_mut(&channel_id).set(&channel);
    }

    fn finalize(&mut self, channel_id: u64) {
        let channel = self.channels.at(&channel_id).get();

        assert!(channel.is_closing(), "Channel is not closing");
        assert!(
            get_block_height() >= channel.dispute_deadline,
            "Dispute period not over"
        );

        // Transfer funds to each party
        let token_client = channel.token_client();

        if channel.submitted_balance_a > 0 {
            token_client
                .transfer(&channel.party_a_dest, channel.submitted_balance_a);
        }

        if channel.submitted_balance_b > 0 {
            token_client
                .transfer(&channel.party_b_dest, channel.submitted_balance_b);
        }

        // Delete the channel
        self.channels.at_mut_exclusive(&channel_id).delete();
    }

    fn get_dispute_period(&self, channel_id: u64) -> u64 {
        let channel = self.channels.at(&channel_id).get();
        channel.dispute_period
    }
}
