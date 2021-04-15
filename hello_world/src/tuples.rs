#[test]
fn test_reverse_tuple() {
    assert_eq!(reverse_tuple((5,6)), (6,5));
}

// #[test]
// fn test_double_tuple() {
//     assert_eq!(double_tuple(([(5,6)]), (10,12));
// }

pub fn reverse_tuple((x, y): (i32, i32)) -> (i32, i32) {
    (y, x)
}

// pub fn double_tuple(input_vector: Vec<(i32, i32)>) {
//     let return_tuple = input_vector.iter().flat_map(|(x, y)| (x * 2, y*2)).collect();
//     return_tuple
// }