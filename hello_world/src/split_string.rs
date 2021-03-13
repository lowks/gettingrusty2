#[test]

fn test_split() {
    assert_eq!(split_string("Hello World"), vec!["Hello", "World"]);
    assert_eq!(split_string("Hello World Too"), vec!["Hello", "World", "Too"]);
}

pub fn split_string(string1: &str) -> Vec<&str> {
    string1.split(" ").collect()
}