use crate::interface::*;
use sdk::*;

/// Bytes written per storage item, capped so a single call cannot exceed what
/// one transaction is allowed to write. The caller asks for [item_bytes] and
/// gets min(item_bytes, this).
const MAX_ITEM_BYTES: usize = 4096;
static FILL_BYTES: [u8; MAX_ITEM_BYTES] = [0xab; MAX_ITEM_BYTES];

/// Key for the [index]th item of the [nonce]th fill call. Distinct nonces give
/// distinct keys, so callers keep inserting rather than overwriting.
fn fill_key(nonce: u64, index: u32) -> StorageKey {
    let mut tag = [0u8; 12];
    tag[..8].copy_from_slice(&nonce.to_le_bytes());
    tag[8..].copy_from_slice(&index.to_le_bytes());
    (b"fill", &tag).hash().into()
}

pub struct StorageFillerContract;

#[contract]
impl StorageFiller for StorageFillerContract {
    fn fill(nonce: u64, count: u32, item_bytes: u32) {
        let len = core::cmp::min(item_bytes as usize, MAX_ITEM_BYTES);
        for index in 0..count {
            let key = fill_key(nonce, index);
            sdk::sys_result::bytestring::bytestring_set(&key, &FILL_BYTES[..len])
                .unwrap();
        }
    }
}
