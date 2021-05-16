#[test]


pub fn test_compare_pointer_and_deref() {
    let a = 5;
    let b = &a;
    assert_eq!(compare_pointer_and_deref(a, b), true);
}

pub fn compare_pointer_and_deref(integer_1: i32, integer_2: &i32) -> bool {
    if integer_1 == *integer_2 {
        return true
    };
    return false
}