#[test]

fn test_create_series() {
    assert_eq!(create_series(1), vec![1, 2, 3])
}

#[test]

fn test_sum_items_in_vec() {
    assert_eq!(sum_items_in_vec(false, vec![1,2,3,4,5]), 15);
    assert_eq!(sum_items_in_vec(true, vec![1, 2, 3]), 12)
}

#[test]

fn test_double_and_sum() {
    assert_eq!(double_and_sum(vec![1,2,3]), 12);
    assert_eq!(double_and_sum(vec![10, 20]), 60);
    // assert_eq!(sum_items_in_vec(true, vec![1, 2, 3]), 12)
}

#[test]

fn test_all_items_contains_string() {
    assert_eq!(all_items_contains_string(vec!["hello world", "hello"], "hello"), true);
    assert_eq!(all_items_contains_string(vec!["hello world", "hello"], "hllo"), false);
}

pub fn create_series(x: i32) -> Vec<i32> {
    let result = vec![x, x + 1, x + 2];

    result
}

pub fn sum_items_in_vec(double: bool, input_vector: Vec<i32>) -> i32 {
    match double {
        false  => input_vector.iter().fold(0, |total, next| total + next),
        true => input_vector.iter().map(|x| x * 2).fold(0, |total, next| total + next),
    }
}

pub fn double_and_sum(mut input_vector: Vec<i32>) -> i32 {
    // input_vector.iter_mut().for_each(|i| *i *= 2);
    // input_vector.iter_mut().for_each(|i| *i *= 2).iter().sum();
    // input_vector.iter().sum()
    input_vector.iter().fold(0, |total, next| total + 2*next)
}

pub fn all_items_contains_string(input_vector: Vec<&str>, search_string: &str) -> bool {
    input_vector.iter().all(|x| x.contains(search_string))
}
