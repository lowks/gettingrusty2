#[test]
fn test_is_palindrome_positive_number() {
    assert_eq!(is_palindrome(11), true);
}

#[test]
fn test_is_palindrome_less_than_ten() {
    assert_eq!(is_palindrome(5), true);
}

#[test]
fn test_is_palindrome_negative_number() {
    assert_eq!(is_palindrome(-4), false);
}
 
pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    } else if x < 10 {
        return true;
    } else {
    x.to_string().chars().collect::<Vec<_>>() == x.to_string().chars().rev().collect::<Vec<_>>()
    }
}