#[test]

pub fn test_remove_string() {
    assert_eq!(remove_string("hello the world", "the"), "hello ")
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
