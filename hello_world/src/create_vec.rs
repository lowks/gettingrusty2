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
