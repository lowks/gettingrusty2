#[test]


pub fn test_compare_pointer_and_deref() {
    let a = 5;
    let b = &a;
    assert_eq!(compare_pointer_and_deref(a, b), true);
}

#[test]

pub fn test_const_pointer() {
    assert_eq!(const_pointer(), "earth");
}

pub fn compare_pointer_and_deref(integer_1: i32, integer_2: &i32) -> bool {
    // if integer_1 == *integer_2 {
    //     return true
    // };
    integer_1 == *integer_2
    // return false
}

pub fn const_pointer() -> &'static str {
    let planet: &str = "earth";
    let planet_ptr: *const &str = &planet as *const &str;
    unsafe {
        match *planet_ptr {
            "earth" => &"earth",
            _ => &"not earth",
        }
    }   
}
