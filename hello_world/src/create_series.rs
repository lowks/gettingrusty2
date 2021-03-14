#[test]

fn test_create_series() {
  assert_eq!(create_series(1), vec![1, 2, 3])
}

pub fn create_series(x: i32) -> Vec<i32> {
    let result = vec![x, x+1, x+2];

    result
}