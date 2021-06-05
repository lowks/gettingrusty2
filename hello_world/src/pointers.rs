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

#[test]

pub fn test_box() {
    assert_eq!(5, *box_new(5));
}

#[test]

pub fn test_return_pointer() {
    let ptr2 = return_pointer();
    assert!(!ptr2.is_null());
    unsafe{ 
        assert!(*ptr2.add(1) > *ptr2);
    }
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

pub fn box_new(input_number: i32) -> Box<i32> {
    let box_num = Box::new(input_number);
    box_num
}

pub fn return_pointer() -> *const u8 {
    let s: &str = "Follow the rabbit";
    let ptr: *const u8 = s.as_ptr();
    ptr
}