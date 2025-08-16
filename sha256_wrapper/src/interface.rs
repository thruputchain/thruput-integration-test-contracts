use sdk::*;

#[contract(client = Client)]
pub trait Sha256Wrapper {
    fn sha256(&self, data: &[u8]) -> SHA256Hash;
    fn sha256_repeated(&self, n: u32, data: &[u8]);
}
