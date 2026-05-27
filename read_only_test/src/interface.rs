use sdk::*;

#[contract(client = "Client")]
pub trait ReadOnlyTest {
    fn read_only_bytestring_set(&self, expect_error: bool);
    fn read_only_log_event(&self, expect_error: bool);
    fn write_storage_and_log_event(&self);
    fn read_only_native_get_balance(&self, target_addr: &ContractAddress);
    fn read_only_native_transfer(&self, receiver_addr: &ContractAddress);
    fn read_only_deploy_output_preserved(&self);
    fn read_only_deploy_inner(&self);
    fn read_only_aliasing_before_read_only_check(&self);
    fn read_only_aliasing_before_read_only_check_inner(&self);
}
