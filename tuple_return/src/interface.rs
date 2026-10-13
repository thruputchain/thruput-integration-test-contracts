use thruput_sdk::*;

#[contract(client = Client)]
pub trait TupleReturn {
    fn call1(n: u32) -> (u32, u32);
    fn call2(n: u32) -> (u32, (u64, u64));

    // BASIC TUPLES
    fn single_element_tuple() -> (u32,);
    fn three_element_tuple() -> (u32, u64, bool);
    fn deeply_nested() -> (u32, (u64, (bool, i32)));

    // TUPLES WITH ARRAYS
    fn tuple_with_fixed_array() -> ([u32; 4], u64);
    fn tuple_with_variable_array() -> (VecM<u32, 100>, u64);
    fn nested_arrays() -> ([u32; 2], [u64; 3]);
    fn array_of_tuples() -> [(u32, u64); 4];

    // NESTED ARRAYS (not in tuples)
    fn nested_array_2d() -> [[u8; 4]; 5];
    fn nested_array_3d() -> [[[u32; 2]; 3]; 4];
    fn deeply_nested_array() -> [[[[[u8; 2]; 2]; 2]; 2]; 2];

    // TUPLES WITH OPTIONAL
    fn tuple_with_option() -> (Option<u32>, u64);
    fn tuple_all_optional() -> (Option<u32>, Option<u64>);
    fn nested_option_tuple() -> (u32, Option<(u64, bool)>);

    // COMPLEX COMBINATIONS
    fn kitchen_sink() -> (u32, Option<[u64; 2]>, (bool, VecM<i32, 50>));
    fn arrays_and_tuples() -> ([[(u32, u64); 2]; 3], bool);
    fn tuple_with_nested_array() -> ([[u32; 2]; 3], u64);

    // LARGE TUPLES (test near limits)
    fn tuple_8_elements() -> (u32, u32, u32, u32, u64, u64, u64, u64);
    fn tuple_16_elements() -> (u32, u32, u32, u32, u32, u32, u32, u32,
                                     u64, u64, u64, u64, u64, u64, u64, u64);

    // EDGE CASES
    fn empty_tuple() -> ();
    fn tuple_with_bytes() -> ([u8; 32], u32);
    fn tuple_with_variable_bytes() -> (VecM<u8, 1000>, u32);
    fn nested_bytes_array() -> [[u8; 32]; 3];

    // MIXED COMPLEXITY
    fn tuple_array_option() -> (Option<[u32; 4]>, [Option<u64>; 2]);
    fn nested_everything() -> ([[(u32, Option<u64>); 2]; 3], Option<(bool, [i32; 4])>);
}
