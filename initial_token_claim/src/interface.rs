use sdk::*;

#[contract(client = Client)]
pub trait InitialTokenClaim {
    fn claim_initial_tokens(
        hash: &SHA256Hash,
        nonce: &SHA256Hash,
        method: &ContractEntrypointID,
        calldata: &[u8],
    ) -> ();
}
