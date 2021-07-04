macro_rules! hello_name {
    ($name:expr) => { format!("Hello there {}", $name) }
}

macro_rules! capitalize {
    ($name: expr) => {
        // $name.chars().map(|x| x.to_uppercase()).collect::<String>()
        str::to_uppercase($name)
    };
}

#[test]

pub fn test_hello_name() {
    assert_eq!(hello_name!("lowks"), "Hello there lowks");
    assert_eq!(capitalize!("lowks"), "LOWKS")
}

