use sdk::*;

#[contract(client = "Client")]
pub trait ReadOnlyTest {
    fn read_only_bytestring_set(expect_error: bool);
    fn read_only_log_event(expect_error: bool);
    fn write_storage_and_log_event();
    fn read_only_native_get_balance(target_addr: &ContractAddress);
    fn read_only_native_transfer(receiver_addr: &ContractAddress);
    fn read_only_deploy_output_preserved();
    fn read_only_deploy_inner();
    fn read_only_aliasing_before_read_only_check();
    fn read_only_aliasing_before_read_only_check_inner();
}
