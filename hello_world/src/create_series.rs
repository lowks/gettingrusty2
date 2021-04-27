#[test]

fn test_create_series() {
    assert_eq!(create_series(1), vec![1, 2, 3])
}

#[test]

fn test_sum_items_in_vec() {
    assert_eq!(sum_items_in_vec(vec![1,2,3,4,5]), 15)
}

pub fn create_series(x: i32) -> Vec<i32> {
    let result = vec![x, x + 1, x + 2];

    result
}

pub fn sum_items_in_vec(input_vector: Vec<i32>) -> i32 {
    input_vector.iter().fold(0, |total, next| total + next)
}
