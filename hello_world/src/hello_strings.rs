#[test]

pub fn test_remove_string() {
    assert_eq!(remove_string("hello the world", "the"), "hello ")
}

#[test]

pub fn test_replace_string() {
    assert_eq!(replace_until("hello the world", "", "world"), "world")
}

#[test]

pub fn test_normal_replace() {
    assert_eq!(
        normal_replace("goodbye world", "goodbye", "hello"),
        "hello world"
    )
}

fn hello(a: &mut String) {
    println!("Inside {}", a);
    a.push_str("I am stupid");
}

fn hello_move() -> String {
    let s = String::from("hello");
    s
}

// fn hello_ref() -> &'static String {
//     let s = String::from("hello");
//     s
// }

pub fn remove_string<'a>(input_string: &'a str, string_to_remove: &'a str) -> String {
    let mut s = String::from(input_string);
    let beta_offset = s.find(string_to_remove).unwrap_or(s.len());
    let t: String = s.drain(..beta_offset).collect();
    t
}

pub fn replace_until<'a>(
    input_string: &'a str,
    string_to_replace: &'a str,
    replace_until: &'a str,
) -> String {
    let mut s = String::from(input_string);
    let beta_offset = s.find(replace_until).unwrap_or(s.len());
    s.replace_range(..beta_offset, string_to_replace);
    s
}

pub fn normal_replace(input: &str, to_replace: &str, replace_with: &str) -> String {
    str::replace(input, to_replace, replace_with)
}
