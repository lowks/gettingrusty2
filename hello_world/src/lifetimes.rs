#[test]
fn test_lifetimes() {
    assert_eq!("hello string", lifetime1());
    assert_eq!(lifetime2("hello", "world"), "world");
    assert_eq!(lifetime3("hello", "world"), "world");
}

fn lifetime1<'a>() -> &'a str {
    let hello_string = "hello string";
    hello_string
}

fn lifetime2<'a>(name1: &'a str, name2: &'a str) -> &'a str {
    return if name1.len() > name2.len() {
        name1
    } else {
        name2
    };
}

fn lifetime3<'a: 'b, 'b>(name1: &'a str, name2: &'b str) -> &'b str {
    return if name1.len() > name2.len() {
        name1
    } else {
        name2
    };
}