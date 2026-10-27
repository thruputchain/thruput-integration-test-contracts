use thruput_sdk::*;

#[contract(client = "Client" )]
pub trait NoContractWallet {
    // Implementation:
    // 1. Check that the current frame is the root frame
    // 2. Check that there is one attached auth claim from the root transaction,
    // and that that auth claim is for the (as-yet-unnamed) type of auth claim
    // that refers to the pk that derives the current invocation's address.
    //   - This may not be strictly necessary, assuming correctness of the scheduler.
    // 3. invoke address with method and calldata, using the entirety of the gas limit.
    fn invoke(&self, address: &ContractAddress, method: &[u8], calldata: &[u8]);
}
