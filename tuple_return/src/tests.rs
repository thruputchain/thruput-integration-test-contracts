use crate::Client;

#[test]
fn test_tuple_return() {
    sdk::client_test(
        |env| env.register_mock(<crate::TupleReturnContract as crate::TupleReturn>::__mock),
        client_test,
    )
}

fn client_test(client: &mut Client) {
    // Existing basic tests
    assert_eq!(client.call1(0), (1, 2));
    assert_eq!(client.call2(0), (3, (4, 5)));

    // Basic tuples
    assert_eq!(client.single_element_tuple(), (42,));
    assert_eq!(client.three_element_tuple(), (10, 20, true));
    assert_eq!(client.deeply_nested(), (1, (2, (true, -3))));

    // Tuples with arrays
    assert_eq!(client.tuple_with_fixed_array(), ([1, 2, 3, 4], 100));
    let (vec_result, val) = client.tuple_with_variable_array();
    assert_eq!(vec_result.as_slice(), &[10, 20, 30]);
    assert_eq!(val, 200);
    assert_eq!(client.nested_arrays(), ([5, 6], [7, 8, 9]));
    assert_eq!(client.array_of_tuples(), [(1, 10), (2, 20), (3, 30), (4, 40)]);

    // Nested arrays (not in tuples)
    let result_2d = client.nested_array_2d();
    assert_eq!(result_2d[0], [1, 2, 3, 4]);
    assert_eq!(result_2d[4], [17, 18, 19, 20]);

    let result_3d = client.nested_array_3d();
    assert_eq!(result_3d[0][0], [1, 2]);
    assert_eq!(result_3d[3][2], [23, 24]);

    let result_4d = client.deeply_nested_array();
    assert_eq!(result_4d[0][0][0][0], [1, 2]);
    assert_eq!(result_4d[1][1][1][1], [31, 32]);

    // Tuples with optional
    assert_eq!(client.tuple_with_option(), (Some(123), 456));
    assert_eq!(client.tuple_all_optional(), (Some(111), Some(222)));
    assert_eq!(client.nested_option_tuple(), (99, Some((88, false))));

    // Complex combinations
    let (a, b, c) = client.kitchen_sink();
    assert_eq!(a, 1);
    assert_eq!(b, Some([10, 20]));
    assert_eq!(c.0, true);
    assert_eq!(c.1.as_slice(), &[1, 2, 3]);

    let (nested, flag) = client.arrays_and_tuples();
    assert_eq!(nested, [[(1, 2), (3, 4)], [(5, 6), (7, 8)], [(9, 10), (11, 12)]]);
    assert_eq!(flag, true);

    let (nested_arr, val) = client.tuple_with_nested_array();
    assert_eq!(nested_arr, [[1, 2], [3, 4], [5, 6]]);
    assert_eq!(val, 999);

    // Large tuples
    assert_eq!(client.tuple_8_elements(), (1, 2, 3, 4, 5, 6, 7, 8));
    // debug not implemented for 16-element tuple
    // assert_eq!(client.tuple_16_elements()?, (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16));

    // Edge cases
    assert_eq!(client.empty_tuple(), ());
    assert_eq!(client.tuple_with_bytes(), ([0xFF; 32], 999));
    let (bytes, val2) = client.tuple_with_variable_bytes();
    assert_eq!(bytes.as_slice(), &[0xAA, 0xBB, 0xCC, 0xDD]);
    assert_eq!(val2, 777);

    let nested_bytes = client.nested_bytes_array();
    assert_eq!(nested_bytes[0], [0x11; 32]);
    assert_eq!(nested_bytes[1], [0x22; 32]);
    assert_eq!(nested_bytes[2], [0x33; 32]);

    // Mixed complexity
    let (opt_arr, arr_opt) = client.tuple_array_option();
    assert_eq!(opt_arr, Some([1, 2, 3, 4]));
    assert_eq!(arr_opt, [Some(10), Some(20)]);

    let (nested_all, opt_tuple) = client.nested_everything();
    assert_eq!(nested_all[0][0], (1, Some(10)));
    assert_eq!(nested_all[2][1], (6, Some(60)));
    assert_eq!(opt_tuple, Some((true, [-1, -2, -3, -4])));
}
