#[test]
fn test_is_palindrome_positive_number() {
    assert_eq!(remove_duplicates([1,1,2]), 2);
}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 1;
        } else {
            nums.dedup();
            nums.len().to_string().parse::<i32>().unwrap()
        }
}