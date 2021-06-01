#[test]


pub fn test_compare_pointer_and_deref() {
    let a = 5;
    let b = &a;
    assert_eq!(compare_pointer_and_deref(a, b), true);
}

#[test]

pub fn test_const_pointer() {
    assert_eq!(const_pointer("earth"), "earth");
    assert_eq!(const_pointer("Mercury"), "Mercury is not earth");
}

pub fn compare_pointer_and_deref(integer_1: i32, integer_2: &i32) -> bool {
    // if integer_1 == *integer_2 {
    //     return true
    // };
    integer_1 == *integer_2
    // return false
}

pub fn const_pointer(input_string: &'static str) -> String {
    let planet: &str = input_string;
    let planet_ptr: *const &str = &planet as *const &str;
    let is_not: &str = " is not earth";

    unsafe {
        let new_string: &str = &format!("{}{}", *planet_ptr, is_not);
        match *planet_ptr {
            "earth" => input_string.to_owned(),
            _ => new_string.to_owned(),
        }
    }   
}
