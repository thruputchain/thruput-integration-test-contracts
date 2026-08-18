use sdk::*;

/// A benchmark contract that exists only to create state.
///
/// The load generator's ERC20 transfers add ~370 bytes of state each, so
/// filling a large cache by transaction traffic takes hours. [fill] writes
/// [count] storage items of [item_bytes] each under keys derived from
/// ([nonce], index), so every call adds fresh state and a run's state growth is
/// set by the workload rather than by the transaction count alone.
///
/// State lands on the shard that owns *this contract's* address, so a benchmark
/// that wants state spread across shards must deploy many instances.
#[contract(client = "Client")]
pub trait StorageFiller {
    fn fill(nonce: u64, count: u32, item_bytes: u32);
}
