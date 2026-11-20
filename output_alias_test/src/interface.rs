use thruput_sdk::*;

#[contract(client = Client)]
pub trait OutputAliasTest {
    fn test_nnint_exact_overlap();
    fn test_nnint_partial_overlap();
    fn test_nnint_no_overlap();
    fn test_calldata_span_writable_overlap();
    fn test_calldata_no_overlap();
    fn test_nnint_input_alias_output();
}
