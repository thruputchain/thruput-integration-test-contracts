use sdk::*;

#[contract(client = Client)]
pub trait OutputAliasTest {
    fn test_nnint_exact_overlap(&self);
    fn test_nnint_partial_overlap(&self);
    fn test_nnint_no_overlap(&self);
    fn test_calldata_span_writable_overlap(&self);
    fn test_calldata_no_overlap(&self);
    fn test_nnint_input_alias_output(&self);
}
