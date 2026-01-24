use sdk::*;

#[contract(client = Client)]
pub trait TupleReturn {
    fn call1(&self, n: u32) -> (u32, u32);
    fn call2(&self, n: u32) -> (u32, (u64, u64));

    // BASIC TUPLES
    fn single_element_tuple(&self) -> (u32,);
    fn three_element_tuple(&self) -> (u32, u64, bool);
    fn deeply_nested(&self) -> (u32, (u64, (bool, i32)));

    // TUPLES WITH ARRAYS
    fn tuple_with_fixed_array(&self) -> ([u32; 4], u64);
    fn tuple_with_variable_array(&self) -> (VecM<u32, 100>, u64);
    fn nested_arrays(&self) -> ([u32; 2], [u64; 3]);
    fn array_of_tuples(&self) -> [(u32, u64); 4];

    // NESTED ARRAYS (not in tuples)
    fn nested_array_2d(&self) -> [[u8; 4]; 5];
    fn nested_array_3d(&self) -> [[[u32; 2]; 3]; 4];
    fn deeply_nested_array(&self) -> [[[[[u8; 2]; 2]; 2]; 2]; 2];

    // TUPLES WITH OPTIONAL
    fn tuple_with_option(&self) -> (Option<u32>, u64);
    fn tuple_all_optional(&self) -> (Option<u32>, Option<u64>);
    fn nested_option_tuple(&self) -> (u32, Option<(u64, bool)>);

    // COMPLEX COMBINATIONS
    fn kitchen_sink(&self) -> (u32, Option<[u64; 2]>, (bool, VecM<i32, 50>));
    fn arrays_and_tuples(&self) -> ([[(u32, u64); 2]; 3], bool);
    fn tuple_with_nested_array(&self) -> ([[u32; 2]; 3], u64);

    // LARGE TUPLES (test near limits)
    fn tuple_8_elements(&self) -> (u32, u32, u32, u32, u64, u64, u64, u64);
    fn tuple_16_elements(&self) -> (u32, u32, u32, u32, u32, u32, u32, u32,
                                     u64, u64, u64, u64, u64, u64, u64, u64);

    // EDGE CASES
    fn empty_tuple(&self) -> ();
    fn tuple_with_bytes(&self) -> ([u8; 32], u32);
    fn tuple_with_variable_bytes(&self) -> (VecM<u8, 1000>, u32);
    fn nested_bytes_array(&self) -> [[u8; 32]; 3];

    // MIXED COMPLEXITY
    fn tuple_array_option(&self) -> (Option<[u32; 4]>, [Option<u64>; 2]);
    fn nested_everything(&self) -> ([[(u32, Option<u64>); 2]; 3], Option<(bool, [i32; 4])>);
}
