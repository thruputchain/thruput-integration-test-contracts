use sdk::prelude::*;

#[contract(client = Client)]
pub trait InitialTokenClaim {
    fn claim_initial_tokens(
        &mut self,
        hash: &SHA256Hash,
        nonce: &SHA256Hash,
        method: &ContractEntrypointID,
        calldata: &[u8],
    ) -> ();
}
