//! An implementation of tuple return testing

use super::interface::TupleReturn;
use sdk::*;

#[derive(Default)]
pub struct TupleReturnContract;

// Check to make sure specs are properly generated for tuple return types.
#[contract(exports = "guest_exports", storage = false)]
impl TupleReturn for TupleReturnContract {
    fn call1(&self, _n: u32) -> (u32, u32) {
        (1, 2)
    }

    fn call2(&self, _n: u32) -> (u32, (u64, u64)) {
        (3, (4, 5))
    }

    fn single_element_tuple(&self) -> (u32,) {
        (42,)
    }

    fn three_element_tuple(&self) -> (u32, u64, bool) {
        (10, 20, true)
    }

    fn deeply_nested(&self) -> (u32, (u64, (bool, i32))) {
        (1, (2, (true, -3)))
    }

    fn tuple_with_fixed_array(&self) -> ([u32; 4], u64) {
        ([1, 2, 3, 4], 100)
    }

    fn tuple_with_variable_array(&self) -> (VecM<u32, 100>, u64) {
        (vec![10, 20, 30].try_into().unwrap(), 200)
    }

    fn nested_arrays(&self) -> ([u32; 2], [u64; 3]) {
        ([5, 6], [7, 8, 9])
    }

    fn array_of_tuples(&self) -> [(u32, u64); 4] {
        [(1, 10), (2, 20), (3, 30), (4, 40)]
    }

    fn nested_array_2d(&self) -> [[u8; 4]; 5] {
        [
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
            [17, 18, 19, 20],
        ]
    }

    fn nested_array_3d(&self) -> [[[u32; 2]; 3]; 4] {
        [
            [[1, 2], [3, 4], [5, 6]],
            [[7, 8], [9, 10], [11, 12]],
            [[13, 14], [15, 16], [17, 18]],
            [[19, 20], [21, 22], [23, 24]],
        ]
    }

    fn deeply_nested_array(&self) -> [[[[[u8; 2]; 2]; 2]; 2]; 2] {
        [
            [
                [[[1, 2], [3, 4]], [[5, 6], [7, 8]]],
                [[[9, 10], [11, 12]], [[13, 14], [15, 16]]],
            ],
            [
                [[[17, 18], [19, 20]], [[21, 22], [23, 24]]],
                [[[25, 26], [27, 28]], [[29, 30], [31, 32]]],
            ],
        ]
    }

    fn tuple_with_option(&self) -> (Option<u32>, u64) {
        (Some(123), 456)
    }

    fn tuple_all_optional(&self) -> (Option<u32>, Option<u64>) {
        (Some(111), Some(222))
    }

    fn nested_option_tuple(&self) -> (u32, Option<(u64, bool)>) {
        (99, Some((88, false)))
    }

    fn kitchen_sink(&self) -> (u32, Option<[u64; 2]>, (bool, VecM<i32, 50>)) {
        (1, Some([10, 20]), (true, vec![1, 2, 3].try_into().unwrap()))
    }

    fn arrays_and_tuples(&self) -> ([[(u32, u64); 2]; 3], bool) {
        ([[(1, 2), (3, 4)], [(5, 6), (7, 8)], [(9, 10), (11, 12)]], true)
    }

    fn tuple_with_nested_array(&self) -> ([[u32; 2]; 3], u64) {
        ([[1, 2], [3, 4], [5, 6]], 999)
    }

    fn tuple_8_elements(&self) -> (u32, u32, u32, u32, u64, u64, u64, u64) {
        (1, 2, 3, 4, 5, 6, 7, 8)
    }

    fn tuple_16_elements(&self) -> (u32, u32, u32, u32, u32, u32, u32, u32,
                                     u64, u64, u64, u64, u64, u64, u64, u64) {
        (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16)
    }

    fn empty_tuple(&self) -> () {
        ()
    }

    fn tuple_with_bytes(&self) -> ([u8; 32], u32) {
        ([0xFF; 32], 999)
    }

    fn tuple_with_variable_bytes(&self) -> (VecM<u8, 1000>, u32) {
        (vec![0xAA, 0xBB, 0xCC, 0xDD].try_into().unwrap(), 777)
    }

    fn nested_bytes_array(&self) -> [[u8; 32]; 3] {
        [[0x11; 32], [0x22; 32], [0x33; 32]]
    }

    fn tuple_array_option(&self) -> (Option<[u32; 4]>, [Option<u64>; 2]) {
        (Some([1, 2, 3, 4]), [Some(10), Some(20)])
    }

    fn nested_everything(&self) -> ([[(u32, Option<u64>); 2]; 3], Option<(bool, [i32; 4])>) {
        (
            [
                [(1, Some(10)), (2, Some(20))],
                [(3, Some(30)), (4, Some(40))],
                [(5, Some(50)), (6, Some(60))],
            ],
            Some((true, [-1, -2, -3, -4])),
        )
    }
}
