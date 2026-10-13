use thruput_sdk::*;

#[contract(client = Client)]
pub trait Sha256Wrapper {
    fn sha256(data: &[u8]) -> SHA256Hash;
    fn sha256_repeated(n: u32, data: &[u8]);
}
