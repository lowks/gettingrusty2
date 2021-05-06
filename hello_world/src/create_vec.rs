use std::collections::VecDeque;

/// # Examples
/// ```
/// use hello_world::add_number_to_element;
/// let d = add_number_to_element(Ok(3));
/// assert_eq!(d, Ok(20));
/// ```

pub enum Multiple {
    Integer(i32),
    Blob(String),
}

impl std::fmt::Display for Multiple {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Multiple::Integer(v) => v.fmt(f),
            Multiple::Blob(v) => v.fmt(f),
        }
    }
}

// #[test]

// fn test_create_vector() {
//     let v = create_vector();
//     let integer = Multiple::Integer(v);
// }

#[test]

pub fn test_add_number_to_element() {
    assert_eq!(add_number_to_element(Ok(3)), Ok(20))
}

#[test]

pub fn test_create_vec_deque() {
    let mut test_deque = create_vec_deque();
    assert_eq!(test_deque.pop_front(), Some(1));
}

#[test]

pub fn test_retain_deque() {
    let mut test_deque = retain_deque(10, 2);
    assert_eq!(test_deque, [2,4,6,8]);
}


pub fn create_vector() -> Vec<Multiple> {
    let v2 = vec![
        Multiple::Integer(100),
        Multiple::Blob("Hello World".to_string()),
    ];
    v2
}

pub fn add_number_to_element(thing: Result<i32, i32>) -> Result<i32, i32> {
    thing
        .and_then(|number| Ok(number + 1))
        .and_then(|number2| Ok(number2 * 5))
}

pub fn create_vec_deque() -> VecDeque<i32> {
    let mut buf = VecDeque::new();
    buf.push_back(1);
    buf.push_back(2);
    buf.push_back(3);
    buf
}

pub fn retain_deque(until: i32, factor_of: i32) -> VecDeque<i32> {
    let mut buf = VecDeque::new();
    buf.extend(1..until);
    buf.retain(|&x| x % factor_of == 0);
    buf
}
