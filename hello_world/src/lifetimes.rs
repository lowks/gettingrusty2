#[test]
fn test_lifetime1() {
    assert_eq!("hello string", lifetime1());
    assert_eq!(lifetime2("hello", "world"), "world");
}

fn lifetime1<'a>() -> &'a str {
    let hello_string = "hello string";
    hello_string
}

fn lifetime2<'a>(name1: &'a str, name2: &'a str) -> &'a str {
    if name1.len() > name2.len() {
       return name1
    } else {
       return name2
    };
}