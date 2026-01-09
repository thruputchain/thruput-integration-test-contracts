use sdk::*;
use xdrs::{ReadXDR, WriteXDR};

pub use sdk::contract_result::*;

#[cfg(feature = "spec")]
use xdrs::XDR;

// Payment Channel Contract Interface
//
// This contract implements Lightning-style payment channels. Two parties can:
// 1. Open a channel and deposit funds
// 2. Exchange signed state updates off-chain
// 3. Close the channel by submitting a signed state
// 4. Dispute during a timeout period with a newer signed state
// 5. Finalize to distribute funds after the dispute period
//
// State updates are signed messages using the ChannelState struct.
// Higher nonces take precedence during disputes.

// The state that both parties sign for off-chain updates and on-chain close/dispute.
// This struct is serialized to XDR format before signing.
// TODO: Add derive(xdrs::xdr_repr::XDR) once xdrs properly exports it
#[derive(Clone, Debug, PartialEq, ReadXDR, WriteXDR)]
#[cfg_attr(feature = "spec", derive(XDR))]
pub struct ChannelState {
    pub channel_id: u64,
    pub balance_a: i128,
    pub balance_b: i128,
    pub nonce: u64,
}

#[contract(client = Client)]
pub trait PaymentChannel {
    // Initialize the contract storage with the default dispute period (in blocks) for new channels.
    #[init]
    fn initialize(&mut self, dispute_period: u64) -> ();

    // Open a new channel. The caller deposits `amount` tokens.
    // Returns the channel ID (derived deterministically from params).
    //
    // party_a_pubkey: Public key for signing state updates (caller's key)
    // party_b_pubkey: Public key of the counterparty
    // party_b_dest: Where party B's funds go on close
    // token: The ERC20 token contract address
    // amount: Amount to deposit (must be > 0)
    // salt: Unique salt for channel ID (allows same parties to have multiple channels)
    //
    // Note: party_a_dest defaults to the caller's address.
    // Channel ID = hash(caller, party_a_pubkey, party_b_pubkey, token, salt) truncated to u64
    fn open_channel(
        &mut self,
        party_a_pubkey: &Ed25519PublicKey,
        party_b_pubkey: &Ed25519PublicKey,
        party_b_dest: &ContractAddress,
        token: &ContractAddress,
        amount: i128,
        salt: u64,
    ) -> u64;

    // Party B joins an existing channel by depositing their funds.
    // The caller must be party_b_dest (or a contract calling on their behalf).
    // channel_id: The channel to join
    // amount: Amount to deposit (must be > 0)
    fn join_channel(&mut self, channel_id: u64, amount: i128) -> ();

    // Either party can deposit additional funds into the channel.
    // The deposit goes to whichever party the caller matches.
    // channel_id: The channel to deposit into
    // amount: Amount to deposit (must be > 0)
    fn deposit(&mut self, channel_id: u64, amount: i128) -> ();

    // Initiate channel closure by submitting a signed state.
    // This starts the dispute period. Either party can call this.
    // The submitted state must be signed by both parties.
    // channel_id: The channel to close
    // balance_a: Final balance for party A
    // balance_b: Final balance for party B
    // nonce: State nonce (must be > 0)
    // sig_a: Party A's signature over ChannelState XDR
    // sig_b: Party B's signature over ChannelState XDR
    fn close_channel(
        &mut self,
        channel_id: u64,
        balance_a: i128,
        balance_b: i128,
        nonce: u64,
        sig_a: &Ed25519Signature,
        sig_b: &Ed25519Signature,
    ) -> ();

    // Challenge a pending close with a newer signed state.
    // Can only be called during the dispute period.
    // The new nonce must be higher than the current submitted nonce.
    // channel_id: The channel to dispute
    // balance_a: New final balance for party A
    // balance_b: New final balance for party B
    // nonce: State nonce (must be > current nonce)
    // sig_a: Party A's signature
    // sig_b: Party B's signature
    fn dispute(
        &mut self,
        channel_id: u64,
        balance_a: i128,
        balance_b: i128,
        nonce: u64,
        sig_a: &Ed25519Signature,
        sig_b: &Ed25519Signature,
    ) -> ();

    // Finalize a channel after the dispute period has passed.
    // Distributes funds according to the final submitted state.
    // Anyone can call this once the dispute period is over.
    fn finalize(&mut self, channel_id: u64) -> ();

    // Query the dispute period for a specific channel
    fn get_dispute_period(&self, channel_id: u64) -> u64;
}
